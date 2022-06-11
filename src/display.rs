use super::simulator::Agent;
use super::Settings;
use console::Term;
use std::iter;

pub fn init(settings: &Settings) {
    // move the cursor down to the bottom of the display
    println!(
        "{}",
        iter::repeat('\n')
            .take(settings.height as usize + 2) // there are two horizontal boundaries extra
            .collect::<String>()
    );
}

pub fn show(agents: &Vec<Agent>, settings: &Settings) {
    // create an empty screen using Vectors of Vectors of chars
    let mut screen = iter::repeat(
        iter::repeat(' ')
            .take(settings.width as usize)
            .collect::<Vec<char>>(),
    )
    .take(settings.height as usize)
    .collect::<Vec<_>>();

    // place the agents on the screen
    for agent in agents {
        let agent_character = match agent.is_it {
            false => 'O',
            true => '#',
        };

        let line = screen.get_mut(agent.position.y as usize).unwrap();
        *line.get_mut(agent.position.x as usize).unwrap() = agent_character;
    }

    // move the cursor back up to overwrite the existing screen
    Term::stdout()
        .move_cursor_up(settings.height as usize + 2) // consider the horizontal boundaries
        .ok();

    let wall = '\u{2588}'; // block character
    let horizontal_boundary = iter::repeat(wall)
        .take(settings.width as usize + 2) // there are two vertical walls as well
        .collect::<String>();
    println!("{}", horizontal_boundary);
    for line in screen {
        println!("{}{}{}", wall, line.iter().collect::<String>(), wall);
    }
    println!("{}", horizontal_boundary);
}
