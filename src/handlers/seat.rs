use smithay::{
    delegate_seat,
    input::{pointer::CursorImageStatus, Seat, SeatHandler},
    reexports::wayland_server::protocol::wl_surface::WlSurface,
};

use crate::state::{Backend, Waysight};

impl<B: Backend + 'static> SeatHandler for Waysight<B> {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;

    fn seat_state(&mut self) -> &mut smithay::input::SeatState<Self> {
        &mut self.seat_state
    }

    fn cursor_image(&mut self, _seat: &Seat<Self>, image: CursorImageStatus) {
        *self.cursor_image_status.lock().unwrap() = image;
    }
}

delegate_seat!(@<B: Backend + 'static> Waysight<B>);
