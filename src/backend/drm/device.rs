use smithay::backend::drm::DrmDeviceFd;

struct Device {
    drm_fd: DrmDeviceFd,
}
