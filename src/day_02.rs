use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Error, Debug)]
enum CommandParseError {
    #[error("invalid command format: {0}")]
    InvalidFormat(String),
    #[error("invalid command name: {0}")]
    InvalidCommand(String),
    #[error("failed to parse distance: {0}")]
    InvalidDistance(ParseIntError),
}

/// allows let command_parse_error: CommandParseError = parse_error.into()
impl From<ParseIntError> for CommandParseError {
    fn from(e: ParseIntError) -> Self {
        CommandParseError::InvalidDistance(e)
    }
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let name: &str = parts.next().ok_or(CommandParseError::InvalidFormat(
            "line contains no command".into(),
        ))?;
        let distance: i32 = parts
            .next()
            .ok_or(CommandParseError::InvalidFormat(
                "line contains no distance".into(),
            ))?
            .parse()?;

        match name {
            "forward" => Ok(Command::Forward(distance)),
            "down" => Ok(Command::Down(distance)),
            "up" => Ok(Command::Up(distance)),
            other => Err(CommandParseError::InvalidCommand(other.into())),
        }
    }
}

#[derive(Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    fn new(horizontal: i32, depth: i32) -> Self {
        Position { horizontal, depth }
    }
}

fn navigate(pos: &Position, command: &Command) -> Position {
    match command {
        Command::Forward(d) => Position::new(pos.horizontal + d, pos.depth),
        Command::Down(d) => Position::new(pos.horizontal, pos.depth + d),
        Command::Up(d) => Position::new(pos.horizontal, pos.depth - d),
    }
}

#[derive(Debug)]
struct Position2 {
    horizontal: i32, depth: i32, aim: i32,
}

impl Position2 {
    fn new(horizontal: i32, depth: i32, aim: i32) -> Self {
        Self { horizontal, depth, aim }
    }
}

fn navigate2(pos: &Position2, command: &Command) -> Position2 {
    match command {
        Command::Forward(d) => Position2::new(pos.horizontal + d, pos.depth + pos.aim * d, pos.aim),
        Command::Down(d) => Position2 { aim: pos.aim + d, ..*pos },
        Command::Up(d) => Position2 { aim: pos.aim - d, ..*pos },
    }
}

mod task_01 {
    // implementation goes here

    #[cfg(test)]
    mod test {
        use crate::day_02::{navigate, Command, Position};
        use crate::input::read_input;
        use std::error::Error;

        #[test]
        fn example() -> Result<(), Box<dyn Error>> {
            let input = read_input("inputs/day_02/example.txt")?;
            let commands: Vec<Command> =
                input.lines().map(&str::parse).collect::<Result<_, _>>()?;
            let final_position = commands
                .iter()
                .fold(Position::new(0, 0), |pos, cmd| navigate(&pos, &cmd));
            assert_eq!(final_position.horizontal * final_position.depth, 150);

            Ok(())
        }

        #[test]
        fn task() -> Result<(), Box<dyn Error>> {
            let input = read_input("inputs/day_02/input.txt")?;
            let commands: Vec<Command> =
                input.lines().map(&str::parse).collect::<Result<_, _>>()?;
            let final_position = commands
                .iter()
                .fold(Position::new(0, 0), |pos, cmd| navigate(&pos, &cmd));
            assert_eq!(final_position.horizontal * final_position.depth, 1714950);

            Ok(())
        }
    }
}

mod task_02 {
    // implementation goes here

    #[cfg(test)]
    mod test {
        use crate::day_02::{Command, Position2, navigate2};
        use crate::input::read_input;
        use std::error::Error;

        #[test]
        fn example() -> Result<(), Box<dyn Error>>{
            let input = read_input("inputs/day_02/example.txt")?;
            let commands: Vec<Command> =
                input.lines().map(&str::parse).collect::<Result<_, _>>()?;
            let position = commands
                .iter()
                .fold(Position2::new(0, 0, 0), |pos, cmd| navigate2(&pos, &cmd));
            assert_eq!(position.horizontal * position.depth, 900);

            Ok(())
        }

        #[test]
        fn task() -> Result<(), Box<dyn Error>> {
            let input = read_input("inputs/day_02/input.txt")?;
            let commands: Vec<Command> =
                input.lines().map(&str::parse).collect::<Result<_, _>>()?;
            let position = commands
                .iter()
                .fold(Position2::new(0, 0, 0), |pos, cmd| navigate2(&pos, &cmd));
            assert_eq!(position.horizontal * position.depth, 1281977850);

            Ok(())
        }
    }
}
