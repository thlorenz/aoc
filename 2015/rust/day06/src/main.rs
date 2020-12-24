use regex::Regex;

use InstructionType::*;

type Coord = (usize, usize);

#[derive(Debug, PartialEq)]
struct Locations {
    from: Coord,
    through: Coord,
}

#[derive(Debug, PartialEq)]
enum InstructionType {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    locations: Locations,
    typ: InstructionType,
}

impl Instruction {
    fn from_strings(
        typ: &str,
        from_x: &str,
        from_y: &str,
        through_x: &str,
        through_y: &str,
    ) -> Instruction {
        let locations = {
            let from = (
                from_x.parse().expect("invalid from_x"),
                from_y.parse().expect("invalid from_y"),
            );
            let through = (
                through_x.parse().expect("invalid through_x"),
                through_y.parse().expect("invalid through_y"),
            );
            Locations { from, through }
        };

        match typ {
            "turn on" => Instruction {
                locations,
                typ: TurnOn,
            },
            "turn off" => Instruction {
                locations,
                typ: TurnOff,
            },
            "toggle" => Instruction {
                locations,
                typ: Toggle,
            },
            _ => panic!("Unknown instruction {}", &typ),
        }
    }
}

struct Grid {
    lights: Vec<bool>,
    dimmables: Vec<u32>,
    ncols: usize,
}

impl Grid {
    fn new(ncols: usize, nrows: usize) -> Self {
        let ncells = ncols * nrows;
        let lights = vec![false; ncells];
        let dimmables = vec![0; ncells];
        Self {
            lights,
            dimmables,
            ncols,
        }
    }

    fn idx(&self, col: usize, row: usize) -> usize {
        row * self.ncols + col
    }

    fn on(&mut self, col: usize, row: usize) {
        let idx = self.idx(col, row);
        self.lights[idx] = true;
        self.dimmables[idx] += 1;
    }

    fn off(&mut self, col: usize, row: usize) {
        let idx = self.idx(col, row);
        self.lights[idx] = false;
        self.dimmables[idx] -= if self.dimmables[idx] == 0 { 0 } else { 1 };
    }

    fn toggle(&mut self, col: usize, row: usize) {
        let idx = self.idx(col, row);
        self.lights[idx] = !self.lights[idx];
        self.dimmables[idx] += 2;
    }

    fn nlit(&self) -> usize {
        self.lights.iter().filter(|&x| *x).count()
    }

    fn brightness(&self) -> u32 {
        self.dimmables.iter().sum()
    }
}

fn process_instruction(grid: &mut Grid, instruction: &Instruction) {
    let Instruction {
        typ,
        locations: Locations { from, through },
    } = instruction;
    for row in from.0..=through.0 {
        for col in from.1..=through.1 {
            match typ {
                TurnOn => grid.on(col, row),
                TurnOff => grid.off(col, row),
                Toggle => grid.toggle(col, row),
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let rx = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    rx.captures_iter(input)
        .filter_map(|x| {
            let groups = (x.get(1), x.get(2), x.get(3), x.get(4), x.get(5));
            match groups {
                (Some(typ), Some(from_x), Some(from_y), Some(through_x), Some(through_y)) => {
                    Some(Instruction::from_strings(
                        typ.as_str(),
                        from_x.as_str(),
                        from_y.as_str(),
                        through_x.as_str(),
                        through_y.as_str(),
                    ))
                }
                _ => None,
            }
        })
        .collect()
}

fn main() {
    let input = include_str!("input.txt");
    let mut grid = Grid::new(1000, 1000);
    let input = parse_input(input);
    for instruction in input {
        process_instruction(&mut grid, &instruction);
    }
    println!("part 1 lights on: {}", grid.nlit());
    println!("part 2 brightness: {}", grid.brightness());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn input_parsing() {
        assert_eq!(
            parse_input(
                "\
            \x20turn on 454,398 through 844,448\n\
            \x20turn off 539,243 through 559,965\n\
            \x20toggle 720,196 through 897,994\
            ",
            ),
            vec![
                Instruction {
                    typ: TurnOn,
                    locations: Locations {
                        from: (454, 398),
                        through: (844, 448)
                    }
                },
                Instruction {
                    typ: TurnOff,
                    locations: Locations {
                        from: (539, 243),
                        through: (559, 965)
                    },
                },
                Instruction {
                    typ: Toggle,
                    locations: Locations {
                        from: (720, 196),
                        through: (897, 994)
                    },
                },
            ]
        );
    }

    #[test]
    fn process_entire_grid() {
        let mut grid = Grid::new(1000, 1000);
        let instructions = parse_input("turn on 0,0 through 999,999");
        process_instruction(&mut grid, &instructions[0]);
        assert_eq!(grid.nlit(), 1_000_000);
    }

    #[test]
    fn process_first_line() {
        let mut grid = Grid::new(1000, 1000);
        let instructions = parse_input("toggle 0,0 through 999,0");
        process_instruction(&mut grid, &instructions[0]);
        assert_eq!(grid.nlit(), 1_000);
    }

    #[test]
    fn process_middle_four_on() {
        let mut grid = Grid::new(1000, 1000);
        let instructions = parse_input("turn on 499,499 through 500,500");
        process_instruction(&mut grid, &instructions[0]);
        assert_eq!(grid.nlit(), 4);
    }

    #[test]
    fn process_middle_four_off() {
        let mut grid = Grid::new(1000, 1000);
        let instructions = parse_input(
            "\
            \x20turn on 0,0 through 999,999\n\
            \x20turn off 499,499 through 500,500\n\
        ",
        );
        for instruction in instructions {
            process_instruction(&mut grid, &instruction);
        }
        assert_eq!(grid.nlit(), 1_000_000 - 4);
    }
}
