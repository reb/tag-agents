use super::Settings;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn distance(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug)]
struct Agent {
    id: usize, // id also denotes position in the vector
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
        agents = make_agents_tag(agents);
    }
    for agent in &agents {
        println!("{:?}", agent);
    }
}

fn make_agents_tag(mut agents: Vec<Agent>) -> Vec<Agent> {
    let (tagging_agent_id, tagging_agent_position) = agents
        .iter()
        .find(|agent| agent.is_it)
        .map(|agent| (agent.id, agent.position))
        .expect("Did not find an agent that's 'It'");

    let tagged_agent: Option<&mut Agent> = agents.iter_mut().find(|agent| {
        // find an agent that's within distance 2 of the tagging agent and isn't the tagging agent
        agent.id != tagging_agent_id && agent.position.distance(&tagging_agent_position) < 2
    });

    if let Some(agent) = tagged_agent {
        agent.is_it = true;
        agents.get_mut(tagging_agent_id).unwrap().is_it = false;
    }
    agents
}

fn create_agents(settings: &Settings) -> Vec<Agent> {
    // create a random number generator to set agent positions
    let mut rng = rand::thread_rng();

    // create a Vector of agents
    let mut agents: Vec<_> = (0..settings.number_of_agents as usize)
        .map(|id| Agent {
            id,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_distance() {
        assert_eq!(
            Position { x: 1, y: 1 }.distance(&Position { x: 0, y: 0 }),
            2
        );
        assert_eq!(
            Position { x: 0, y: 1 }.distance(&Position { x: 0, y: 0 }),
            1
        );
        assert_eq!(
            Position { x: 1, y: 1 }.distance(&Position { x: 2, y: 2 }),
            2
        );
        assert_eq!(
            Position { x: 1, y: 1 }.distance(&Position { x: 1, y: 1 }),
            0
        );
    }

    #[test]
    fn test_make_agents_tag() {
        let mut agents = vec![
            Agent {
                id: 0,
                position: Position { x: 1, y: 1 },
                is_it: true,
                x_bound: 10,
                y_bound: 10,
            },
            Agent {
                id: 1,
                position: Position { x: 0, y: 1 },
                is_it: false,
                x_bound: 10,
                y_bound: 10,
            },
        ];

        agents = make_agents_tag(agents);

        assert!(!agents[0].is_it, "Expecting agent 0 to not be it");
        assert!(agents[1].is_it, "Expecting agent 1 to be it");
    }

    #[test]
    fn test_make_agents_tag_two_targets() {
        let mut agents = vec![
            Agent {
                id: 0,
                position: Position { x: 1, y: 1 },
                is_it: true,
                x_bound: 10,
                y_bound: 10,
            },
            Agent {
                id: 1,
                position: Position { x: 0, y: 1 },
                is_it: false,
                x_bound: 10,
                y_bound: 10,
            },
            Agent {
                id: 2,
                position: Position { x: 0, y: 1 },
                is_it: false,
                x_bound: 10,
                y_bound: 10,
            },
        ];

        agents = make_agents_tag(agents);

        // count the agents that are 'It'
        agents = agents.into_iter().filter(|agent| agent.is_it).collect();
        assert_eq!(agents.len(), 1, "Only expecting 1 agent to be it");
    }

    #[test]
    fn test_make_agents_tag_no_target() {
        let mut agents = vec![
            Agent {
                id: 0,
                position: Position { x: 1, y: 1 },
                is_it: true,
                x_bound: 10,
                y_bound: 10,
            },
            Agent {
                id: 1,
                position: Position { x: 0, y: 0 },
                is_it: false,
                x_bound: 10,
                y_bound: 10,
            },
        ];

        agents = make_agents_tag(agents);

        assert!(agents[0].is_it, "Expecting agent 0 to still be it");
        assert!(!agents[1].is_it, "Expecting agent 1 to still not be it");
    }
}
