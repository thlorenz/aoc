// https://adventofcode.com/2015/day/3
use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_byte(n: &u8) -> Option<Self> {
        // b'^' => ... would work as well
        match n {
            62 => Some(Direction::Right),
            94 => Some(Direction::Up),
            118 => Some(Direction::Down),
            60 => Some(Direction::Left),
            _ => None,
        }
    }

    fn next_loc(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (x, y + 1),
            Direction::Down => (x, y - 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

#[derive(Debug, Clone)]
struct Delivery {
    visited: HashSet<(i32, i32)>,
    santa_loc: (i32, i32),
    robot_loc: (i32, i32),
}

fn main() {
    let input = include_bytes!("input.txt");
    let directions: Vec<Direction> = input.into_iter().filter_map(Direction::from_byte).collect();

    let start_delivery = {
        let mut visited = HashSet::new();
        let start_loc = (0, 0);
        visited.insert(start_loc);
        Delivery {
            visited,
            santa_loc: start_loc,
            robot_loc: start_loc,
        }
    };

    //
    // Part 1
    //
    let santa_only_delivery: &Delivery =
        &directions
            .iter()
            .fold(start_delivery.clone(), |mut delivery, direction| {
                let Delivery {
                    ref mut visited,
                    santa_loc: (x, y),
                    ..
                } = delivery;
                let loc = direction.next_loc((x, y));
                delivery.santa_loc = loc;
                visited.insert(loc);

                delivery
            });

    let santa_only_at_least_one = santa_only_delivery.visited.len();
    println!("{:?}", santa_only_at_least_one);

    //
    // Part 2
    //
    // Instead of indexed positions and separating them in fold we could have done
    // `directions.into_iter().step_by(2)` and `directions.into_iter().skip(1).step_by(2)`
    let indexed_directions = directions.into_iter().zip(0..).into_iter();
    let santa_and_robot_delivery: &Delivery =
        &indexed_directions.fold(start_delivery.clone(), |mut delivery, (direction, idx)| {
            let Delivery {
                ref mut visited,
                santa_loc,
                robot_loc,
            } = delivery;

            let loc = if idx % 2 == 0 {
                delivery.santa_loc = direction.next_loc(santa_loc);
                delivery.santa_loc
            } else {
                delivery.robot_loc = direction.next_loc(robot_loc);
                delivery.robot_loc
            };
            visited.insert(loc);
            delivery
        });

    let santa_and_robot_at_least_one = santa_and_robot_delivery.visited.len();
    println!("{:?}", santa_and_robot_at_least_one);
}
