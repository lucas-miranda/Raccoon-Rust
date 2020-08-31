use std::{
    borrow::{
        Borrow,
        Cow
    },
    cell::RefCell,
    iter::once
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

use std::time::Instant;

use crate::{
    core::GameLoopInterface,
    math::{
        Triangle
    },
    window::{
        Window
    }
};

static VERTEX_SOURCE: &'static str = include_str!("../../resources/shaders/basic_shader.vert");

static FRAGMENT_SOURCE: &'static str = include_str!("../../resources/shaders/basic_shader.frag");

pub struct HalState<B: Backend> {
    instance: B::Instance,
    device: B::Device,
    surface: ManuallyDrop<B::Surface>,
    adapter: adapter::Adapter<B>,
    format: format::Format,
    queue_group: queue::family::QueueGroup<B>,
    frames_in_flight: usize,
    dimensions: window::Extent2D,
    viewport: pso::Viewport,
    current_frame: u64,

    // descriptors
    descriptor_set_layout: ManuallyDrop<B::DescriptorSetLayout>,
    descriptor_pool: ManuallyDrop<B::DescriptorPool>,
    descriptor_set: B::DescriptorSet,

    // resources
    submission_complete_semaphores: Vec<B::Semaphore>,
    submission_complete_fences: Vec<B::Fence>,
    command_pools: Vec<B::CommandPool>,
    command_buffers: Vec<B::CommandBuffer>,

    // vertex buffer
    vertex_buffer: ManuallyDrop<B::Buffer>,
    vertex_buffer_memory: ManuallyDrop<B::Memory>,

    // pass
    render_pass: ManuallyDrop<B::RenderPass>,

    // pipeline
    graphics_pipeline: ManuallyDrop<B::GraphicsPipeline>,
    pipeline_layout: ManuallyDrop<B::PipelineLayout>,
}

impl<B: Backend> HalState<B> {
    pub fn new<L: 'static + GameLoopInterface>(window: &Window<L>) -> Result<Self, &'static str> {
        let instance = B::Instance::create("App Name", 1)
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
        let bindings = {
            /*
            let bindings = vec![
                pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::Image {
                        ty: pso::ImageDescriptorType::Sampled {
                            with_sampler: false,
                        },
                    },
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
                pso::DescriptorSetLayoutBinding {
                    binding: 1,
                    ty: pso::DescriptorType::Sampler,
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                }
            ];
            */

            Vec::<pso::DescriptorSetLayoutBinding>::new()
        };

        let immutable_samplers = Vec::<B::Sampler>::new();

        let descriptor_set_layout = unsafe {
                device.create_descriptor_set_layout(bindings, immutable_samplers)
        }
        .expect("Can't create descrpitor set layout");

        let desc: Vec<pso::DescriptorRangeDesc> = Vec::new();

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

        // pipeline
        let (pipeline_layout, graphics_pipeline) = Self::create_pipeline(&mut device, extent, &render_pass, &descriptor_set_layout)?;

        // vertex buffer
        let (vertex_buffer, vertex_buffer_memory, vertex_buffer_requirements) = unsafe {
            const F32_XY_RGB_TRIANGLE: u64 = (size_of::<f32>() * (2 + 3) * 3) as u64;
            let mut vertex_buffer = device
                .create_buffer(F32_XY_RGB_TRIANGLE, buffer::Usage::VERTEX)
                .map_err(|_| "Couldn't create a buffer for the vertices")?;

            let vertex_buffer_requirements = device.get_buffer_requirements(&vertex_buffer);
            let memory_type_id = adapter
                .physical_device
                .memory_properties()
                .memory_types
                .iter()
                .enumerate()
                .find(|&(id, memory_type)| {
                    vertex_buffer_requirements.type_mask & (1 << id) != 0
                        && memory_type.properties.contains(memory::Properties::CPU_VISIBLE)
                })
                .map(|(id, _)| MemoryTypeId(id))
                .ok_or("Coudl'nt find a memory type to support the vertex buffer!")?;

            let vertex_buffer_memory = device
                .allocate_memory(memory_type_id, vertex_buffer_requirements.size)
                .map_err(|_| "Couldn't allocate vertex buffer memory")?;

            device
                .bind_buffer_memory(&vertex_buffer_memory, 0, &mut vertex_buffer)
                .map_err(|_| "Couldn't bind the buffer memory!")?;

            (vertex_buffer, vertex_buffer_memory, vertex_buffer_requirements)
        };

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
            device,
            surface: ManuallyDrop::new(surface),
            adapter,
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
            vertex_buffer: ManuallyDrop::new(vertex_buffer),
            vertex_buffer_memory: ManuallyDrop::new(vertex_buffer_memory),

            // pass
            render_pass,

            // pipeline
            pipeline_layout: ManuallyDrop::new(pipeline_layout),
            graphics_pipeline: ManuallyDrop::new(graphics_pipeline),
        })
    }

    #[allow(clippy::type_complexity)]
    fn create_pipeline(device: &mut B::Device, extent: window::Extent2D, render_pass: &B::RenderPass, descriptor_set_layout: &B::DescriptorSetLayout) -> Result<(B::PipelineLayout, B::GraphicsPipeline), &'static str> {
        let mut compiler = shaderc::Compiler::new().ok_or("shaderc not found!")?;
        let vertex_compile_artifact = compiler
            .compile_into_spirv(
                VERTEX_SOURCE,
                shaderc::ShaderKind::Vertex,
                "vertex.vert",
                "main",
                None
            )
            .map_err(|e| {
                eprintln!("{}", e);
                "Couldn't compile vertex shader!"
            })?;

        let fragment_compile_artifact = compiler
            .compile_into_spirv(
                FRAGMENT_SOURCE,
                shaderc::ShaderKind::Fragment,
                "fragment.frag",
                "main",
                None
            )
            .map_err(|e| {
                eprintln!("{}", e);
                "Couldn't compile fragment shader!"
            })?;

        let vertex_shader_module = unsafe {
            device.create_shader_module(vertex_compile_artifact.as_binary())
                  .map_err(|_| "Couldn't make the vertex module")?
        };

        let fragment_shader_module = unsafe {
            device.create_shader_module(fragment_compile_artifact.as_binary())
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
                main_pass: &*render_pass
            };

            let vertex_buffers: Vec<pso::VertexBufferDesc> = vec![
                pso::VertexBufferDesc {
                    binding: 0,
                    stride: (size_of::<f32>() * 5) as pso::ElemStride,
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

                // color attr
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
                    rect: extent.to_extent().rect(),
                    depth: (0.0..1.0)
                }),
                scissor: Some(extent.to_extent().rect()),
                blend_color: None,
                depth_bounds: None
            };

            let push_constants = vec![(pso::ShaderStageFlags::FRAGMENT, 0..1)];
            let layout = unsafe {
                device.create_pipeline_layout(once(descriptor_set_layout), push_constants)
                      .map_err(|_| "Couldn't create a pipeline layout")?
            };

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

        Ok((pipeline_layout, gfx_pipeline))
    }

    pub fn draw_clear_frame(&mut self, color: [f32; 4]) {
        // setup

        let surface_image = unsafe {
            match self.surface.acquire_image(!0) {
                Ok((image, _)) => image,
                Err(e) => panic!(e)
            }
        };

        let framebuffer = unsafe {
            self.device
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

            self.device
                .wait_for_fence(fence, !0)
                .expect("Failed to wait for fence");

            self.device
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

            command_buffer.bind_graphics_pipeline(&self.graphics_pipeline);
            command_buffer.bind_vertex_buffers(
                0,
                once((&*self.vertex_buffer, buffer::SubRange::WHOLE))
            );
            command_buffer.bind_graphics_descriptor_sets(
                &self.pipeline_layout,
                0,
                once(&self.descriptor_set),
                &[]
            );

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

            self.device.destroy_framebuffer(framebuffer);

            /*
            if present_result.is_err() {
                self.recreate_swapchain();
            }
            */
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
}

impl<B: Backend> core::ops::Drop for HalState<B> {
    fn drop(&mut self) {
        /*
        let _ = self.device.wait_idle();

        unsafe {
            for descriptor_set_layout in self.descriptor_set_layouts.drain(..) {
                self.device
                    .destroy_descriptor_set_layout(descriptor_set_layout);
            }

            for fence in self.in_flight_fences.drain(..) {
                self.device.destroy_fence(fence);
            }

            for semaphore in self.render_finished_semaphores.drain(..) {
                self.device.destroy_semaphore(semaphore);
            }

            for semaphore in self.image_available_semaphores.drain(..) {
                self.device.destroy_semaphore(semaphore);
            }

            /*
            for framebuffer in self.framebuffers.drain(..) {
                self.device.destroy_framebuffer(framebuffer);
            }
            */

            /*
            for swapchain_image in self.swapchain_images.drain(..) {
                self.device.destroy_image_view(swapchain_image);
            }
            */

            use core::ptr::read;

            self.device
                .destroy_buffer(ManuallyDrop::into_inner(read(&self.buffer)));

            self.device
                .free_memory(ManuallyDrop::into_inner(read(&self.memory)));

            self.device
                .destroy_pipeline_layout(ManuallyDrop::into_inner(read(&self.pipeline_layout)));

            self.device
                .destroy_graphics_pipeline(ManuallyDrop::into_inner(read(&self.graphics_pipeline)));

            self.device.destroy_command_pool(
                ManuallyDrop::into_inner(read(&self.command_pool))
            );

            self.device
                .destroy_render_pass(ManuallyDrop::into_inner(read(&self.render_pass)));

            /*
            self.device
                .destroy_swapchain(ManuallyDrop::into_inner(read(&self.swapchain)));
            */

            self._surface.unconfigure_swapchain(&self.device);

            ManuallyDrop::drop(&mut self.device);
            ManuallyDrop::drop(&mut self._instance);
        }
        */
    }
}
