use std::{
    borrow::{
        Borrow,
        Cow
    },
    cell::RefCell,
    iter::once,
};

#[cfg(feature = "dx12")]
use gfx_backend_dx12 as back;

#[cfg(feature = "metal")]
use gfx_backend_metal as back;

#[cfg(feature = "vulkan")]
use gfx_backend_vulkan as back;

use arrayvec::ArrayVec;

use core::mem::{
    size_of,
    ManuallyDrop
};

use raw_window_handle::{
    HasRawWindowHandle
};

use gfx_hal::{
    adapter,
    buffer,
    command,
    format,
    image,
    memory,
    pass,
    pool,
    prelude::*,
    pso,
    queue,
    window,
    Backend, 
    Instance,
    MemoryTypeId
};

use std::{
    ptr,
    time::Instant
};

use crate::{
    core::GameLoopInterface,
    graphics::{
        shaders::{
            Shader,
            ShaderBuilder
        },
        Texture
    },
    math::{
        Triangle
    },
    rendering::{
        RendererBackend,
        RendererBackendInterface,
        GraphicsDevice,
        VertexPosition,
        VertexUV
    },
    window::{
        Window
    }
};

use super::{
    DeviceAdapterBackend
};

type InternalBackend = <RendererBackend as RendererBackendInterface>::InternalBackend;

pub struct State {
    instance: <InternalBackend as Backend>::Instance,
    pub graphics_device: GraphicsDevice,
    surface: ManuallyDrop<<InternalBackend as Backend>::Surface>,
    format: format::Format,
    queue_group: queue::family::QueueGroup<InternalBackend>,
    frames_in_flight: usize,
    dimensions: window::Extent2D,
    viewport: pso::Viewport,
    current_frame: u64,

    // descriptors
    descriptor_set_layout: ManuallyDrop<<InternalBackend as Backend>::DescriptorSetLayout>,
    descriptor_pool: ManuallyDrop<<InternalBackend as Backend>::DescriptorPool>,
    descriptor_set: <InternalBackend as Backend>::DescriptorSet,

    // resources
    submission_complete_semaphores: Vec<<InternalBackend as Backend>::Semaphore>,
    submission_complete_fences: Vec<<InternalBackend as Backend>::Fence>,
    command_pools: Vec<<InternalBackend as Backend>::CommandPool>,
    command_buffers: Vec<<InternalBackend as Backend>::CommandBuffer>,

    // vertex buffer
    vertex_buffer: Option<ManuallyDrop<<InternalBackend as Backend>::Buffer>>,
    vertex_buffer_memory: Option<ManuallyDrop<<InternalBackend as Backend>::Memory>>,

    // pass
    render_pass: ManuallyDrop<<InternalBackend as Backend>::RenderPass>,

    // pipeline
    graphics_pipeline: Option<ManuallyDrop<<InternalBackend as Backend>::GraphicsPipeline>>,
    pipeline_layout: Option<ManuallyDrop<<InternalBackend as Backend>::PipelineLayout>>,

    // texture handlers
    loaded_texture_uid: u64
}

