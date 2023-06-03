use smithay::{
    backend::renderer::utils,
    reexports::wayland_server::{protocol::wl_surface::WlSurface, Client},
    wayland::compositor::{self, CompositorHandler, CompositorState},
};

use crate::state::{Backend, ClientState, Waysight};

impl<B: Backend + 'static> CompositorHandler for Waysight<B> {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor
    }

    fn client_compositor_state<'a>(
        &self,
        client: &'a Client,
    ) -> &'a smithay::wayland::compositor::CompositorClientState {
        &client.get_data::<ClientState>().unwrap().compositor_client
    }

    fn commit(&mut self, surface: &WlSurface) {
        utils::on_commit_buffer_handler::<Self>(surface);

        let mut root = surface;
        if let Some(parent) = compositor::get_parent(root) {
            root = &parent;
        }
    }
}
