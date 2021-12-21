use anyhow::Result;

use super::input;

pub fn print_result(input: impl std::io::Read) -> Result<()> {
    let target = parse_target_area(input)?;
    let max_x = std::cmp::max(target.0.x, target.1.x);
    // only relevant if target is below origin. Since y velocity increases every tick
    // it will overshoot past this.
    let min_y = std::cmp::min(target.0.y, target.1.y).abs();

    let mut peak_position: Position = (0, 0).into();
    let mut hit_count = 0;
    for yv in -min_y..=min_y {
        for xv in 0..=max_x {
            let (hit, peak) = simulate_trajectory((xv, yv), target);
            if !hit {
                continue;
            }
            hit_count += 1;
            if peak.y > peak_position.y {
                peak_position = peak;
            }
        }
    }

    println!(
        "Part 1: Found peak at (x={}, y={})",
        peak_position.x, peak_position.y
    );

    println!(
        "Part 2: Number of distinct velocities to hit = {}",
        hit_count,
    );

    Ok(())
}

// returns true if target is hit, and returns the *peak* position in the arc.
fn simulate_trajectory(
    initial_velocity: (isize, isize),
    target: (Position, Position),
) -> (bool, Position) {
    let mut probe = Probe::new(initial_velocity);
    let x_lim = std::cmp::max(target.0.x, target.1.x);
    let y_lim = std::cmp::min(target.0.y, target.1.y);

    let mut peak = probe.pos;
    let mut hit = false;

    while probe.pos.x <= x_lim && probe.pos.y >= y_lim {
        probe.tick();
        if peak.y <= probe.pos.y {
            peak = probe.pos
        }
        if probe.within_bounds(target) {
            hit = true;
            break;
        }
    }
    (hit, peak)
}

struct Probe {
    vx: isize,
    vy: isize,
    pos: Position,
}

impl Probe {
    fn new(initial_velocity: (isize, isize)) -> Self {
        Self {
            vx: initial_velocity.0,
            vy: initial_velocity.1,
            pos: (0, 0).into(),
        }
    }

    fn tick(&mut self) {
        self.pos.x += self.vx;
        self.pos.y += self.vy;
        if self.vx > 0 {
            self.vx -= 1;
        } else if self.vx < 0 {
            self.vx += 1;
        }
        self.vy -= 1;
    }

    fn within_bounds(&self, bounds: (Position, Position)) -> bool {
        let x_within = self.pos.x <= std::cmp::max(bounds.0.x, bounds.1.x)
            && self.pos.x >= std::cmp::min(bounds.0.x, bounds.1.x);
        let y_within = self.pos.y <= std::cmp::max(bounds.0.y, bounds.1.y)
            && self.pos.y >= std::cmp::min(bounds.0.y, bounds.1.y);
        x_within && y_within
    }
}

fn parse_target_area(input: impl std::io::Read) -> Result<(Position, Position)> {
    let lines = input::get_input_lines(input)?;
    let line = lines.get(0).ok_or(anyhow::Error::msg("bad input"))?;
    if !line.starts_with("target area: ") {
        return Err(anyhow::Error::msg("bad input"));
    }
    let mut positions = (Position::new(0, 0), Position::new(0, 0));
    for part in line
        .strip_prefix("target area: ")
        .ok_or(anyhow::Error::msg("bad input"))?
        .split(", ")
    {
        let (xy, range) = part
            .split_once("=")
            .ok_or(anyhow::Error::msg("bad input"))?;
        let (pos1, pos2) = match xy {
            "x" => (&mut positions.0.x, &mut positions.1.x),
            "y" => (&mut positions.0.y, &mut positions.1.y),
            _ => return Err(anyhow::Error::msg("bad input")),
        };
        let (start, end) = range
            .split_once("..")
            .ok_or(anyhow::Error::msg("bad input"))?;
        *pos1 = start.parse()?;
        *pos2 = end.parse()?;
    }

    Ok(positions)
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl From<(isize, isize)> for Position {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}
