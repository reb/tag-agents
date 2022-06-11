use super::Settings;
use rand::Rng;

#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Agent {
    position: Position,
    is_it: bool,
}

pub fn simulate(settings: Settings) {
    let agents = create_agents(&settings);
    for agent in agents {
        println!("{:?}", agent);
    }
}

fn create_agents(settings: &Settings) -> Vec<Agent> {
    // create a random number generator to set agent positions
    let mut rng = rand::thread_rng();

    // create a Vector of agents
    let mut agents: Vec<_> = (0..settings.number_of_agents)
        .map(|_| Agent {
            position: Position {
                x: rng.gen_range(0..settings.width),
                y: rng.gen_range(0..settings.height),
            },
            is_it: false,
        })
        .collect();

    // Set the first agent as 'It'
    agents.get_mut(0).unwrap().is_it = true;
    agents
}
