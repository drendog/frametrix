use thiserror::Error;

#[derive(Error, Debug)]
pub enum LedMatrixError {
    #[error("Serial port error: {0}")]
    SerialPort(#[from] serialport::Error),

    #[error("No LED Matrix devices found")]
    NoDevicesFound,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}
