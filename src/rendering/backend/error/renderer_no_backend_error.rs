
#[derive(Debug)]
pub enum RendererNoBackendError {
}

impl Display for RendererNoBackendError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        None
    }
}

impl Error for RendererNoBackendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
