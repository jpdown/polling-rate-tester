use std::ops::Sub;
use std::ptr::write;
use std::time::Instant;
use hidapi::DeviceInfo;

fn main() {
    let api = hidapi::HidApi::new().unwrap();
    // Print out information about all connected devices

    // NON SPOOFED SDVX CON
    // let devices = api.device_list().find(|dev| {
    //     dev.vendor_id() == 0x2341
    //         && dev.product_id() == 0x8036
    //         && dev.usage_page() == 1
    //         && dev.usage() == 5
    // });

    // SPOOFED SDVX CON
    // let devices = api.device_list().find(|dev| {
    //     dev.vendor_id() == 0x1CCF
    //         && dev.product_id() == 0x101C
    //         && dev.usage_page() == 1
    //         && dev.usage() == 5
    // });

    // CHUNITHM AIRS
    let devices = api.device_list().find(|dev| {
        dev.vendor_id() == 0xa152
            && dev.product_id() == 0xa161
            && dev.usage_page() == 0xFF2B
            && dev.usage() == 0xA2
    });

    let conn = match devices {
        Some(device) => device.open_device(&api).unwrap(),
        None => panic!("device not found"),
    };

    conn.set_blocking_mode(true).unwrap();

    let mut start_of_second = Instant::now();
    let mut updates = 0;
    let mut buf: [u8; 512] = [0; 512];

    loop {
        let data = conn.read(&mut buf);
        updates += 1;
        if start_of_second.elapsed().as_secs() >= 1 {
            println!("{updates} Hz");
            updates = 0;
            start_of_second = Instant::now();
        }
    }
}
