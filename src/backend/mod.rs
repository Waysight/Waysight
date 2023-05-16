mod drm;
mod winit;

pub trait Backend {
    fn initialize();
    // TODO: add more methods
}
