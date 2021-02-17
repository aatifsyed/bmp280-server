use bme280::BME280; // Also BMP280 compatible
use error_chain::error_chain;
use hyper::service::{self, Service};
use hyper::{Body, Request, Response};
use linux_embedded_hal::{Delay, I2cdev};
use serde::Serialize;
use serde_json;
use std::cell::{RefCell, RefMut};
use std::net::SocketAddr;
use structopt::StructOpt;

error_chain! {}

thread_local!(static SENSOR: RefCell<Option<BME280<I2cdev,Delay>>> = RefCell::new(None));

fn parse_hex(src: &str) -> Result<u8> {
    Ok(u8::from_str_radix(src, 16).chain_err(|| "foo")?)
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

async fn measurement(_req: Request<Body>) -> Result<Response<Body>> {
    let ret;
    SENSOR.with(|tld| {
        let sensor: Option<BME280<I2cdev, Delay>> = *tld.try_borrow_mut()?;
        Ok(Response::new("foo".into()))
    });
    ret
}

#[tokio::main]
async fn _main() -> Result<()> {
    let opt = Opt::from_args();
    let i2c_bus = I2cdev::new(opt.i2c_bus_path).chain_err(|| "Couldn't open i2c device")?;
    let mut sensor = BME280::new(i2c_bus, opt.i2c_address, Delay);
    sensor.init().chain_err(|| "Couldn't initialize device")?;

    SENSOR.with(|tld| *tld.borrow_mut() = Some(sensor));

    Ok(())
}

fn main() {
    if let Err(err) = _main() {
        for e in err.iter() {
            eprintln!("{}", e)
        }
        std::process::exit(1);
    }
}
