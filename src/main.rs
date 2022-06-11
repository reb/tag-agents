extern crate console;
extern crate rand;

mod display;
mod simulator;

pub struct Settings {
    width: u32,
    height: u32,
    number_of_agents: u32,
    simulation_steps: u32,
    simulation_speed: u8, // measured in steps per second, if 0 it's unlimited
}

fn main() {
    simulator::simulate(Settings {
        width: 80,
        height: 40,
        number_of_agents: 100,
        simulation_steps: 100,
        simulation_speed: 8,
    })
}
