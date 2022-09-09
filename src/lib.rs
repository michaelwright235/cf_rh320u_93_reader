mod buffer;
mod commands;
mod status_code;

pub use {status_code::*, commands::*};
use buffer::*;
use rusb::{Device, DeviceHandle, Context, UsbContext};
use std::time::Duration;

const VID: u16 = 0xffff;
const PID: u16 = 0x0035;
const TIMEOUT: Duration = Duration::from_millis(500);

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
}

pub struct CFRH320U93 {
    handle: DeviceHandle<Context>,
    timeout: Duration
}

impl CFRH320U93 {
    pub fn open() -> Result<Self, ReaderError> {
        let mut context = Context::new().unwrap();
        let (mut device, mut handle) = Self::open_device(&mut context, VID, PID)?;
        let mut endpoints = Self::find_readable_endpoints(&mut device)?;
        
        if endpoints.len() == 0 {
            return Err(rusb::Error::NoDevice.into());
        }
        let mut endpoint = endpoints.remove(0);

        // Windows requires interface 1 for this type of HID device
        if cfg!(windows) {
            endpoint.iface = 1;
        }
        match handle.kernel_driver_active(endpoint.iface) {
            Ok(true) => {
                handle.detach_kernel_driver(endpoint.iface)?;
            }
            _ => (),
        };
    
        // claim and configure device
        Self::configure_endpoint(&mut handle, &endpoint)?;

        Ok(Self {handle, timeout: TIMEOUT} )
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    fn open_device<T: UsbContext>(context: &mut T, vid: u16, pid: u16) -> Result<(Device<T>, DeviceHandle<T>), rusb::Error> {
        let devices = context.devices()?;
    
        for device in devices.iter() {
            let device_desc = match device.device_descriptor() {
                Ok(d) => d,
                Err(_) => continue,
            };
    
            if device_desc.vendor_id() == vid && device_desc.product_id() == pid{
                match device.open() {
                    Ok(handle) => return Ok((device, handle)),
                    Err(_) => continue,
                }
            }
        }
        Err(rusb::Error::NoDevice)
    }

    // returns all readable endpoints for given usb device and descriptor
    fn find_readable_endpoints<T: UsbContext>(device: &mut Device<T>) -> Result<Vec<Endpoint>, rusb::Error> {
        let device_desc = device.device_descriptor()?;
        let mut endpoints = vec![];
        for n in 0..device_desc.num_configurations() {
            let config_desc = match device.config_descriptor(n) {
                Ok(c) => c,
                Err(_) => continue,
            };
            for interface in config_desc.interfaces() {
                for interface_desc in interface.descriptors() {
                    for _ in interface_desc.endpoint_descriptors() {
                        endpoints.push(Endpoint {
                            config: config_desc.number(),
                            iface: interface_desc.interface_number(),
                            setting: interface_desc.setting_number(),
                        });
                    }
                }
            }
        }

        Ok(endpoints)
    }

    fn configure_endpoint<T: UsbContext>(handle: &mut DeviceHandle<T>, endpoint: &Endpoint,) -> Result<(), rusb::Error> {
        handle.set_active_configuration(endpoint.config)?;
        handle.claim_interface(endpoint.iface)?;
        handle.set_alternate_setting(endpoint.iface, endpoint.setting)?;
        Ok(())
    }

    fn set_report(&self, buf: &[u8]) -> Result<(), rusb::Error> {
        self.handle.write_control(0x21, 0x09, 0x0301, 1, buf, self.timeout)?;
        Ok(())
    }

    fn get_report(&self) -> Result<Vec<u8>, rusb::Error> {
        // todo: check checksum and if data is valid altogether
        let mut buf: [u8; 256] = [0; 256];
        self.handle.read_control(0xa1, 0x01, 0x0302, 1, &mut buf, self.timeout)?;

        // getting rid of empty bytes at the end of the response
        let mut new_len = buf.len();
        for i in 0..(buf.len()-1) {
            if buf[buf.len()-1-i] == 0x00 {
                new_len = buf.len()-1-i;
            } else {
                break;
            }
        }
        new_len -= 2; // getting rid of the last two bytes which are the checksum and the end byte
        Ok(buf[0..new_len].to_vec())
    }
}
