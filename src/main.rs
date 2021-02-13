use bmp280::Bmp280Builder;
use simple_error::bail;
use std::error::Error;
use std::num::ParseIntError;
use structopt::StructOpt;

fn parse_hex(src: &str) -> Result<u16, ParseIntError> {
    u16::from_str_radix(src, 16)
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "a", long, parse(try_from_str = parse_hex))]
    i2c_address: u16,
    #[structopt(short = "b", long)]
    i2c_bus_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let mut sensor = match Bmp280Builder::new()
        .address(opt.i2c_address)
        .path(opt.i2c_bus_path)
        .build()
    {
        Ok(ok) => ok,
        Err(error) => bail!("Failed to get sensor: {:?}", error),
    };

    println!("{:?}", sensor.read_temperature());
    Ok(())
}
