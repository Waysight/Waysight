use smithay::{input::SeatHandler, reexports::wayland_server::protocol::wl_surface::WlSurface};

use crate::state::{Backend, Waysight};

impl<B: Backend + 'static> SeatHandler for Waysight<B> {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;

    fn seat_state(&mut self) -> &mut smithay::input::SeatState<Self> {
        &mut self.seat_state
    }
}
