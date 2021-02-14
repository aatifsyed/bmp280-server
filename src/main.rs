use bme280::BME280; // Also BMP280 compatible
use linux_embedded_hal::{Delay, I2cdev};
use std::error::Error;
use std::num::ParseIntError;
use structopt::StructOpt;

fn parse_hex(src: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(src, 16)
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "a", long, parse(try_from_str = parse_hex))]
    /// Address of bmp280, in base-16
    i2c_address: u8,
    #[structopt(short = "b", long)]
    /// Path of the i2c bus
    i2c_bus_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let i2c_bus = I2cdev::new(opt.i2c_bus_path)?;
    let mut sensor = BME280::new(i2c_bus, opt.i2c_address, Delay);
    sensor.init().expect("Failed to init");
    let measurements = sensor.measure().expect("Measurement error");
    println!("{:?}", measurements);
    Ok(())
}
