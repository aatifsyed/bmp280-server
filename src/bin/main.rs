use bme280::BME280; // Also BMP280 compatible
use linux_embedded_hal::{Delay, I2cdev};
use std::error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;
use warp::{self, Filter};

fn parse_hex(src: &str) -> Result<u8, std::num::ParseIntError> {
    Ok(u8::from_str_radix(src, 16)?)
}

#[derive(StructOpt)]
#[structopt(about)]
struct Opt {
    /// Address of bmp280, in base-16
    #[structopt(short = "a", long, parse(try_from_str = parse_hex))]
    i2c_address: u8,

    /// Path of the i2c bus
    #[structopt(short = "b", long)]
    i2c_bus_path: String,

    /// Like "127.0.0.1:80"
    #[structopt(short, long)]
    socket: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let opt = Opt::from_args();
    let i2c_bus = I2cdev::new(opt.i2c_bus_path)?;
    let mut sensor = BME280::new(i2c_bus, opt.i2c_address, Delay);
    sensor
        .init()
        .map_err(|error| format!("Couldn't initialize sensor: {:?}", error))?;

    let sensor = Arc::new(Mutex::new(sensor));

    let route = warp::path("measurement")
        .and(warp::get())
        // Give a copy of the sensor to each handler thread
        .and(warp::any().map(move || sensor.clone()))
        .and_then(handle_measurement);

    warp::serve(route).run(opt.socket).await;

    Ok(())
}

async fn handle_measurement(
    sensor: Arc<Mutex<BME280<I2cdev, Delay>>>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let mut sensor = sensor.lock().unwrap();
    match sensor.measure() {
        Ok(measurement) => Ok(Box::new(warp::reply::json(&measurement))),
        Err(err) => Ok(Box::new(format!("{:#?}", err))),
    }
}
