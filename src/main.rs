extern crate console;
extern crate rand;

mod display;
mod simulator;

pub struct Settings {
    width: u32,
    height: u32,
    number_of_agents: u32,
    simulation_steps: u32,
}

fn main() {
    simulator::simulate(Settings {
        width: 10,
        height: 10,
        number_of_agents: 10,
        simulation_steps: 10,
    })
}
