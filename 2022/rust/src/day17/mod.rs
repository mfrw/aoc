use crate::utils;

pub struct Solver;

impl utils::Solver<17> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let mut ops = input.chars().cycle();
        let mut chamber = Chamber::new();
        let mut count: usize = 0;
        while count < 2022 {
            match ops.next().unwrap() {
                '>' => chamber.move_right(),
                '<' => chamber.move_left(),
                _ => continue,
            }
            if chamber.move_down() {
                count += 1;
            }
        }
        Ok(chamber.height)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let mut ops = input.chars().cycle();
        let mut chamber = Chamber::new();
        let mut deltas = [0; 5000];
        let mut previous = 0;
        let mut count = 0;
        while count < 5000 {
            match ops.next().unwrap() {
                '>' => chamber.move_right(),
                '<' => chamber.move_left(),
                _ => continue,
            }
            if chamber.move_down() {
                deltas[count] = chamber.height - previous;
                previous = chamber.height;
                count += 1;
            }
        }
        let (offset, size) = (0..500)
            .find_map(|offset| {
                let delta_iter = deltas.iter().skip(offset);
                let size = (2..=2500).find(|size| {
                    let window = deltas[offset..offset + size].iter().cycle();
                    delta_iter.clone().zip(window).all(|(a, b)| a == b)
                });
                size.map(|size| (offset, size))
            })
            .unwrap();
        let mut delta_iter = deltas.iter();
        let mut count = 1_000_000_000_000;
        let offset_delta = delta_iter.by_ref().take(offset).sum::<usize>();
        count -= offset;
        let cycle_deltas: Vec<usize> = delta_iter.take(size).copied().collect();
        let cycle_delta = cycle_deltas.iter().sum::<usize>();
        let cycle_count = count / size;
        count %= size;
        let remaining_height = cycle_deltas.into_iter().take(count).sum::<usize>();
        let height: usize = offset_delta + cycle_count * cycle_delta + remaining_height;

        Ok(height)
    }
}
type Point = (usize, usize);

//- Stores the point at the lower left corner
#[derive(Debug)]
struct Rock {
    rock_type: RockType,
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum RockType {
    Horizontal = 0,
    Cross = 1,
    Angle = 2,
    Vertical = 3,
    Square = 4,
}

impl RockType {
    fn set_next_type(&mut self) {
        *self = match self {
            RockType::Horizontal => Self::Cross,
            RockType::Cross => Self::Angle,
            RockType::Angle => Self::Vertical,
            RockType::Vertical => Self::Square,
            RockType::Square => Self::Horizontal,
        }
    }
}

impl Rock {
    fn new(x: usize, y: usize) -> Self {
        Self {
            rock_type: RockType::Horizontal,
            x,
            y,
        }
    }

    /// Finds the points of the rock based on the lower left point of the bounding box
    fn get_points(&self) -> Vec<Point> {
        let (x, y) = (self.x, self.y);
        match self.rock_type {
            RockType::Horizontal => (0..4).map(|delta| (x + delta, y)).collect(),
            RockType::Cross => vec![
                (x, y + 1),
                (x + 1, y + 1),
                (x + 2, y + 1),
                (x + 1, y),
                (x + 1, y + 2),
            ],
            RockType::Angle => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            RockType::Vertical => (0..4).map(|delta| (x, y + delta)).collect(),
            RockType::Square => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        }
    }

    fn set_next_type(&mut self) {
        self.rock_type.set_next_type();
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Air,
    Rock,
}

#[derive(Debug)]
struct Chamber {
    grid: Vec<Tile>,
    height: usize,
    rock: Rock,
}

fn to_index((x, y): Point) -> usize {
    y * 7 + x
}

impl Chamber {
    fn new() -> Self {
        Self {
            grid: vec![Tile::Air; 7],
            height: 1,
            rock: Rock::new(2, 3),
        }
    }

    fn get(&self, point: Point) -> Tile {
        *self.grid.get(to_index(point)).unwrap_or(&Tile::Air)
    }

    fn is_occupied(&self, point: Point) -> bool {
        self.get(point) == Tile::Rock
    }

    fn set(&mut self, (x, y): Point) {
        if y > self.height - 1 {
            // need to add row
            self.height = y + 1;
            self.grid
                .extend(vec![Tile::Air; (self.height + 1 - self.grid.len() / 7) * 7]);
        }
        // below height of container
        self.grid[to_index((x, y))] = Tile::Rock;
    }

    fn move_down(&mut self) -> bool {
        let landed = self
            .rock
            .get_points()
            .into_iter()
            .any(|(x, y)| y == 0 || self.is_occupied((x, y - 1)));
        if landed {
            // set the rock in stone
            for point in self.rock.get_points() {
                self.set(point);
            }
            // change to next type of rock
            self.rock.set_next_type();
            // reset position;
            self.rock.y = self.height + 3; // offset one in the example
            self.rock.x = 2; // rule from the example
        } else {
            self.rock.y -= 1;
        }
        landed
    }

    fn move_right(&mut self) {
        let bordering_right = self
            .rock
            .get_points()
            .into_iter()
            .any(|(x, y)| x == 6 || self.is_occupied((x + 1, y)));
        if !bordering_right {
            self.rock.x += 1;
        }
    }

    fn move_left(&mut self) {
        let bordering_left = self
            .rock
            .get_points()
            .into_iter()
            .any(|(x, y)| x == 0 || self.is_occupied((x - 1, y)));
        if !bordering_left {
            self.rock.x -= 1;
        }
    }
}

impl std::fmt::Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rock_points = self.rock.get_points();
        for y in (0..self.height + 3).rev() {
            for x in 0..7 {
                if rock_points.contains(&(x, y)) {
                    write!(f, "@")?;
                } else {
                    let tile = self.get((x, y));
                    let out = match tile {
                        Tile::Air => '.',
                        Tile::Rock => '#',
                    };
                    write!(f, "{}", out)?;
                }
            }
            write!(f, " {y}")?;
            writeln!(f)?;
        }
        Ok(())
    }
}
