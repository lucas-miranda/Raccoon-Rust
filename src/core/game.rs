use crate::{
    core::{
        GameError,
        System,
        scene::{
            SceneDirector
        }
    },
    rendering::{
        Renderer
    },
    tools::{
        log::{
            StdoutListener,
            log_info
        }
    }
};

pub struct Game {
    pub scene_director: SceneDirector,
    pub system: System
}

impl Game {
    pub fn new() -> Result<Self, GameError> {
        /*
        info!("Creating Window...");
        let window = Window::default();

        let (frame_width, frame_height) = window
            .winit_window
            .get_inner_size()
            .map(|logical| logical.into())
            .unwrap_or((0.0, 0.0));
        */

        let mut system = System::new();
        system.logger.register(StdoutListener::new());

        Ok(Game {
            scene_director: SceneDirector::new().unwrap(),
            system
        })
    }

    pub fn start(&mut self) {
        log_info!(self.system.logger, "~ Raccoon Rust ~");
        self.run();
        log_info!(self.system.logger, "Terminating Raccoon Rust...");

        /*
        loop {
            let inputs = UserInput::poll_events_loop(&mut self.window.events_loop);
            if inputs.end_requested {
                break;
            }

            if inputs.new_frame_size.is_some() {
                debug!("Window changed size, restarting Renderer...");
                std::mem::drop(renderer);
                renderer = match Renderer::new(&self.window) {
                    Ok(renderer) => renderer,
                    Err(e) => panic!(e)
                };
            }

            self.update_from_input(inputs);

            if let Err(e) = self.render(&mut renderer) {
                error!("Rendering Error: {:?}", e);
                debug!("Auto-restarting HalState...");
                std::mem::drop(renderer);
                renderer = match Renderer::new(&self.window) {
                    Ok(renderer) => renderer,
                    Err(e) => panic!(e)
                };
            }
        }
        */
    }

    fn run(&mut self) {
        self.system.initialize();
        log_info!(self.system.logger, "Initializing...");

        let renderer = Renderer::new()
                                .unwrap();

        self.scene_director.initialize();

        log_info!(self.system.logger, "Starting...");
        self.system.start();
        while self.system.is_running() {
            self.system.step_timer();
            //println!("dt: {:?}, et: {:?}", delta_time, self.system.get_timer());

            self.update();
            self.render();
        }
    }

    fn update(&mut self) {
        self.scene_director.update(&mut self.system);
    }

    fn render(&self) {
        self.scene_director.render();
    }

    /*
    pub fn render(&mut self, renderer: &mut Renderer) -> Result<(), &'static str> {
        /*
        let r = (self.mouse_x / self.frame_width) as f32;
        let g = (self.mouse_y / self.frame_height) as f32;
        let b = (r + g) * 0.3;
        let a = 1.0;
        self._hal_state.draw_clear_frame([r, g, b, a])
        */

        let x = ((self.mouse_x / self.frame_width) * 2.0) - 1.0;
        let y = ((self.mouse_y / self.frame_height) * 2.0) - 1.0;
        let triangle = Triangle {
            points: [
                [-0.5, 0.5], 
                [-0.5, -0.5], 
                [x as f32, y as f32]
            ]
        };

        renderer.draw_triangle_frame(triangle)
    }

    fn update_from_input(&mut self, input: UserInput) {
        if let Some(frame_size) = input.new_frame_size {
            self.frame_width = frame_size.0;
            self.frame_height = frame_size.1;
        }

        if let Some(position) = input.new_mouse_position {
            self.mouse_x = position.0;
            self.mouse_y = position.1;
        }
    }
    */
}

impl Drop for Game {
    fn drop(&mut self) {
    }
}
