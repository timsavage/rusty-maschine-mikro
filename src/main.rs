mod controller;
mod maschine_mikro_mk2;

use controller::{Controller, Display};
use hidapi::HidApi;
use maschine_mikro_mk2::MaschineMikroMk2;
use crate::controller::Button;


fn on_button_change(button: controller::Button, pressed: bool, shift: bool) {

}


fn on_encoder_change(encoder: u8, direction: controller::Direction, shift: bool) {

}

fn main() {
    let api = HidApi::new().unwrap();

    let mut ctlr = MaschineMikroMk2::new(
        api.open(MaschineMikroMk2::VENDOR_ID, MaschineMikroMk2::PRODUCT_ID)
            .expect("Cannot open device"),
    );
    ctlr.display.fill(controller::Pixel::Off);
    ctlr.set_button_led(Button::Play, controller::WHITE);

    println!(
        "Device Product: {}",
        ctlr.device
            .get_product_string()
            .unwrap()
            .unwrap_or_default()
    );
    println!(
        "Device Manufacturer: {}",
        ctlr.device
            .get_manufacturer_string()
            .unwrap()
            .unwrap_or_default()
    );
    println!(
        "Device Serial Number: {}",
        ctlr.device
            .get_serial_number_string()
            .unwrap()
            .unwrap_or_default()
    );

    loop {
        ctlr.tick(on_button_change, on_encoder_change).unwrap();
    }
}
