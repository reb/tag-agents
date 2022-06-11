use super::Settings;
use rand::distributions::{Distribution, Standard};
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
    x_bound: u32, // upper bound, lower bound is 0
    y_bound: u32, // upper bound, lower bound is 0
}

impl Agent {
    fn decide_action(&self) -> Action {
        // return a random move
        rand::random()
    }

    fn execute_action(mut self, action: Action) -> Self {
        let mut x = self.position.x;
        let mut y = self.position.y;
        // use checked_sub to make sure x and y always stays above 0
        match action {
            Action::Stay => {}
            Action::Up => y = y.checked_sub(1).unwrap_or_default(),
            Action::Right => x += 1,
            Action::Down => y += 1,
            Action::Left => x = x.checked_sub(1).unwrap_or_default(),
        }
        // check the upper bound
        if x >= self.x_bound {
            x = self.x_bound - 1;
        }
        if y >= self.y_bound {
            y = self.y_bound - 1;
        }
        self.position = Position { x, y };
        self
    }
}

#[derive(Debug)]
enum Action {
    Stay,
    Up,
    Right,
    Down,
    Left,
}

impl Distribution<Action> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action {
        match rng.gen_range(0..=4) {
            0 => Action::Stay,
            1 => Action::Up,
            2 => Action::Right,
            3 => Action::Down,
            _ => Action::Left,
        }
    }
}

pub fn simulate(settings: Settings) {
    let mut agents = create_agents(&settings);

    for _ in 0..settings.simulation_steps {
        for agent in &agents {
            println!("{:?}", agent);
        }
        // all agents take an action
        agents = agents
            .into_iter()
            .map(|agent| {
                let action = agent.decide_action();
                agent.execute_action(action)
            })
            .collect();

        // check whether the tagging agent is close enough to any other agent
    }
    for agent in &agents {
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
            x_bound: settings.width,
            y_bound: settings.height,
        })
        .collect();

    // Set the first agent as 'It'
    agents.get_mut(0).unwrap().is_it = true;
    agents
}
