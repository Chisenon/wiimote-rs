use crate::prelude::*;

use crate::input::{AcknowledgeData, InputReport, MemoryData};
use crate::output::{Addressing, OutputReport};

const RETRY_COUNT: usize = 10;
const READ_TIMEOUT: usize = 250;
const WRITE_MEMORY_REPORT_ID: u8 = 0x16;

fn read_setup_report(wiimote: &WiimoteDevice) -> WiimoteResult<Option<InputReport>> {
    match wiimote.read_timeout(READ_TIMEOUT) {
        Ok(report) => Ok(Some(report)),
        Err(WiimoteError::WiimoteDeviceError(
            WiimoteDeviceError::MissingData | WiimoteDeviceError::InvalidData,
        )) => Ok(None),
        Err(error) => Err(error),
    }
}

/// Reads up to 16 bytes from the Wii remote.
/// Discards reports other than the expected data, only use during setup to prevent race-conditions.
pub fn read_16_bytes_sync(
    wiimote: &WiimoteDevice,
    addressing: Addressing,
) -> WiimoteResult<MemoryData> {
    let expected_address = addressing.address as u16;
    let expected_size = addressing.size;
    let memory_read_request = OutputReport::ReadMemory(addressing);
    wiimote.write(&memory_read_request)?;

    for _i in 0..RETRY_COUNT {
        if let Some(InputReport::ReadMemory(memory_data)) = read_setup_report(wiimote)? {
            if memory_data.address_offset() == expected_address
                && (memory_data.size() as u16) >= expected_size
            {
                return Ok(memory_data);
            }
        }
    }
    Err(WiimoteDeviceError::InvalidData.into())
}

/// Reads up to 16 bytes from the Wii remote and checks the resulting report data.
/// Discards reports other than the expected data, only use during setup to prevent race-conditions.
pub fn read_16_bytes_sync_checked(
    wiimote: &WiimoteDevice,
    addressing: Addressing,
) -> WiimoteResult<[u8; 16]> {
    let memory_data = read_16_bytes_sync(wiimote, addressing)?;
    Ok(memory_data.data)
}

/// Writes up to 16 bytes to the Wii remote.
/// Discards reports other than the acknowledge result, only use during setup to prevent race-conditions.
pub fn write_16_bytes_sync(
    wiimote: &WiimoteDevice,
    addressing: Addressing,
    data: &[u8; 16],
) -> WiimoteResult<AcknowledgeData> {
    let memory_write_request = OutputReport::WriteMemory(addressing, *data);
    wiimote.write(&memory_write_request)?;

    for _i in 0..RETRY_COUNT {
        if let Some(InputReport::Acknowledge(acknowledge_data)) = read_setup_report(wiimote)? {
            if acknowledge_data.report_number() == WRITE_MEMORY_REPORT_ID {
                return Ok(acknowledge_data);
            }
        }
    }
    Err(WiimoteDeviceError::InvalidData.into())
}
