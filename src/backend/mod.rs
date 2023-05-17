use std::env;

use self::{drm::DrmBackend, winit::WinitBackend};
mod drm;
mod winit;

pub trait Backend {
    fn initialize();
    // TODO: add more methods
}

pub fn backend_init_from_name(name: &str) {
    match name {
        "drm" => DrmBackend::initialize(),
        "winit" => WinitBackend::initialize(),
        "x11" => {
            todo!()
        }
        _ => {
            tracing::error!("Unknown backend");
        }
    }
}

pub fn backend_autoinit() {
    if env::var("WAYLAND_DISPLAY").is_ok() {
        backend_init_from_name("winit");
    } else if env::var("DISPLAY").is_ok() {
        backend_init_from_name("x11");
    } else {
        backend_init_from_name("drm");
    }
}
