use crate::*;

pub fn get_device_languages() -> Result<Vec<rusb::Language>, ReaderError> {
    let device = CFRH320U93::init()?;
    let result = device.handle.read_languages(device.timeout);
    match result {
        Ok(l) => Ok(l),
        Err(e) => Err(e.into())
    }
}

pub fn get_manufacturer() -> Result<String, ReaderError> {
    let lang = get_device_languages()?[0];
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

pub fn get_product_string() -> Result<String, ReaderError> {
    let lang = get_device_languages()?[0];
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

pub fn get_serial_number() -> Result<String, ReaderError> {
    let lang = get_device_languages()?[0];
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
