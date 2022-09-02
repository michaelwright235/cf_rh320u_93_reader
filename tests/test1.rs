use std::time::Duration;
use cf_rh320u_93_reader as reader;

#[allow(dead_code)]
fn print_result(status_code: reader::StatusCode) {
    print!("status code = {:?}", status_code);
    let code: u8 = status_code.into();
    println!(" = {:#X}", code);
}

fn useful_bytes(b: &[u8]) -> Vec<u8> {
    let v = b.to_vec();
    let mut l = v.len();
    for i in 0..(v.len()-1) {
        if v[v.len()-1-i] == 0x00 {
            l = v.len()-1-i;
        } else {
            break;
        }
    }
    v[0..l].to_owned()
}

fn print_vec<T: std::fmt::UpperHex>(vec: &Vec<T>) {
    print!("[");
    for byte in vec {
        print!("{byte:#X}, ");
    }
    println!("]");
}

#[test]
fn buzzer() {
    reader::control_buzzer(0x1,0x05).unwrap();
}

#[test]
fn green_led() {
    reader::green_led().unwrap();
}

#[test]
fn change_colors() {
    reader::red_led().unwrap();
    loop {
        reader::green_led().unwrap();
        std::thread::sleep(Duration::from_millis(2000));
        reader::red_led().unwrap();
        std::thread::sleep(Duration::from_millis(2000));
    }
}

#[test]
fn red_led() {
    reader::red_led().unwrap();
}

#[test]
fn get_internal_serial_number() {
    println!("{:#X?}", reader::get_internal_serial_number().unwrap() );
}

#[test]
fn set_internal_serial_number() {
    let ser = [0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF];
    reader::set_internal_serial_number(&ser).unwrap();
}

#[test]
fn get_version_number() {
    let result = reader::get_version_number().unwrap();
    println!("{:#X?}", String::from_utf8(result.to_vec()).unwrap() );
}

#[test]
fn set_speed() {
    reader::set_speed(reader::Speed::S115200).unwrap();
}

#[test]
fn device_info() {
    println!("Manufacturer: {}", reader::get_manufacturer().unwrap());
    println!("Product String: {}", reader::get_product_string().unwrap());
    println!("Serial Number: {}", reader::get_serial_number().unwrap());
}

#[test]
fn iso15693_inventory() {
    let result = reader::iso15693_inventory().unwrap();
    for card in result {
        print_vec(&card.to_vec());
    }
}

#[test]
fn iso15693_read() {
    let result = reader::iso15693_read(
        reader::AccessFlag::WithoutUID,
        0x00,
        0x0a
    ).unwrap();
    print_vec(&result);
}

#[test]
fn iso15693_write() {
    let array =  [0x81, 0x01, 0x01, 0x32,
                            0x39, 0x33, 0x35, 0x30,
                            0x30, 0x30, 0x30, 0x30,
                            0x33, 0x36, 0x34, 0x39,
                            0x00, 0x00, 0x00, 0x87,
                            0x93, 0x52, 0x55, 0x32,
                            0x39, 0x33];
        reader::iso15693_write(
        reader::AccessFlag::WithoutUID,
        0x00,
        &array.to_vec()
    ).unwrap();
}

#[test]
fn test_write() {
    let array =  [0x81, 0x01, 0x01, 0x32,
                            0x39, 0x33, 0x35, 0x30,
                            0x30, 0x30, 0x30, 0x30,
                            0x33, 0x36, 0x34, 0x39,
                            0x00, 0x00, 0x00, 0x87,
                            0x93, 0x52, 0x55, 0x32,
                            0x39, 0x33];
    
    // Card data before erase
    let result = reader::iso15693_read(
        reader::AccessFlag::WithoutUID,
        0x00,
        0x0a
    ).unwrap();

    println!("Card data:");
    print_vec(&useful_bytes(&result.as_slice()));

    // Erase
    let u = useful_bytes(result.as_slice()).len();
    let mut erasor = vec![];
    for _ in 0..u {erasor.push(0x00)}

    reader::iso15693_write(
        reader::AccessFlag::WithoutUID,
        0x00,
        &erasor
    ).unwrap();
    println!("Data is successfully erased:");
    print_vec(&result);

    // Writing
    reader::iso15693_write(
        reader::AccessFlag::WithoutUID,
        0x00,
        &array.to_vec()
    ).unwrap();
    println!("Data is successfully written");

    //Checking
    let result = reader::iso15693_read(
        reader::AccessFlag::WithoutUID,
        0x00,
        0x0a
    ).unwrap();
    println!("Data is successfully read");
    print_vec(&useful_bytes(&result.as_slice()));
}

#[test]
fn erase_card() {
    let result = reader::iso15693_read(
        reader::AccessFlag::WithoutUID,
        0x00,
        0x10
    ).unwrap();
    let u = useful_bytes(result.as_slice()).len();
    let mut erasor = vec![];
    for _ in 0..u {erasor.push(0x00)}

    reader::iso15693_write(
        reader::AccessFlag::WithoutUID,
        0x00,
        &erasor
    ).unwrap();
}


#[test]
fn stay_quiet() {
    let uid = [0x68, 0xE, 0x4E, 0x38, 0x8, 0x1, 0x4, 0xE0];

    reader::iso15693_stay_quiet(reader::AccessFlag::WithoutUID, &uid).unwrap();
}

#[test]
fn select() {
    //let uid = [0x68, 0xE, 0x4E, 0x38, 0x8, 0x1, 0x4, 0xE0];
    let uid2 = [0x3, 0xE8, 0xA4, 0x93, 0x50, 0x1, 0x4, 0xE0];
    reader::iso15693_select(reader::AccessFlag::WithUID, &uid2).unwrap();
}