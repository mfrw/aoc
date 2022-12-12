use crate::utils;
use crate::utils::grid::{Grid, GridCoord};
use itertools::Itertools;

pub struct Solver;

impl utils::Solver<8> for Solver {
    type Part1 = usize;

    type Part2 = usize;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let grid = parse_grid(input);
        let all_rows = (0..grid.height()).into_iter().flat_map(|y| {
            let l: Box<dyn Iterator<Item = GridCoord>> = Box::new(
                (0..grid.width())
                    .into_iter()
                    .map(move |x| GridCoord::from((x, y))),
            );
            let r = Box::new(
                (0..grid.width())
                    .into_iter()
                    .rev()
                    .map(move |x| GridCoord::from((x, y))),
            );
            [l as Box<dyn Iterator<Item = GridCoord>>, r].into_iter()
        });

        let all_cols = (0..grid.width()).into_iter().flat_map(|x| {
            let t = Box::new(
                (0..grid.height())
                    .into_iter()
                    .map(move |y| GridCoord::from((x, y))),
            );
            let b = Box::new(
                (0..grid.height())
                    .into_iter()
                    .rev()
                    .map(move |y| GridCoord::from((x, y))),
            );
            [t as Box<dyn Iterator<Item = GridCoord>>, b].into_iter()
        });

        let all_lines = all_rows.chain(all_cols);
        let all_visible = all_lines.flat_map(|it| {
            let mut it = it
                .map(|coord| (coord, *grid.cell(coord).unwrap()))
                .peekable();
            // the first cell from the edge is always visible
            let first = it.peek().unwrap().0;
            std::iter::once(first).chain(
                it.coalesce(|(a_coord, a_height), (b_coord, b_height)| {
                    if b_height <= a_height {
                        Ok((a_coord, a_height))
                    } else {
                        Err(((a_coord, a_height), (b_coord, b_height)))
                    }
                })
                .tuple_windows()
                .map_while(
                    |((_prev_coord, prev_height), (curr_coord, curr_height))| {
                        if prev_height < curr_height {
                            Some(curr_coord)
                        } else {
                            None
                        }
                    },
                ),
            )
        });
        let coords = all_visible.unique().collect_vec();
        Ok(coords.len())
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let grid = parse_grid(input);
        let all_coords = (0..grid.height())
            .into_iter()
            .flat_map(|y| (0..grid.width()).map(move |x| GridCoord::from((x, y))));

        let best_place = all_coords
            .map(|coord| (coord, scenic_score(&grid, coord)))
            .max_by_key(|(_, score)| *score)
            .unwrap();
        Ok(best_place.1)
    }
}

fn parse_grid(input: &str) -> Grid<usize> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid = Grid::new(width, height);
    for (y, line) in input.lines().enumerate() {
        for (x, col) in line.chars().enumerate() {
            assert!(col.is_ascii_digit());
            grid.replace((x, y).into(), col as usize - '0' as usize);
        }
    }

    grid
}

fn visible_trees_in_dir(grid: &Grid<usize>, coord: GridCoord, (dx, dy): (isize, isize)) -> usize {
    let line = (1..).into_iter().map_while(|i| {
        let coord = GridCoord {
            x: coord.x.checked_add_signed(dx * i)?,
            y: coord.y.checked_add_signed(dy * i)?,
        };
        Some(*grid.cell(coord)?)
    });

    let mut total = 0;
    let our_height = *grid.cell(coord).unwrap();
    for height in line {
        total += 1;
        if height >= our_height {
            break;
        }
    }
    total
}

fn scenic_score(grid: &Grid<usize>, coord: GridCoord) -> usize {
    let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    dirs.into_iter()
        .map(|(dx, dy)| visible_trees_in_dir(grid, coord, (dx, dy)))
        .product()
}
