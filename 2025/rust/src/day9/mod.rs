use crate::utils;

pub struct Solver;

impl utils::Solver<9> for Solver {
    type Part1 = usize;

    type Part2 = u64;

    fn part1(&self, input: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let points = parse_input(input);
        Ok(max_area(&points))
    }

    fn part2(&self, input: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        Ok(solve_part2(&parse_input(input)))
    }
}

#[derive(Clone, Copy)]
struct Point(usize, usize);

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .split_once(",")
                .map(|(a, b)| Point(a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .unwrap()
        })
        .collect()
}

fn max_area(pts: &[Point]) -> usize {
    let mut mx = 0;
    for i in 0..pts.len() {
        for j in i + 1..pts.len() {
            let w = pts[i].0.abs_diff(pts[j].0) + 1;
            let h = pts[i].1.abs_diff(pts[j].1) + 1;
            mx = mx.max(w * h);
        }
    }
    mx
}

fn solve_part2(coords: &[Point]) -> u64 {
    let mut max_area = 0;

    for (i, &p1) in coords.iter().enumerate() {
        for &p2 in coords.iter().skip(i + 1) {
            let x1 = p1.0.min(p2.0);
            let x2 = p1.0.max(p2.0);
            let y1 = p1.1.min(p2.1);
            let y2 = p1.1.max(p2.1);

            let width = (x2 - x1) as u64 + 1;
            let height = (y2 - y1) as u64 + 1;
            let area = width * height;

            if area <= max_area {
                continue;
            }

            if is_valid_rect(x1, x2, y1, y2, coords) {
                max_area = area;
            }
        }
    }
    max_area
}

fn is_valid_rect(x1: usize, x2: usize, y1: usize, y2: usize, poly: &[Point]) -> bool {
    // 1. Check if center is inside or on boundary
    // We scale everything by 2 to stay in integers.
    // Center of (x1, x2) is (x1+x2)/2. Scaled by 2, it is simply (x1+x2).
    let mx = x1 as u64 + x2 as u64;
    let my = y1 as u64 + y2 as u64;

    if !is_point_in_poly(mx, my, poly) {
        return false;
    }

    let len = poly.len();
    // 2. Check if any edge intersects the INTERIOR of the rectangle
    for i in 0..len {
        let u = poly[i];
        let v = poly[(i + 1) % len];

        if u.0 == v.0 {
            // Vertical edge at ex
            let ex = u.0;
            let ey_min = u.1.min(v.1);
            let ey_max = u.1.max(v.1);

            // Intersection with interior x-range implies x1 < ex < x2
            if ex > x1 && ex < x2 {
                // Check if y-intervals overlap strictly
                // Rect Y: [y1, y2]. Edge Y: [ey_min, ey_max]
                let overlap_start = y1.max(ey_min);
                let overlap_end = y2.min(ey_max);
                if overlap_start < overlap_end {
                    return false;
                }
            }
        } else {
            // Horizontal edge at ey
            let ey = u.1;
            let ex_min = u.0.min(v.0);
            let ex_max = u.0.max(v.0);

            if ey > y1 && ey < y2 {
                let overlap_start = x1.max(ex_min);
                let overlap_end = x2.min(ex_max);
                if overlap_start < overlap_end {
                    return false;
                }
            }
        }
    }

    true
}

fn is_point_in_poly(x: u64, y: u64, poly: &[Point]) -> bool {
    // x and y are passed as DOUBLED coordinates (2*real_x, 2*real_y)

    let len = poly.len();

    // 1. Check exact boundary
    for i in 0..len {
        let u = poly[i];
        let v = poly[(i + 1) % len];

        // Edge coordinates must be doubled to compare with x, y
        let u0_2 = u.0 as u64 * 2;
        let u1_2 = u.1 as u64 * 2;
        let v0_2 = v.0 as u64 * 2;
        let v1_2 = v.1 as u64 * 2;

        // Vertical Segment
        if u.0 == v.0 {
            if u0_2 == x {
                let min_y = u1_2.min(v1_2);
                let max_y = u1_2.max(v1_2);
                if y >= min_y && y <= max_y {
                    return true;
                }
            }
        } else {
            // Horizontal Segment
            if u1_2 == y {
                let min_x = u0_2.min(v0_2);
                let max_x = u0_2.max(v0_2);
                if x >= min_x && x <= max_x {
                    return true;
                }
            }
        }
    }

    // 2. Ray casting (Odd-Even rule)
    let mut intersections = 0;
    for i in 0..len {
        let u = poly[i];
        let v = poly[(i + 1) % len];

        if u.0 == v.0 {
            // Vertical edge
            let min_y = (u.1.min(v.1) as u64) * 2;
            let max_y = (u.1.max(v.1) as u64) * 2;
            let ex = (u.0 as u64) * 2;

            // Ray at y. Point slightly above check logic: include start, exclude end
            if y >= min_y && y < max_y && ex > x {
                intersections += 1;
            }
        }
    }

    intersections % 2 == 1
}
