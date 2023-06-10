use crate::state::{Backend, CalloopData, Waysight};
use smithay::reexports::{calloop::EventLoop, wayland_server::Display};

pub struct WinitBackend {}

impl Backend for WinitBackend {
    fn seat_name(&self) -> String {
        todo!()
    }
}
pub fn initialize() {
    let display = Display::<Waysight<WinitBackend>>::new().unwrap();
    let event_loop = EventLoop::<'static, CalloopData<WinitBackend>>::try_new().unwrap();
}
