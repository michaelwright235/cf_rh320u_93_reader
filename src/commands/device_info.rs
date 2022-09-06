use crate::*;

/// Reads the languages supported by the device's string descriptors.
pub fn device_languages() -> Result<Vec<rusb::Language>, ReaderError> {
    let device = CFRH320U93::init()?;
    let result = device.handle.read_languages(device.timeout);
    match result {
        Ok(l) => Ok(l),
        Err(e) => Err(e.into())
    }
}

/// Reads the device's manufacturer string descriptor.
pub fn manufacturer() -> Result<String, ReaderError> {
    let lang = device_languages()?[0];
    let device = CFRH320U93::init()?;
    let result = device.handle
    .read_manufacturer_string(
        lang, 
        &device.handle.device().device_descriptor()?, 
        device.timeout);
    match result {
        Ok(l) => Ok(l),
        Err(e) => Err(e.into())
    }
}

/// Reads the device's product string descriptor.
pub fn product_string() -> Result<String, ReaderError> {
    let lang = device_languages()?[0];
    let device = CFRH320U93::init()?;
    let result = device.handle
    .read_product_string(
        lang, 
        &device.handle.device().device_descriptor()?, 
        device.timeout);
    match result {
        Ok(l) => Ok(l),
        Err(e) => Err(e.into())
    }
}

/// Reads the device's serial number string descriptor.
/// Don't confuse it with its internal serial number.
pub fn serial_number() -> Result<String, ReaderError> {
    let lang = device_languages()?[0];
    let device = CFRH320U93::init()?;
    let result = device.handle
    .read_serial_number_string(
        lang, 
        &device.handle.device().device_descriptor()?, 
        device.timeout);
    match result {
        Ok(l) => Ok(l),
        Err(e) => Err(e.into())
    }
}
