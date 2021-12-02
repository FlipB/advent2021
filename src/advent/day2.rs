use std::str::FromStr;

use anyhow::Result;

use super::input;

pub fn print_position(input: impl std::io::Read) -> Result<()> {
    let cmds = parse_commands(input)?;

    let mut pos = Position { x: 0, z: 0, v: 0 };
    pos.part1_apply_commands(cmds.as_slice());
    println!("Predicted Position: X*Z = {}", pos.value());

    let mut pos = Position { x: 0, z: 0, v: 0 };
    pos.part2_apply_commands(cmds.as_slice());
    println!("Predicted Position using Aim: X*Z = {}", pos.value());

    Ok(())
}

struct Position {
    /// x track horizontal position
    x: i64,
    /// z tracks vertical position
    z: i64,
    /// v tracks aim (only used for part 2)
    v: i64,
}

impl Position {
    fn part1_apply_commands(&mut self, cmds: &[Command]) {
        for cmd in cmds {
            match cmd {
                Command::Forward(n) => self.x += n,
                Command::Backward(n) => self.x -= n,
                Command::Up(n) => self.z -= n,
                Command::Down(n) => self.z += n,
            };
        }
    }

    fn part2_apply_commands(&mut self, cmds: &[Command]) {
        for cmd in cmds {
            match cmd {
                Command::Forward(n) => {
                    self.x += n;
                    self.z += self.v * n;
                }
                Command::Backward(n) => self.x -= n,
                Command::Up(n) => self.v -= n,
                Command::Down(n) => self.v += n,
            };
        }
    }

    fn value(&self) -> i64 {
        self.x * self.z
    }
}

enum Command {
    Forward(i64),
    Backward(i64),
    Up(i64),
    Down(i64),
}

impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Command::from_str(value)
    }
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Can we do this with a match and destructuring?
        let cmd = match s.split_once(" ") {
            Some(("forward", n)) => Command::Forward(n.parse()?),
            Some(("backward", n)) => Command::Backward(n.parse()?),
            Some(("up", n)) => Command::Up(n.parse()?),
            Some(("down", n)) => Command::Down(n.parse()?),
            _ => return Err(anyhow::format_err!("bad command")),
        };
        Ok(cmd)
    }
}

fn parse_commands(input: impl std::io::Read) -> Result<Vec<Command>> {
    input::get_input_lines(input)?
        .iter()
        .map(String::as_str)
        .map(Command::from_str)
        .collect()
}
