mod renderer;
pub use renderer::Renderer;

mod backends;
pub use backends::*;

mod rendering_requirements;
pub use rendering_requirements::RenderingRequirements;

// macros
#[macro_export]
macro_rules! verify_backend_requirements {
    ($requirements:expr) => {
        if !Backend::has_requirements($requirements) {
            panic!(
                "For selected backend: {}\nIt doesn't met one or more requirements: {}", 
                Backend::name(),
                $requirements
            );
        }
    };

    ($requirements:expr, $additional_message:literal) => {
        if !Backend::has_requirements($requirements) {
            panic!(
                "{}\nFor selected backend: {}\nIt doesn't met one or more requirements: {}", 
                $additional_message,
                Backend::name(),
                $requirements
            );
        }
    };
}

pub use verify_backend_requirements;

