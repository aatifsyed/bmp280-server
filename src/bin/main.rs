use bme280::BME280; // Also BMP280 compatible
use error_chain::error_chain;
use linux_embedded_hal::{Delay, I2cdev};
use std::net::SocketAddr;
use structopt::StructOpt;
use tokio::net::TcpListener;

fn parse_hex(src: &str) -> Result<u8> {
    Ok(u8::from_str_radix(src, 16).chain_err(|| "foo")?)
}

error_chain! {}

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
async fn _main() -> Result<()> {
    let opt = Opt::from_args();
    let i2c_bus = I2cdev::new(opt.i2c_bus_path).chain_err(|| "Couldn't open i2c device")?;
    let mut sensor = BME280::new(i2c_bus, opt.i2c_address, Delay);
    sensor.init().expect("Couldn't initialize device");
    let listener = TcpListener::bind(opt.socket)
        .await
        .chain_err(|| "Couldn't bind to socket")?;

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .chain_err(|| "Couldn't accept connection")?;
    }

    let measurements = sensor
        .measure()
        .chain_err(|| "Couldn't perform measurement");
    println!("{:?}", measurements);
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
