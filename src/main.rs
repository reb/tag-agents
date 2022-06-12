extern crate clap;
extern crate console;
extern crate pyo3;
extern crate rand;

mod display;
mod simulator;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
pub struct Settings {
    /// Width of the tag field
    #[clap(short, long, default_value_t = 10)]
    width: u32,

    /// Height of the tag field
    #[clap(short, long, default_value_t = 10)]
    height: u32,

    /// Number of agents to create
    #[clap(short, long, default_value_t = 10)]
    agents: u32,

    /// Length of the simulation
    #[clap(short, long, default_value_t = 100)]
    length: u32,

    /// Speed of the simulation, measured in steps per second. Uncapped if set to 0.
    #[clap(short, long, default_value_t = 8)]
    speed: u8,
}

fn main() {
    let settings = Settings::parse();
    simulator::simulate(settings);
}
