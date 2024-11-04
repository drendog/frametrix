use super::{
    Command, BAUD_RATE, COMMAND_BUFFER_SIZE, FWK_MAGIC, LED_MATRIX_HEIGHT, LED_MATRIX_WIDTH,
    MATRIX_BUFFER_SIZE, SERIAL_TIMEOUT,
};
use crate::error::LedMatrixError;
use serialport::SerialPort;

pub type PatternId = u8;

#[derive(Debug)]
pub struct LedMatrix {
    port_name: String,
}

impl LedMatrix {
    pub fn new(port_name: String) -> Self {
        Self { port_name }
    }

    pub fn open_serial_connection(&self) -> Result<Box<dyn SerialPort>, LedMatrixError> {
        serialport::new(&self.port_name, BAUD_RATE)
            .timeout(SERIAL_TIMEOUT)
            .open()
            .map_err(|e| match e.kind {
                serialport::ErrorKind::Io(std::io::ErrorKind::PermissionDenied) => {
                    LedMatrixError::PermissionDenied(
                        "Ensure that you have permission, for example using a udev rule or sudo."
                            .to_string(),
                    )
                }
                _ => LedMatrixError::SerialPort(e),
            })
    }

    fn send_command(&self, command: Command, args: &[u8]) -> Result<(), LedMatrixError> {
        let mut port = self.open_serial_connection()?;
        let mut buffer = [0u8; COMMAND_BUFFER_SIZE];

        buffer[..2].copy_from_slice(FWK_MAGIC);
        buffer[2] = command as u8;
        buffer[3..3 + args.len()].copy_from_slice(args);

        port.write_all(&buffer[..3 + args.len()])
            .map_err(|e| LedMatrixError::SerialPort(e.into()))?;

        Ok(())
    }

    // TODO: Find a better way to do this, maybe using bitflags or bitvec
    pub fn render_matrix(
        &self,
        matrix: &[[u8; LED_MATRIX_HEIGHT]; LED_MATRIX_WIDTH],
    ) -> Result<(), LedMatrixError> {
        let mut vals = [0u8; MATRIX_BUFFER_SIZE];

        for (x, column) in matrix.iter().enumerate() {
            for (y, &value) in column.iter().enumerate() {
                let bit_index = x + LED_MATRIX_WIDTH * y;
                if value == 0xFF {
                    vals[bit_index / 8] |= 1 << (bit_index % 8);
                }
            }
        }

        self.send_command(Command::DisplayBwImage, &vals)
    }

    pub fn set_pattern(&self, pattern: PatternId) -> Result<(), LedMatrixError> {
        self.send_command(Command::Pattern, &[pattern])
    }

    pub fn set_brightness(&self, brightness: u8) -> Result<(), LedMatrixError> {
        self.send_command(Command::Brightness, &[brightness])
    }

    pub fn port_name(&self) -> &str {
        &self.port_name
    }
}
