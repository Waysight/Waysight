use smithay::{
    backend::{
        allocator::gbm::GbmDevice,
        drm::{DrmDevice, DrmDeviceFd, DrmError, DrmNode, NodeType},
        egl::{display::EGLDisplay, EGLDevice, Error as EglErr},
        session::{libseat::Error as LibseatErr, Session},
        udev::{all_gpus, primary_gpu},
    },
    reexports::nix::fcntl::OFlag,
    utils::DeviceFd,
};

use crate::backend::drm::DrmBackend;
use crate::state::Waysight;
use smithay_drm_extras::drm_scanner::DrmScanEvent;
use smithay_drm_extras::drm_scanner::{DrmScanResult, DrmScanner};
use std::{os::fd::FromRawFd, path::Path};

#[allow(dead_code)]
pub struct Device {
    drm: DrmDevice,
    gbm: GbmDevice<DrmDeviceFd>,
    device_node: DrmNode,
    render_node: DrmNode,
    scanner: DrmScanner,
}

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum DeviceError {
    #[error("Failed to open device with session: {0}")]
    OpenDevFile(LibseatErr),
    #[error("Unable to add drm device: {0}")]
    NewDevice(DrmError),
    #[error("Unable to add gbm device: {0}")]
    GbmDevice(std::io::Error),
    #[error("Failure to create egl display: {0}")]
    Egl(EglErr),
}

#[allow(unused)]
impl Device {
    pub fn new<S: Session>(node: DrmNode, path: &Path, session: &mut S) -> Result<Self, DeviceError>
    where
        S::Error: Into<LibseatErr>,
    {
        let fd = session
            .open(
                path,
                OFlag::O_RDWR | OFlag::O_CLOEXEC | OFlag::O_NOCTTY | OFlag::O_NONBLOCK,
            )
            .map_err(|err| DeviceError::OpenDevFile(err.into()))?;

        let fd = unsafe { DeviceFd::from_raw_fd(fd) };
        let drm_fd = DrmDeviceFd::new(fd);

        let (drm, notifier) =
            DrmDevice::new(drm_fd, true).map_err(|err| DeviceError::NewDevice(err))?;
        let gbm =
            GbmDevice::new(drm.device_fd().clone()).map_err(|err| DeviceError::GbmDevice(err))?;

        tracing::info!(
            "Successfully found and added device: {}",
            node.dev_path().unwrap().to_str().unwrap()
        );

        let render_node =
            EGLDevice::device_for_display(&EGLDisplay::new(gbm.clone()).map_err(DeviceError::Egl)?)
                .map_err(DeviceError::Egl)
                .ok()
                .and_then(|egl| egl.try_get_render_node().map_err(DeviceError::Egl).ok()?)
                .unwrap_or(node);

        Ok(Device {
            drm,
            gbm,
            device_node: node,
            render_node,
            scanner: DrmScanner::new(),
        })
    }

    pub fn on_device_changed(&mut self, state: &mut Waysight<DrmBackend>) {
        for event in self.scanner.scan_connectors(&self.drm) {
            match event {
                DrmScanEvent::Connected {
                    connector,
                    crtc: Some(crtc),
                } => {
                    tracing::info!("Connector found");
                }
                DrmScanEvent::Disconnected { connector, crtc } => {
                    tracing::info!("Connector disconnected");
                }
                _ => {}
            }
        }
    }
}

pub fn find_primary_gpu<T: AsRef<str> + Clone>(seat: T) -> DrmNode {
    let primary_gpu = primary_gpu(seat.clone())
        .unwrap_or_else(|_| {
            Some(
                all_gpus(seat.clone())
                    .expect("No gpu found")
                    .get(0)
                    .unwrap()
                    .clone(),
            )
        })
        .expect("Path to gpu device not found");
    let node = match DrmNode::from_path(primary_gpu)
        .expect("Node creation error")
        .node_with_type(NodeType::Primary)
    {
        Some(res) => match res {
            Ok(node) => {
                tracing::info!(
                    "Found primary node at: {}",
                    node.dev_path().unwrap().to_str().unwrap()
                );
                node
            }
            Err(err) => {
                tracing::info!("Error finding primary node: {}", err);
                panic!();
            }
        },
        None => {
            tracing::error!("No node found");
            panic!()
        }
    };
    node
}
