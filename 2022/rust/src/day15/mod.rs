use crate::utils;

pub struct Solver;

impl utils::Solver<15> for Solver {
    type Part1 = i64;

    type Part2 = i64;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let beacons = parse_input(input).collect::<Vec<_>>();
        let left_x = beacons
            .iter()
            .map(|b| b.position.0 - b.m_dist)
            .min()
            .unwrap();
        let right_x = beacons
            .iter()
            .map(|b| b.position.0 + b.m_dist)
            .max()
            .unwrap();

        let mut count = 0;
        for x in left_x..=right_x {
            let point = (x, 2_000_000);
            for beacon in &beacons {
                if beacon.dist(point) <= beacon.m_dist && beacon.closest != point {
                    count += 1;
                    break;
                }
            }
        }
        Ok(count)
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let beacons = parse_input(input).collect::<Vec<_>>();
        let mut valid = (0, 0);
        for one in &beacons {
            let (x, y) = one.position;
            let mut d_y = 0;
            for n_x in x - one.m_dist - 1..x.min(4000000) {
                if n_x < 0 {
                    d_y += 1;
                    continue;
                }

                let point = (n_x, y + d_y);
                if point.1 <= 4000000 && valid_spot(&beacons, point) {
                    valid = point;
                    break;
                }

                let point = (n_x, y - d_y);
                if point.1 >= 0 && valid_spot(&beacons, point) {
                    valid = point;
                    break;
                }

                d_y += 1;
            }
        }
        Ok(valid.0 * 4000000 + valid.1)
    }
}

type Point = (i64, i64);

struct Beacon {
    position: Point,
    closest: Point,
    m_dist: i64,
}

impl Beacon {
    fn new(position @ (p_x, p_y): Point, closest @ (c_x, c_y): Point) -> Self {
        let m_dist = p_x.abs_diff(c_x) + p_y.abs_diff(c_y);
        let m_dist = m_dist as i64;
        Self {
            position,
            closest,
            m_dist,
        }
    }

    fn dist(&self, (o_x, o_y): Point) -> i64 {
        let m_dist = self.position.0.abs_diff(o_x) + self.position.1.abs_diff(o_y);
        m_dist as i64
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Beacon> + '_ {
    input.lines().map(|l| {
        let i = scan_fmt!(
            l,
            "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
            i64,
            i64,
            i64,
            i64
        );
        let i = i.unwrap();
        Beacon::new((i.0, i.1), (i.2, i.3))
    })
}

fn valid_spot(beacons: &[Beacon], point: Point) -> bool {
    beacons
        .iter()
        .all(|beacon| beacon.dist(point) > beacon.m_dist)
}
