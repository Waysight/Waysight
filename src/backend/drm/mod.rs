mod device;

use std::cell::RefCell;
use std::collections::HashMap;

use self::device::{find_primary_gpu, Device};
use crate::state::{Backend, CalloopData, Waysight};
use smithay::{
    backend::{
        drm::DrmNode,
        session::{libseat::LibSeatSession, Session},
        udev::UdevBackend,
    },
    reexports::{calloop::EventLoop, wayland_server::Display},
};

pub struct DrmBackend {
    primary_gpu: DrmNode,
    devices: HashMap<DrmNode, Device>,
    session: LibSeatSession,
}

impl Backend for DrmBackend {
    fn seat_name(&self) -> String {
        self.session.seat()
    }
}

pub fn initialize() {
    let mut display: Display<Waysight<DrmBackend>> = Display::new().unwrap();
    let event_loop: EventLoop<CalloopData<DrmBackend>> = EventLoop::try_new().unwrap();

    let (mut session, notifier) = match LibSeatSession::new() {
        Ok((ses, not)) => {
            tracing::info!(
                "Successfully created libseat session with name: {}",
                ses.seat()
            );
            (ses, not)
        }
        Err(err) => {
            tracing::error!("Error creating seat session: {}", err);
            return;
        }
    };

    let backend = UdevBackend::new(session.seat().as_str()).unwrap();
    let primary_gpu = find_primary_gpu(session.seat().as_str());
    let data = DrmBackend {
        primary_gpu,
        devices: HashMap::new(),
        session,
    };

    let mut state = RefCell::new(Waysight::new(&event_loop, &mut display, data));
    for (dev_id, node_path) in backend.device_list() {
        let node = DrmNode::from_path(node_path).unwrap();
        state.borrow_mut().backend_data.devices.insert(
            node,
            Device::new(
                node,
                node_path,
                &mut state.borrow_mut().backend_data.session,
            )
            .expect("Error creating device struct"),
        );
        // Created a let binding because `state.borrow_mut()` creates a temporary value that lives shorter than `device`
        let mut borrowed_state = state.borrow_mut();
        let device = if let Some(device) = borrowed_state.backend_data.devices.get_mut(&node) {
            device
        } else {
            return;
        };
        device.on_device_changed(&mut state.borrow_mut());
    }
}
