mod device;
use crate::state::Backend;

pub struct DrmBackend {}

impl Backend for DrmBackend {
    fn initialize() {}
}