impl State {
    pub fn new<L: 'static + GameLoopInterface>(window: &Window<L>) -> Result<Self, &'static str> {
        let instance = <InternalBackend as Backend>::Instance::create("App Name", 1)
                                      .map_err(|_e| "Can't create backend instance")?;

        let mut surface = unsafe {
            instance.create_surface(window)
                    .map_err(|_e| {
                        panic!("error: {}", _e);
                        "Can't create surface using window."
                    })?
        };

        let adapters = instance.enumerate_adapters();

        println!("Available adapters:");
        for adapter in &adapters {
            println!("- {:?}", adapter.info);
        }

        let adapter = adapters.into_iter()
                              .find(|a| {
                                  a.queue_families
                                   .iter()
                                   .any(|qf| qf.queue_type().supports_graphics() && surface.supports_queue_family(qf))
                              })
                              .ok_or("Couldn't find a graphical Adapter!")?;

        let (mut device, queue_group) = {
            let queue_family = adapter
                .queue_families
                .iter()
                .find(|qf| qf.queue_type().supports_graphics() && surface.supports_queue_family(qf))
                .ok_or("Couldn't find a QueueFamily with graphics!")?;

            let adapter::Gpu { device, mut queue_groups } = unsafe {
                //let device_features = adapter.physical_device.features();
                //println!("gpu enable features: {:?}", device_features);

                adapter
                    .physical_device
                    .open(&[(queue_family, &[1.0])], gfx_hal::Features::empty())
                    .map_err(|_| "Couldn't open the PhysicalDevice!")?
            };

            let queue_group = match queue_groups.iter().position(|qg| qg.family == queue_family.id()) {
                Some(index) => queue_groups.remove(index),
                None => panic!("Can't find a valid queue group.")
            };

            if queue_group.queues.len() > 0 {
                Ok(())
            } else {
                Err("The QueueGroup did not have any CommandQueues available!")
            }?;

            (device, queue_group)
        };

        let mut command_pool = unsafe { 
            device.create_command_pool(queue_group.family, pool::CommandPoolCreateFlags::empty()) 
        }
        .expect("Can't create command pool");

        // descriptors
        let bindings = vec![
            pso::DescriptorSetLayoutBinding {
                binding: 0,
                ty: pso::DescriptorType::Image {
                    ty: pso::ImageDescriptorType::Sampled {
                        with_sampler: false,
                    },
                },
                count: 1,
                stage_flags: pso::ShaderStageFlags::FRAGMENT,
                immutable_samplers: false,
            },
            pso::DescriptorSetLayoutBinding {
                binding: 1,
                ty: pso::DescriptorType::Sampler,
                count: 1,
                stage_flags: pso::ShaderStageFlags::FRAGMENT,
                immutable_samplers: false,
            }
        ];

        let immutable_samplers = Vec::<<InternalBackend as Backend>::Sampler>::new();

        let descriptor_set_layout = unsafe {
                device.create_descriptor_set_layout(bindings, immutable_samplers)
        }
        .expect("Can't create descrpitor set layout");

        let desc = [
            pso::DescriptorRangeDesc {
                ty: pso::DescriptorType::Image {
                    ty: pso::ImageDescriptorType::Sampled {
                        with_sampler: false
                    }
                },
                count: 1
            },
            pso::DescriptorRangeDesc {
                ty: pso::DescriptorType::Sampler,
                count: 1
            }
        ];

        let mut descriptor_pool = ManuallyDrop::new(
            unsafe {
                device.create_descriptor_pool(
                    1, // sets
                    &desc,
                    pso::DescriptorPoolCreateFlags::empty()
                )
            }
            .expect("Can't create descriptor pool")
        );

        let descriptor_set = unsafe {
            descriptor_pool.allocate_set(&descriptor_set_layout)
        }
        .unwrap();

        let (extent, format, frames_in_flight) = {
            let capabilities = surface.capabilities(&adapter.physical_device);
            let preferred_formats = surface.supported_formats(&adapter.physical_device);

            println!("capabilities: {:?}", capabilities);
            println!("preferred_formats: {:?}", preferred_formats);

            let format = match preferred_formats {
                Some(formats) => formats.iter()
                                        .find(|format| format.base_format().1 == format::ChannelType::Srgb)
                                        .map(|format| *format)
                                        .unwrap_or(formats[0]),

                None => format::Format::Rgba8Srgb
            };

            let extent = {
                let window_client_area = window.inner_size();

                window::Extent2D {
                    width: capabilities.extents
                                       .end()
                                       .width
                                       .min(window_client_area.width()),

                    height: capabilities.extents
                                        .end()
                                        .height
                                        .min(window_client_area.height())
                }
            };

            let swapchain_config = window::SwapchainConfig::from_caps(&capabilities, format, extent);
            let frames_in_flight = swapchain_config.image_count as usize;

            println!("{:?}", swapchain_config);
            println!("frames in flight: {}", frames_in_flight);

            unsafe { surface.configure_swapchain(&device, swapchain_config) }
                            .expect("Failed to configure swapchain");


            (extent, format, frames_in_flight)
        };

        let render_pass = {
            let attachment = pass::Attachment {
                format: Some(format),
                samples: 1,
                ops: pass::AttachmentOps {
                    load: pass::AttachmentLoadOp::Clear,
                    store: pass::AttachmentStoreOp::Store
                },
                stencil_ops: pass::AttachmentOps::DONT_CARE,
                layouts: image::Layout::Undefined..image::Layout::Present
            };

            let subpass = pass::SubpassDesc {
                colors: &[(0, image::Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[]
            };

            ManuallyDrop::new(
                unsafe {
                    device.create_render_pass(&[attachment], &[subpass], &[])
                }
                .expect("Can't create render pass!")
            )
        };

        // resources
        let mut submission_complete_semaphores = Vec::with_capacity(frames_in_flight);
        let mut submission_complete_fences = Vec::with_capacity(frames_in_flight);
        let mut command_pools = Vec::with_capacity(frames_in_flight);
        let mut command_buffers = Vec::with_capacity(frames_in_flight);

        command_pools.push(command_pool);
        for _ in 1..frames_in_flight {
            unsafe {
                command_pools.push(
                    device.create_command_pool(queue_group.family, pool::CommandPoolCreateFlags::empty())
                          .expect("Can't create command pool")
                );
            }
        }

        for i in 0..frames_in_flight {
            submission_complete_semaphores.push(
                device.create_semaphore()
                      .expect("Could not create semaphore")
            );

            submission_complete_fences.push(
                device.create_fence(true)
                      .expect("Could not create fence")
            );

            command_buffers.push(
                unsafe {
                    command_pools[i].allocate_one(command::Level::Primary)
                }
            );
        }

        // viewport
        let viewport = pso::Viewport {
            rect: pso::Rect {
                x: 0,
                y: 0,
                w: extent.width as _,
                h: extent.height as _,
            },
            depth: 0.0..1.0
        };

        Ok(Self {
            instance,
            graphics_device: GraphicsDevice::new(DeviceAdapterBackend::new(device, adapter)),
            surface: ManuallyDrop::new(surface),
            format,
            queue_group,
            frames_in_flight,
            dimensions: extent,
            viewport,
            current_frame: 0,

            // descriptors
            descriptor_set_layout: ManuallyDrop::new(descriptor_set_layout),
            descriptor_pool,
            descriptor_set,

            // resources
            submission_complete_semaphores,
            submission_complete_fences,
            command_pools,
            command_buffers,

            // vertex buffer
            vertex_buffer: None,
            vertex_buffer_memory: None,

            // pass
            render_pass,

            // pipeline
            pipeline_layout: None,
            graphics_pipeline: None,

            // texture handlers
            loaded_texture_uid: 0
        })
    }

    pub fn draw_clear_frame(&mut self, color: [f32; 4]) {
        // setup
        let device_handle = self.graphics_device.backend().device();

        let surface_image = unsafe {
            match self.surface.acquire_image(!0) {
                Ok((image, _)) => image,
                Err(e) => panic!(e)
            }
        };

        let framebuffer = unsafe {
            device_handle
                .create_framebuffer(
                    &self.render_pass,
                    once(surface_image.borrow()),
                    image::Extent {
                        width: self.dimensions.width as u32,
                        height: self.dimensions.height as u32,
                        depth: 1
                    }
                )
                .expect("Failed to create a framebuffer!")
        };

        let frame_index = self.current_frame as usize % self.frames_in_flight;

        unsafe {
            let fence = &self.submission_complete_fences[frame_index];

            device_handle
                .wait_for_fence(fence, !0)
                .expect("Failed to wait for fence");

            device_handle
                .reset_fence(fence)
                .expect("Failed to reset fence");

            self.command_pools[frame_index].reset(false);
        }

        // rendering

        // record commands
        let command_buffer = &mut self.command_buffers[frame_index];
        unsafe {
            command_buffer.begin_primary(command::CommandBufferFlags::ONE_TIME_SUBMIT);

            command_buffer.set_viewports(0, &[self.viewport.clone()]);
            command_buffer.set_scissors(0, &[self.viewport.rect]);

            match self.graphics_pipeline {
                Some(ref graphics_pipeline) => command_buffer.bind_graphics_pipeline(&*graphics_pipeline),
                None => panic!("Missing graphics pipeline.")
            };

            match &self.vertex_buffer {
                Some(vertex_buffer) => command_buffer.bind_vertex_buffers(
                                               0,
                                               once((**vertex_buffer, buffer::SubRange::WHOLE))
                                           ),
                None => panic!("Missing vertex buffer.")
            }

            match &self.pipeline_layout {
                Some(pipeline_layout) => command_buffer.bind_graphics_descriptor_sets(
                                             &*pipeline_layout,
                                             0,
                                             once(&self.descriptor_set),
                                             &[]
                                         ),
                None => panic!("Missing pipeline layout.")
            }

            let clear_values = [
                command::ClearValue {
                    color: command::ClearColor { 
                        float32: color 
                    }
                }
            ];

            command_buffer.begin_render_pass(
                &self.render_pass,
                &framebuffer,
                self.viewport.rect,
                &clear_values,
                command::SubpassContents::Inline
            );

            //command_buffer.draw(0..6, 0..1);

            command_buffer.end_render_pass();
            command_buffer.finish();
        }

        // submission and present

        let submission = queue::Submission {
            command_buffers: once(&*command_buffer),
            wait_semaphores: None,
            signal_semaphores: once(&self.submission_complete_semaphores[frame_index])
        };

        let command_queue = &mut self.queue_group.queues[0];
        unsafe {
            command_queue.submit(
                submission, 
                Some(&self.submission_complete_fences[frame_index])
            );

            let present_result = command_queue.present(
                &mut self.surface, 
                surface_image, 
                Some(&self.submission_complete_semaphores[frame_index])
            );

            device_handle.destroy_framebuffer(framebuffer);

            /*
            if present_result.is_err() {
                self.recreate_swapchain();
            }
            */
        };

        self.current_frame += 1;
    }

    pub fn draw_texture_with_vertices<V, P, U>(&mut self, vertices: &[V], texture: &mut Texture, shader: &Shader) where
        V: VertexPosition<P> + VertexUV<U>
    {
        // load vertices
        let vertex_buffer_memory_type = self.load_vertices(vertices);

        // prepare texture
        self.load_texture(texture, shader, vertex_buffer_memory_type);

        // prepare pipeline
        self.create_pipeline::<V>(shader)
            .unwrap();

        // fetch surface and framebuffer
        let (surface_image, framebuffer) = self.prepare_surface_and_framebuffer();
        let frame_index = self.current_frame as usize % self.frames_in_flight;

        unsafe {
            self.wait_at_fence(frame_index);
        };

        // record commands
        let device_handle = self.graphics_device.backend().device();
        let command_buffer = &mut self.command_buffers[frame_index];
        unsafe {
            command_buffer.begin_primary(command::CommandBufferFlags::ONE_TIME_SUBMIT);

            command_buffer.set_viewports(0, &[self.viewport.clone()]);
            command_buffer.set_scissors(0, &[self.viewport.rect]);

            match self.graphics_pipeline {
                Some(ref graphics_pipeline) => command_buffer.bind_graphics_pipeline(&*graphics_pipeline),
                None => panic!("Missing graphics pipeline.")
            };


            match self.vertex_buffer {
                Some(vertex_buffer) => command_buffer.bind_vertex_buffers(
                                               0,
                                               once((*vertex_buffer, buffer::SubRange::WHOLE))
                                           ),
                None => panic!("Missing vertex buffer.")
            }

            match self.pipeline_layout {
                Some(ref pipeline_layout) => command_buffer.bind_graphics_descriptor_sets(
                                             &*pipeline_layout,
                                             0,
                                             once(&self.descriptor_set),
                                             &[]
                                         ),
                None => panic!("Missing pipeline layout.")
            }

            let clear_values = [
                command::ClearValue {
                    color: command::ClearColor { 
                        float32: [0.8, 0.8, 0.8, 1.0] 
                    }
                }
            ];

            command_buffer.begin_render_pass(
                &self.render_pass,
                &framebuffer,
                self.viewport.rect,
                &clear_values,
                command::SubpassContents::Inline
            );

            command_buffer.draw(0..6, 0..1);

            command_buffer.end_render_pass();
            command_buffer.finish();
        }

        // submission and present

        let command_queue = &mut self.queue_group.queues[0];
        unsafe {
            command_queue.submit(
                queue::Submission {
                    command_buffers: once(&*command_buffer),
                    wait_semaphores: None,
                    signal_semaphores: once(&self.submission_complete_semaphores[frame_index])
                }, 
                Some(&self.submission_complete_fences[frame_index])
            );

            let present_result = command_queue.present(
                &mut self.surface, 
                surface_image, 
                Some(&self.submission_complete_semaphores[frame_index])
            );

            /*
            if present_result.is_err() {
                self.recreate_swapchain();
            }
            */
        };

        unsafe {
            device_handle.destroy_framebuffer(framebuffer);
        };

        self.current_frame += 1;
    }

    /*
    pub fn draw_triangle_frame(&mut self, triangle: Triangle) -> Result<(), &'static str> {
        let image_available = &self.image_available_semaphores[self.current_frame];
        let render_finished = &self.render_finished_semaphores[self.current_frame];

        self.current_frame = (self.current_frame + 1) % self.frames_in_flight;

        let (i_u32, i_usize) = unsafe {
            let image_index = self
                .swapchain
                .acquire_image(core::u64::MAX)
                .map_err(|_| "Couldn't acquire an image from the swapchain!")?;

            (image_index, image_index as usize)
        };

        let flight_fence = &self.in_flight_fences[i_usize];
        unsafe {
            self.device
                .wait_for_fence(flight_fence, core::u64::MAX)
                .map_err(|_| "Failed to wait on the fence!")?;

            self.device
                .reset_fence(flight_fence)
                .map_err(|_| "Coudl'nt reset the fence")?;
        }

        // write triangle data
        unsafe {
            let mut data_target = self
                .device
                .acquire_mapping_writer(&self.memory, 0..self.requirements.size)
                .map_err(|_| "Failed to acquire a memory writer")?;

            let [[a, b], [c, d], [e, f]] = triangle.points;
            let data = [
                a, b, 1.0, 0.0, 0.0, // red
                c, d, 0.0, 1.0, 0.0, // green
                e, f, 0.0, 0.0, 1.0 // blue
            ];

            data_target[..data.len()].copy_from_slice(&data);

            self.device
                .release_mapping_writer(data_target)
                .map_err(|_| "Couldn't release the mapping writer")?;
        }

        // time data
        let duration = Instant::now().duration_since(self.creation_instant);
        let time_f32 = duration.as_secs() as f32 + duration.subsec_nanos() as f32 * 1e-9;

        // record command
        unsafe {
            let buffer = &mut self.command_buffers[i_usize];
            const TRIANGLE_CLEAR: [ClearValue; 1] = [
                ClearValue {
                    color: ClearColor {
                        float32: [0.1, 0.2, 0.3, 1.0]
                    }
                }
            ];

            buffer.begin(false);
            {
                let mut encoder = buffer.begin_render_pass_inline(
                    &self.render_pass,
                    &self.framebuffers[i_usize],
                    self.render_area,
                    TRIANGLE_CLEAR.iter()
                );

                encoder.bind_graphics_pipeline(&self.graphics_pipeline);
                let buffer_ref: &<back::Backend as Backend>::Buffer = &self.buffer;
                let buffers: ArrayVec<[_; 1]> = [(buffer_ref, 0)].into();
                encoder.bind_vertex_buffers(0, buffers);
                encoder.push_graphics_constants(
                    &self.pipeline_layout,
                    ShaderStageFlags::FRAGMENT,
                    0,
                    &[time_f32.to_bits()]
                );

                encoder.draw(0..3, 0..1);
            }

            buffer.finish();
        }

        // submission and present
        let command_buffers = &self.command_buffers[i_usize..=i_usize];
        let wait_semaphores: ArrayVec<[_; 1]> = [(image_available, PipelineStage::COLOR_ATTACHMENT_OUTPUT)].into();
        let signal_semaphores: ArrayVec<[_; 1]> = [render_finished].into();
        let present_wait_semaphores: ArrayVec<[_; 1]> = [render_finished].into();

        let submission = Submission {
            command_buffers,
            wait_semaphores,
            signal_semaphores
        };

        let command_queue = &mut self.queue_group.queues[0];
        unsafe {
            command_queue.submit(submission, Some(flight_fence));
            self.swapchain
                .present(command_queue, i_u32, present_wait_semaphores)
                .map_err(|_| "Failed to present into the swapchain")
        }
    }
    */

    fn prepare_surface_and_framebuffer(&mut self) -> (<<InternalBackend as Backend>::Surface as window::PresentationSurface<InternalBackend>>::SwapchainImage, <InternalBackend as Backend>::Framebuffer) {
        let surface_image = unsafe {
            match self.surface.acquire_image(!0) {
                Ok((image, _)) => image,
                Err(e) => panic!(e)
            }
        };

        let device = self.graphics_device.backend().device();
        let framebuffer = unsafe {
            device
                .create_framebuffer(
                    &self.render_pass,
                    once(surface_image.borrow()),
                    image::Extent {
                        width: self.dimensions.width as u32,
                        height: self.dimensions.height as u32,
                        depth: 1
                    }
                )
                .expect("Failed to create a framebuffer!")
        };

        (surface_image, framebuffer)
    }

    unsafe fn wait_at_fence(&mut self, frame_index: usize) {
        let device = self.graphics_device.backend().device();
        let fence = &self.submission_complete_fences[frame_index];

        device
            .wait_for_fence(fence, !0)
            .expect("Failed to wait for fence");

        device
            .reset_fence(fence)
            .expect("Failed to reset fence");

        self.command_pools[frame_index].reset(false);
    }

    #[allow(clippy::type_complexity)]
    fn create_pipeline<V>(&mut self, shader: &Shader) -> Result<(), &'static str> {
        /*
        let mut builder = ShaderBuilder::new()?;
        let shader = builder.shader_from_files(
            "../../resources/shaders/basic_shader.vert", 
            "../../resources/shaders/basic_shader.frag"
        )?;
        */

        if let Some(_) = self.graphics_pipeline {
            return Ok(());
        }

        let device = self.graphics_device.backend().device();

        let vertex_shader_module = unsafe {
            device.create_shader_module(shader.vertex_data())
                  .map_err(|_| "Couldn't make the vertex module")?
        };

        let fragment_shader_module = unsafe {
            device.create_shader_module(shader.fragment_data())
                  .map_err(|_| "Couldn't make the fragment module")?
        };

        let (pipeline_layout, gfx_pipeline) = {
            let (vs_entry, fs_entry) = (
                pso::EntryPoint {
                    entry: "main",
                    module: &vertex_shader_module,
                    specialization: pso::Specialization::default()
                },
                pso::EntryPoint {
                    entry: "main",
                    module: &fragment_shader_module,
                    specialization: pso::Specialization::default()
                }
            );

            let subpass = pass::Subpass {
                index: 0,
                main_pass: &*self.render_pass
            };

            let vertex_buffers: Vec<pso::VertexBufferDesc> = vec![
                pso::VertexBufferDesc {
                    binding: 0,
                    stride: size_of::<V>() as pso::ElemStride,
                    rate: pso::VertexInputRate::Vertex
                }
            ];

            let attributes: Vec<pso::AttributeDesc> = vec![
                // position attr
                pso::AttributeDesc {
                    location: 0,
                    binding: 0,
                    element: pso::Element {
                        format: format::Format::Rg32Sfloat,
                        offset: 0
                    }
                },

                // uv attr
                pso::AttributeDesc {
                    location: 1,
                    binding: 0,
                    element: pso::Element {
                        format: format::Format::Rgb32Sfloat,
                        offset: (size_of::<f32>() * 2) as pso::ElemOffset
                    }
                }
            ];

            let input_assembler = pso::InputAssemblerDesc {
                primitive: pso::Primitive::TriangleList,
                with_adjacency: false,
                restart_index: None
            };

            let primitive_assembler = pso::PrimitiveAssemblerDesc::Vertex {
                buffers: &vertex_buffers,
                attributes: &attributes,
                input_assembler,
                vertex: vs_entry,
                tessellation: None,
                geometry: None
            };

            let rasterizer = pso::Rasterizer {
                depth_clamping: false,
                polygon_mode: pso::PolygonMode::Fill,
                cull_face: pso::Face::NONE,
                front_face: pso::FrontFace::Clockwise,
                depth_bias: None,
                conservative: false,
                line_width: pso::State::Static(1f32)
            };

            let depth_stencil = pso::DepthStencilDesc {
                depth: None,
                depth_bounds: false,
                stencil: None
            };

            let blender = {
                let blend_state = pso::BlendState {
                    color: pso::BlendOp::Add {
                        src: pso::Factor::One,
                        dst: pso::Factor::Zero
                    },
                    alpha: pso::BlendOp::Add {
                        src: pso::Factor::One,
                        dst: pso::Factor::Zero
                    }
                };

                pso::BlendDesc {
                    logic_op: Some(pso::LogicOp::Copy),
                    targets: vec![
                        pso::ColorBlendDesc {
                            mask: pso::ColorMask::ALL,
                            blend: Some(blend_state)
                        }
                    ]
                }
            };

            let baked_states = pso::BakedStates {
                viewport: Some(pso::Viewport {
                    rect: self.dimensions.to_extent().rect(),
                    depth: (0.0..1.0)
                }),
                scissor: Some(self.dimensions.to_extent().rect()),
                blend_color: None,
                depth_bounds: None
            };

            let push_constants = vec![(pso::ShaderStageFlags::VERTEX, 0..8)];
            let layout = unsafe {
                device.create_pipeline_layout(once(&*self.descriptor_set_layout), push_constants)
                      .map_err(|_| "Couldn't create a pipeline layout")
            }?;

            let gfx_pipeline = {
                let desc = pso::GraphicsPipelineDesc {
                    primitive_assembler,
                    fragment: Some(fs_entry),
                    rasterizer,
                    blender,
                    depth_stencil,
                    multisampling: None,
                    baked_states,
                    layout: &layout,
                    subpass,
                    flags: pso::PipelineCreationFlags::empty(),
                    parent: pso::BasePipeline::None
                };

                unsafe {
                    device.create_graphics_pipeline(&desc, None)
                          .map_err(|_| "Couldn't create a graphics pipeline!")?
                }
            };

            (layout, gfx_pipeline)
        };

        unsafe {
            device.destroy_shader_module(vertex_shader_module);
            device.destroy_shader_module(fragment_shader_module);
        }

        self.pipeline_layout = Some(ManuallyDrop::new(pipeline_layout));
        self.graphics_pipeline = Some(ManuallyDrop::new(gfx_pipeline));

        Ok(())
    }

    fn load_vertices<V, P, U>(&mut self, vertices: &[V]) -> MemoryTypeId where 
        V: VertexPosition<P> + VertexUV<U>
    {
        let device = self.graphics_device.backend().device();
        device.wait_idle().unwrap();

        unsafe {
            match &mut self.vertex_buffer {
                Some(ref mut vertex_buffer) => {
                    device.destroy_buffer(ManuallyDrop::take(vertex_buffer))
                },
                None => ()
            };

            match &mut self.vertex_buffer_memory {
                Some(ref mut vertex_buffer_memory) => {
                    device.free_memory(ManuallyDrop::take(vertex_buffer_memory))
                },
                None => ()
            }
        }

        let adapter = self.graphics_device.backend().adapter();
        let memory_types = adapter.physical_device
                                  .memory_properties()
                                  .memory_types;

        //println!("Memory types: {:?}", memory_types);

        let limits = adapter.physical_device
                            .limits();

        let non_coherent_alignment = limits.non_coherent_atom_size as u64;

        let buffer_stride = size_of::<V>() as u64;
        //println!("buffer_strid: {}", buffer_stride);
        let buffer_len = vertices.len() as u64 * buffer_stride;
        assert_ne!(buffer_len, 0);
        let padded_buffer_len = ((buffer_len + non_coherent_alignment - 1) / non_coherent_alignment) * non_coherent_alignment;

        let mut vertex_buffer = ManuallyDrop::new(
            unsafe {
                device.create_buffer(padded_buffer_len, buffer::Usage::VERTEX)
                      .unwrap()
            }
        );

        let vertex_buffer_requirements = unsafe {
            device.get_buffer_requirements(&vertex_buffer)
        };

        /*
        let vertex_memory_type_id = memory_types
            .iter()
            .enumerate()
            .position(|(id, memory_type)| {
                vertex_buffer_requirements.type_mask & (1 << id) != 0
                    && memory_type.properties.contains(memory::Properties::CPU_VISIBLE)
            })
            .unwrap()
            .into();
        */
        let vertex_memory_type_id = self.get_memory_type(&vertex_buffer_requirements, memory::Properties::CPU_VISIBLE);

        let buffer_memory = unsafe {
            let mem = device.allocate_memory(vertex_memory_type_id, vertex_buffer_requirements.size)
                            .unwrap();

            device.bind_buffer_memory(&mem, 0, &mut vertex_buffer)
                  .unwrap();

            let mapping = device.map_memory(&mem, memory::Segment::ALL).unwrap();
            ptr::copy_nonoverlapping(vertices.as_ptr() as *const u8, mapping, buffer_len as usize);
            device.flush_mapped_memory_ranges(once((&mem, memory::Segment::ALL)))
                  .unwrap();

            device.unmap_memory(&mem);

            ManuallyDrop::new(mem)
        };

        self.vertex_buffer = Some(vertex_buffer);
        self.vertex_buffer_memory = Some(buffer_memory);

        vertex_memory_type_id
    }

    fn load_texture(&mut self, texture: &mut Texture, shader: &Shader, vertex_buffer_memory_type: MemoryTypeId) {
        if self.loaded_texture_uid == texture.uid() {
            // texture is already loaded
            return;
        }

        let texture_size = *texture.size();
        let texture_row_pitch = texture.bindings.row_pitch();
        let texture_image_stride = texture.bindings.image_stride();
        let texture_upload_buffer = texture.bindings.copy_into_stagging_buffer(vertex_buffer_memory_type, &self.graphics_device);

        let device = self.graphics_device.backend().device();
        let mut image_object = ManuallyDrop::new(
            unsafe {
                device.create_image(
                    gfx_hal::image::Kind::D2(texture_size.width() as gfx_hal::image::Size, texture_size.height() as gfx_hal::image::Size, 1, 1),
                    1,
                    gfx_hal::format::Format::Rgba8Srgb,
                    gfx_hal::image::Tiling::Optimal,
                    gfx_hal::image::Usage::TRANSFER_DST | gfx_hal::image::Usage::SAMPLED,
                    gfx_hal::image::ViewCapabilities::empty()
                )
            }
            .unwrap()
        );

        let image_object_requirements = unsafe {
            device.get_image_requirements(&image_object)
        };

        let image_object_memory_type = self.get_memory_type(&image_object_requirements, memory::Properties::DEVICE_LOCAL);

        let image_object_memory = ManuallyDrop::new(
            unsafe {
                device.allocate_memory(image_object_memory_type, image_object_requirements.size)
            }
            .unwrap()
        );

        unsafe {
            device.bind_image_memory(&image_object_memory, 0, &mut image_object)
        }
        .unwrap();

        let image_view = ManuallyDrop::new(
            unsafe {
                device.create_image_view(
                    &image_object,
                    gfx_hal::image::ViewKind::D2,
                    gfx_hal::format::Format::Rgba8Srgb,
                    gfx_hal::format::Swizzle::NO,
                    gfx_hal::image::SubresourceRange {
                        aspects: gfx_hal::format::Aspects::COLOR,
                        ..Default::default()
                    }
                )
            }
            .unwrap()
        );

        unsafe {
            device.write_descriptor_sets(
                vec![
                    pso::DescriptorSetWrite {
                        set: &self.descriptor_set,
                        binding: 0,
                        array_offset: 0,
                        descriptors: Some(
                            pso::Descriptor::Image(
                                &*image_view,
                                image::Layout::ShaderReadOnlyOptimal
                            )
                        )
                    },
                    pso::DescriptorSetWrite {
                        set: &self.descriptor_set,
                        binding: 1,
                        array_offset: 0,
                        descriptors: Some(
                            pso::Descriptor::Sampler(shader.bindings.sampler())
                        )
                    }
                ]
            );
        };

        let mut copy_fence = device.create_fence(false)
                                   .expect("Could not create fence");

        unsafe {
            let mut command_buffer = self.command_pools[0].allocate_one(command::Level::Primary);
            command_buffer.begin_primary(command::CommandBufferFlags::ONE_TIME_SUBMIT);

            let image_barrier = memory::Barrier::Image {
                states: (image::Access::empty(), image::Layout::Undefined)
                        ..(image::Access::TRANSFER_WRITE, image::Layout::TransferDstOptimal),
                target: &*image_object,
                families: None,
                range: image::SubresourceRange {
                    aspects: format::Aspects::COLOR,
                    ..Default::default()
                }
            };

            command_buffer.pipeline_barrier(
                pso::PipelineStage::TOP_OF_PIPE..pso::PipelineStage::TRANSFER,
                memory::Dependencies::empty(),
                &[image_barrier]
            );

            command_buffer.copy_buffer_to_image(
                texture_upload_buffer,
                &image_object,
                image::Layout::TransferDstOptimal,
                &[
                    command::BufferImageCopy {
                        buffer_offset: 0,
                        buffer_width: texture_row_pitch / (texture_image_stride as u32),
                        buffer_height: texture_size.height(),
                        image_layers: image::SubresourceLayers {
                            aspects: format::Aspects::COLOR,
                            level: 0,
                            layers: 0..1
                        },
                        image_offset: image::Offset { x: 0, y: 0, z: 0 },
                        image_extent: image::Extent {
                            width: texture_size.width(),
                            height: texture_size.height(),
                            depth: 1
                        }
                    }
                ]
            );

            let image_barrier = memory::Barrier::Image {
                states: (image::Access::TRANSFER_WRITE, image::Layout::TransferDstOptimal)
                        ..(image::Access::SHADER_READ, image::Layout::ShaderReadOnlyOptimal),
                target: &*image_object,
                families: None,
                range: image::SubresourceRange {
                    aspects: format::Aspects::COLOR,
                    ..Default::default()
                }
            };

            command_buffer.pipeline_barrier(
                pso::PipelineStage::TRANSFER..pso::PipelineStage::FRAGMENT_SHADER,
                memory::Dependencies::empty(),
                &[image_barrier]
            );

            command_buffer.finish();

            self.queue_group.queues[0]
                            .submit_without_semaphores(Some(&command_buffer), Some(&mut copy_fence));

            device.wait_for_fence(&copy_fence, !0)
                  .expect("Can't wait for fence.");

            device.destroy_fence(copy_fence);
        }

        self.loaded_texture_uid = texture.uid();
    }

    fn get_memory_type(&self, requirements: &memory::Requirements, properties: memory::Properties) -> MemoryTypeId {
        self.graphics_device
            .backend()
            .adapter()
            .physical_device
            .memory_properties()
            .memory_types
            .iter()
            .enumerate()
            .position(|(id, memory_type)| {
                requirements.type_mask & (1 << id) != 0
                    && memory_type.properties.contains(properties)
            })
            .unwrap()
            .into()
    }
}

impl core::ops::Drop for State {
    fn drop(&mut self) {
        let device = self.graphics_device.backend().device();
        device.wait_idle().unwrap();

        unsafe {
            device.destroy_descriptor_pool(ManuallyDrop::take(&mut self.descriptor_pool));
            device.destroy_descriptor_set_layout(ManuallyDrop::take(&mut self.descriptor_set_layout));

            // buffers
            match &mut self.vertex_buffer {
                Some(ref mut vertex_buffer) => {
                    device.destroy_buffer(ManuallyDrop::take(vertex_buffer));
                },
                None => ()
            };

            for command_pool in self.command_pools.drain(..) {
                device.destroy_command_pool(command_pool);
            }

            for semaphore in self.submission_complete_semaphores.drain(..) {
                device.destroy_semaphore(semaphore);
            }

            for fence in self.submission_complete_fences.drain(..) {
                device.destroy_fence(fence);
            }

            device.destroy_render_pass(ManuallyDrop::take(&mut self.render_pass));
            self.surface.unconfigure_swapchain(device);

            // memory
            match &mut self.vertex_buffer_memory {
                Some(ref mut vertex_buffer_memory) => {
                    device.free_memory(ManuallyDrop::take(vertex_buffer_memory));
                },
                None => ()
            }

            match &mut self.graphics_pipeline {
                Some(ref mut graphics_pipeline) => {
                    device.destroy_graphics_pipeline(ManuallyDrop::take(graphics_pipeline));
                },
                None => ()
            }

            match &mut self.pipeline_layout {
                Some(ref mut pipeline_layout) => {
                    device.destroy_pipeline_layout(ManuallyDrop::take(pipeline_layout));
                },
                None => ()
            }

            self.instance.destroy_surface(ManuallyDrop::take(&mut self.surface));
        }
    }
}
