use super::display;
use super::Settings;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::thread;
use std::time::Duration;

pub fn simulate(settings: Settings) {
    let py_simple = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/simple.py"));
    let py_utils = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python/utils.py"));
    let gil = Python::acquire_gil();
    let py = gil.python();

    // load the utils module
    PyModule::from_code(py, py_utils, "utils", "utils").unwrap();
    let simple_decide_action: Py<PyAny> = PyModule::from_code(py, py_simple, "", "")
        .unwrap()
        .getattr("decide_action")
        .unwrap()
        .into();

    let mut agents = create_agents(&settings);

    display::init(&settings);
    display::show(&agents, &settings);

    for _ in 0..settings.length {
        // grab the tagging agent's position to pass as an argument
        let tagging_agent_position = agents
            .iter()
            .find(|agent| agent.is_it)
            .map(|agent| agent.position)
            .unwrap();

        // all agents take an action
        let actions: Vec<Action> = agents
            .iter()
            .map(|agent| {
                let kwargs = PyDict::new(py);

                kwargs.set_item("is_it", agent.is_it).ok();
                kwargs.set_item("position", agent.position.as_tuple()).ok();
                kwargs
                    .set_item("tagging_agent_position", tagging_agent_position.as_tuple())
                    .ok();
                // TODO: pass all other agent positions

                // match the result from the python module back to the Rust enum
                match simple_decide_action
                    .call(py, (), Some(kwargs))
                    .unwrap()
                    .extract::<u8>(py)
                    .unwrap()
                {
                    0 => Action::Stay,
                    1 => Action::Up,
                    2 => Action::Right,
                    3 => Action::Down,
                    4 => Action::Left,
                    _ => rand::random(),
                }
            })
            .collect();
        agents = agents
            .into_iter()
            .zip(actions.into_iter())
            .map(|(agent, action)| agent.execute_action(action))
            .collect();

        // check whether the tagging agent is close enough to any other agent
        agents = make_agents_tag(agents);

        display::show(&agents, &settings);

        if settings.speed > 0 {
            thread::sleep(Duration::from_millis(1000 / settings.speed as u64))
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    fn distance(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn as_tuple(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

#[derive(Debug)]
pub struct Agent {
    id: usize, // id also denotes position in the vector
    pub position: Position,
    pub is_it: bool,
    has_immunity: bool,
    x_bound: u32, // upper bound, lower bound is 0
    y_bound: u32, // upper bound, lower bound is 0
}

impl Agent {
    fn decide_action(&self, agents: &Vec<Agent>) -> Action {
        match self.is_it {
            true => {
                // find closest agent that's not immune or itself
                let closest_agent = agents
                    .iter()
                    .filter(|agent| !agent.has_immunity && !(agent.id == self.id))
                    .min_by_key(|agent| agent.position.distance(&self.position))
                    .expect("Expected there to be another agent");
                if self.position.x.abs_diff(closest_agent.position.x)
                    >= self.position.y.abs_diff(closest_agent.position.y)
                {
                    if self.position.x >= closest_agent.position.x {
                        Action::Left
                    } else {
                        Action::Right
                    }
                } else {
                    if self.position.y >= closest_agent.position.y {
                        Action::Up
                    } else {
                        Action::Down
                    }
                }
            }
            false => {
                // find tagging agent
                let tagging_agent = agents
                    .iter()
                    .find(|agent| agent.is_it)
                    .expect("Expected there to be a tagging agent");
                if self.position.distance(&tagging_agent.position) > 4 {
                    return rand::random();
                }
                if self.position.x.abs_diff(tagging_agent.position.x)
                    < self.position.y.abs_diff(tagging_agent.position.y)
                {
                    if self.position.x < tagging_agent.position.x {
                        Action::Left
                    } else {
                        Action::Right
                    }
                } else {
                    if self.position.y < tagging_agent.position.y {
                        Action::Up
                    } else {
                        Action::Down
                    }
                }
            }
        }
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

fn make_agents_tag(mut agents: Vec<Agent>) -> Vec<Agent> {
    let (tagging_agent_id, tagging_agent_position) = agents
        .iter()
        .find(|agent| agent.is_it)
        .map(|agent| (agent.id, agent.position))
        .expect("Did not find an agent that's 'It'");

    let tagged_agent: Option<&mut Agent> = agents.iter_mut().find(|agent| {
        // find an agent that's within distance 2 of the tagging agent and isn't the tagging agent
        let not_the_tagging_agent = agent.id != tagging_agent_id;
        let agent_close_enough = agent.position.distance(&tagging_agent_position) < 2;
        let agent_not_immune = !agent.has_immunity;
        not_the_tagging_agent && agent_not_immune && agent_close_enough
    });

    if let Some(agent) = tagged_agent {
        agent.is_it = true;

        // look for an immune agent to make sure it loses that immunity
        if let Some(immune_agent) = agents.iter_mut().find(|agent| agent.has_immunity) {
            immune_agent.has_immunity = false;
        }

        // the tagging_agent will not be 'It' anymore and has immunity until another tag
        let mut tagging_agent = agents.get_mut(tagging_agent_id).unwrap();
        tagging_agent.is_it = false;
        tagging_agent.has_immunity = true;
    }
    agents
}

fn create_agents(settings: &Settings) -> Vec<Agent> {
    // create a random number generator to set agent positions
    let mut rng = rand::thread_rng();

    // create a Vector of agents
    let mut agents: Vec<_> = (0..settings.agents as usize)
        .map(|id| Agent {
            id,
            position: Position {
                x: rng.gen_range(0..settings.width),
                y: rng.gen_range(0..settings.height),
            },
            is_it: false,
            has_immunity: false,
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
                has_immunity: false,
                x_bound: 10,
                y_bound: 10,
            },
            Agent {
                id: 1,
                position: Position { x: 0, y: 1 },
                is_it: false,
                has_immunity: false,
                x_bound: 10,
                y_bound: 10,
            },
        ];

        agents = make_agents_tag(agents);

        assert!(!agents[0].is_it, "Expecting agent 0 to not be it");
        assert!(agents[1].is_it, "Expecting agent 1 to be it");
        assert!(agents[0].has_immunity, "Expecting agent 0 to have immunity");
    }

    #[test]
    fn test_make_agents_tag_two_targets() {
        let mut agents = vec![
            Agent {
                id: 0,
                position: Position { x: 1, y: 1 },
                is_it: true,
                has_immunity: false,
                x_bound: 10,
                y_bound: 10,
            },
            Agent {
                id: 1,
                position: Position { x: 0, y: 1 },
                is_it: false,
                has_immunity: false,
                x_bound: 10,
                y_bound: 10,
            },
            Agent {
                id: 2,
                position: Position { x: 0, y: 1 },
                is_it: false,
                has_immunity: false,
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
                has_immunity: false,
                x_bound: 10,
                y_bound: 10,
            },
            Agent {
                id: 1,
                position: Position { x: 0, y: 0 },
                is_it: false,
                has_immunity: false,
                x_bound: 10,
                y_bound: 10,
            },
        ];

        agents = make_agents_tag(agents);

        assert!(agents[0].is_it, "Expecting agent 0 to still be it");
        assert!(!agents[1].is_it, "Expecting agent 1 to still not be it");
    }

    #[test]
    fn test_make_agents_tag_has_immunity() {
        let mut agents = vec![
            Agent {
                id: 0,
                position: Position { x: 1, y: 1 },
                is_it: true,
                has_immunity: false,
                x_bound: 10,
                y_bound: 10,
            },
            Agent {
                id: 1,
                position: Position { x: 1, y: 0 },
                is_it: false,
                has_immunity: true,
                x_bound: 10,
                y_bound: 10,
            },
        ];

        agents = make_agents_tag(agents);

        assert!(agents[0].is_it, "Expecting agent 0 to still be it");
        assert!(!agents[1].is_it, "Expecting agent 1 to still not be it");
    }

    #[test]
    fn test_make_agents_tag_has_immunity_two_targets() {
        let mut agents = vec![
            Agent {
                id: 0,
                position: Position { x: 1, y: 1 },
                is_it: true,
                has_immunity: false,
                x_bound: 10,
                y_bound: 10,
            },
            Agent {
                id: 1,
                position: Position { x: 1, y: 0 },
                is_it: false,
                has_immunity: true,
                x_bound: 10,
                y_bound: 10,
            },
            Agent {
                id: 1,
                position: Position { x: 0, y: 1 },
                is_it: false,
                has_immunity: false,
                x_bound: 10,
                y_bound: 10,
            },
        ];

        agents = make_agents_tag(agents);

        assert!(!agents[0].is_it, "Expecting agent 0 to not be it");
        assert!(!agents[1].is_it, "Expecting agent 1 to not be it");
        assert!(agents[2].is_it, "Expecting agent 2 to be it");
        assert!(agents[0].has_immunity, "Expecting agent 0 to have immunity");
        assert!(
            !agents[1].has_immunity,
            "Expecting agent 1 to not have immunity"
        );
        assert!(
            !agents[2].has_immunity,
            "Expecting agent 2 to not have immunity"
        );
    }
}
