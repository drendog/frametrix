use crate::{
    config::Config,
    device::{discover_devices, LedMatrix},
    error::LedMatrixError,
    DeviceInfo, MainWindow,
};
use slint::{ModelRc, SharedString, VecModel, Weak};
use std::rc::Rc;
use tracing::{error, info};

fn for_each_devices<F>(operation_name: &str, mut operation: F)
where
    F: FnMut(&LedMatrix) -> Result<(), LedMatrixError>,
{
    match discover_devices() {
        Ok(devices) => {
            for device in devices {
                if let Err(e) = operation(&device) {
                    error!(
                        "Failed to execute {} on {}: {}",
                        operation_name,
                        device.port_name(),
                        e
                    );
                } else {
                    info!(
                        "{} executed successfully on {}",
                        operation_name,
                        device.port_name()
                    );
                }
            }
        }
        Err(e) => error!("Error discovering devices: {}", e),
    }
}

pub fn refresh_devices(window_weak: &Weak<MainWindow>) {
    info!("Refreshing devices");

    let Some(window) = window_weak.upgrade() else {
        error!("Window reference lost");
        return;
    };

    let devices_model: Rc<VecModel<DeviceInfo>> = Rc::new(VecModel::default());

    if let Ok(devices) = discover_devices() {
        for device in devices {
            devices_model.push(DeviceInfo {
                name: SharedString::from(device.port_name()),
                status: SharedString::from("Connected"),
            });
        }
    }

    window.set_devices(ModelRc::from(devices_model));
}

pub fn send_pattern_to_devices(pattern_idx: i32, config: &Config) {
    info!("Sending pattern {}", pattern_idx);

    let Some(pattern) = config.patterns.values().nth(pattern_idx as usize) else {
        error!("Pattern index {} not found", pattern_idx);
        return;
    };

    for_each_devices("pattern", |device| device.set_pattern(pattern.id));
}

pub fn send_brightness_to_devices(brightness: i32) {
    info!("Sending brightness {}", brightness);
    for_each_devices("brightness", |device| {
        device.set_brightness(brightness as u8)
    });
}
