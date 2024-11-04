use crate::error::LedMatrixError;
use led_matrix::LedMatrix;
use serialport::SerialPortType;
use std::time::Duration;

const FWK_MAGIC: &[u8] = &[0x32, 0xAC];
const FRAMEWORK_VID: u16 = 0x32AC;
const LED_MATRIX_PID: u16 = 0x0020;
const SERIAL_TIMEOUT: Duration = Duration::from_millis(20);
const BAUD_RATE: u32 = 115_200;
pub const LED_MATRIX_WIDTH: usize = 9;
pub const LED_MATRIX_HEIGHT: usize = 34;
pub const COMMAND_BUFFER_SIZE: usize = 64;
pub const MATRIX_BUFFER_SIZE: usize = 39;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Command {
    Brightness = 0x00,
    Pattern = 0x01,
    DisplayBwImage = 0x06,
}

pub mod led_matrix;
pub mod matrix_ops;
pub mod util;

pub fn discover_devices() -> Result<Vec<LedMatrix>, LedMatrixError> {
    let ports = serialport::available_ports().map_err(LedMatrixError::SerialPort)?;

    let devices = ports
        .iter()
        .filter_map(|port| match &port.port_type {
            SerialPortType::UsbPort(info) if is_led_matrix_device(info) => {
                Some(LedMatrix::new(port.port_name.clone()))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    if devices.is_empty() {
        Err(LedMatrixError::NoDevicesFound)
    } else {
        Ok(devices)
    }
}

fn is_led_matrix_device(info: &serialport::UsbPortInfo) -> bool {
    info.vid == FRAMEWORK_VID && info.pid == LED_MATRIX_PID
}
