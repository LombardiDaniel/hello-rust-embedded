use esp_idf_hal::{
    delay::FreeRtos,
    // gpio::{IOPin, PinDriver},
    gpio::PinDriver,
    peripherals::Peripherals,
};

mod calculator;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let mut led_pin = PinDriver::output(peripherals.pins.gpio2).unwrap();

    let mut i = 1;
    while i < 100 {
        print!("{}\n", i);
        i += 1;
    }

    log::info!("Hello, world!");

    log::info!("{}", calculator::add(5, 7));

    loop {
        led_pin.set_low().unwrap();
        FreeRtos::delay_ms(1000);
        led_pin.set_high().unwrap();
        FreeRtos::delay_ms(1000);
    }
}
