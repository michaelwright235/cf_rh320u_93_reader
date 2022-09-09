use crate::*;

impl CFRH320U93 {
    /// Reads the languages supported by the device's string descriptors.
    pub fn device_languages(&self) -> Result<Vec<rusb::Language>, ReaderError> {
        let result = self.handle.read_languages(self.timeout);
        match result {
            Ok(l) => Ok(l),
            Err(e) => Err(e.into())
        }
    }

    /// Reads the device's manufacturer string descriptor.
    pub fn manufacturer(&self) -> Result<String, ReaderError> {
        let lang = self.device_languages()?[0];
        let result = self.handle
        .read_manufacturer_string(
            lang, 
            &self.handle.device().device_descriptor()?, 
            self.timeout);
        match result {
            Ok(l) => Ok(l),
            Err(e) => Err(e.into())
        }
    }

    /// Reads the device's product string descriptor.
    pub fn product_string(&self) -> Result<String, ReaderError> {
        let lang = self.device_languages()?[0];
        let result = self.handle
        .read_product_string(
            lang, 
            &self.handle.device().device_descriptor()?, 
            self.timeout);
        match result {
            Ok(l) => Ok(l),
            Err(e) => Err(e.into())
        }
    }

    /// Reads the device's serial number string descriptor.
    /// Don't confuse it with its internal serial number.
    pub fn serial_number(&self) -> Result<String, ReaderError> {
        let lang = self.device_languages()?[0];
        let result = self.handle
        .read_serial_number_string(
            lang, 
            &self.handle.device().device_descriptor()?, 
            self.timeout);
        match result {
            Ok(l) => Ok(l),
            Err(e) => Err(e.into())
        }
    }
}
