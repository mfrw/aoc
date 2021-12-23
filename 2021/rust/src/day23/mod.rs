use crate::utils;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

pub fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;
    println!("Day23/Part1 Sol: {}", part1(&input));
    println!("Day23/Part2 Sol: {}", part2(input));
    Ok(())
}

use Amphipod::*;
type Hallway = [Option<Amphipod>; 11];
type DestRoom = [Option<Amphipod>; 4];
type DestRooms = [DestRoom; 4];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    hallway: Hallway,
    dest_rooms: DestRooms,
}

fn dest_room_to_hallway(dest_room_idx: usize) -> usize {
    match dest_room_idx {
        0 => 2,
        1 => 4,
        2 => 6,
        3 => 8,
        _ => panic!(),
    }
}

fn home_of(dest_room_idx: usize) -> Amphipod {
    match dest_room_idx {
        0 => Amber,
        1 => Bronze,
        2 => Copper,
        3 => Desert,
        _ => panic!(),
    }
}

fn home_for(a: Amphipod) -> usize {
    match a {
        Amber => 0,
        Bronze => 1,
        Copper => 2,
        Desert => 3,
    }
}

fn move_energy(a: Amphipod) -> usize {
    match a {
        Amber => 1,
        Bronze => 10,
        Copper => 100,
        Desert => 1000,
    }
}

fn parse_amphipod(c: char) -> Amphipod {
    match c {
        'A' => Amber,
        'B' => Bronze,
        'C' => Copper,
        'D' => Desert,
        _ => panic!(),
    }
}

#[derive(Debug, Clone)]
enum Pos {
    Hallway(usize),
    DestRoom(usize, usize),
}

#[derive(Debug)]
struct Move(Amphipod, Pos, Pos, usize);

fn is_unobstructed(hallway: &Hallway, pos: usize, a: usize, b: usize) -> bool {
    let from = usize::min(a, b);
    let to = usize::max(a, b);
    (from..=to).all(|i| i == pos || hallway[i] == None)
}

fn hallway_dist(a: usize, b: usize) -> usize {
    let min = usize::min(a, b);
    let max = usize::max(a, b);
    max - min
}

fn possible_moves_from_hallway(hallway_idx: usize, state: &State) -> Option<Move> {
    match state.hallway[hallway_idx] {
        Some(hallway_a) => {
            let new_home = home_for(hallway_a);
            let new_home_hallway_idx = dest_room_to_hallway(new_home);
            if is_unobstructed(
                &state.hallway,
                hallway_idx,
                hallway_idx,
                new_home_hallway_idx,
            ) {
                if state.dest_rooms[new_home]
                    .iter()
                    .all(|v| v == &None || v == &Some(hallway_a))
                {
                    let new_home_top = find_open_slot(&state.dest_rooms[new_home]);
                    return Some(Move(
                        hallway_a,
                        Pos::Hallway(hallway_idx),
                        Pos::DestRoom(new_home, new_home_top),
                        (move_inout_cost(new_home_top)
                            + hallway_dist(hallway_idx, new_home_hallway_idx))
                            * move_energy(hallway_a),
                    ));
                }
            }
        }
        None => {}
    }
    None
}

fn can_move_to_hallway(idx: usize) -> bool {
    idx != 2 && idx != 4 && idx != 6 && idx != 8
}

fn find_top(dest_room: &DestRoom) -> Option<(usize, Amphipod)> {
    dest_room
        .iter()
        .enumerate()
        .find(|(_, it)| it.is_some())
        .map(|(idx, a)| (idx, a.unwrap()))
}

fn find_open_slot(dest_room: &DestRoom) -> usize {
    for i in (0..4).rev() {
        if dest_room[i] == None {
            return i;
        }
    }
    panic!();
}

fn move_inout_cost(slot: usize) -> usize {
    slot + 1
}

fn possible_moves_from_room(room_idx: usize, state: &State) -> Option<Vec<Move>> {
    let dest_room = state.dest_rooms[room_idx];
    if let Some((top_slot, top_a)) = find_top(&dest_room) {
        if (top_slot..4).all(|i| dest_room[i] == Some(home_of(room_idx))) {
            return None;
        }
        let mut result: Vec<Move> = vec![];
        let cur_pos = Pos::DestRoom(room_idx, top_slot);
        let new_home = home_for(top_a);
        if new_home != room_idx
            && is_unobstructed(
                &state.hallway,
                99,
                dest_room_to_hallway(room_idx),
                dest_room_to_hallway(new_home),
            )
        {
            if state.dest_rooms[new_home]
                .iter()
                .all(|v| v == &None || v == &Some(top_a))
            {
                let new_home_slot = find_open_slot(&state.dest_rooms[new_home]);
                result.push(Move(
                    top_a,
                    cur_pos.clone(),
                    Pos::DestRoom(new_home, new_home_slot),
                    (move_inout_cost(top_slot)
                        + move_inout_cost(new_home_slot)
                        + hallway_dist(
                            dest_room_to_hallway(room_idx),
                            dest_room_to_hallway(new_home),
                        ))
                        * move_energy(top_a),
                ))
            }
        }

        for hw in 0..11 {
            if can_move_to_hallway(hw) {
                if is_unobstructed(&state.hallway, 99, dest_room_to_hallway(room_idx), hw) {
                    result.push(Move(
                        top_a,
                        cur_pos.clone(),
                        Pos::Hallway(hw),
                        (move_inout_cost(top_slot)
                            + hallway_dist(dest_room_to_hallway(room_idx), hw))
                            * move_energy(top_a),
                    ))
                }
            }
        }

        if result.len() > 0 {
            return Some(result);
        }
    }
    None
}

fn apply_move(m: &Move, state: &State) -> State {
    let mut hallway = state.hallway.clone();
    let mut dest_rooms = state.dest_rooms.clone();
    let Move(a, from, to, _cost) = m;
    match from {
        &Pos::DestRoom(idx, slot) => {
            assert_eq!(&dest_rooms[idx][slot].unwrap(), a);
            dest_rooms[idx][slot] = None;
        }
        &Pos::Hallway(idx) => {
            assert_eq!(&hallway[idx].unwrap(), a);
            hallway[idx] = None;
        }
    }
    match to {
        &Pos::DestRoom(idx, slot) => {
            assert_eq!(dest_rooms[idx][slot], None);
            dest_rooms[idx][slot] = Some(*a);
        }
        &Pos::Hallway(idx) => {
            assert_eq!(hallway[idx], None);
            hallway[idx] = Some(*a);
        }
    }
    State {
        hallway,
        dest_rooms,
    }
}

fn all_possible_moves(state: &State) -> Vec<Move> {
    let dr1_moves = (0..4)
        .filter_map(|dr| possible_moves_from_room(dr, state))
        .flat_map(|v| v);
    let hallway_moves = (0..11usize).filter_map(|room| possible_moves_from_hallway(room, state));
    dr1_moves.chain(hallway_moves).collect()
}

fn is_complete(state: &State) -> bool {
    state.dest_rooms.iter().enumerate().all(|(idx, dr)| {
        let a = home_of(idx);
        dr.iter().all(|da| da == &Some(a))
    })
}

#[allow(unused)]
fn dbg_amphipod(a: Amphipod) -> char {
    match a {
        Amber => 'A',
        Bronze => 'B',
        Copper => 'C',
        Desert => 'D',
    }
}

#[allow(unused)]
fn dbg_opt_amphipod(a: Option<Amphipod>) -> char {
    match a {
        None => '.',
        Some(a) => dbg_amphipod(a),
    }
}

#[allow(unused)]
fn dbg_state(hallway: &Hallway, dest_rooms: &DestRooms) {
    let hallway = hallway
        .iter()
        .map(|p| match p {
            None => '.',
            Some(a) => dbg_amphipod(*a),
        })
        .collect::<String>();
    let (dr_a, dr_b, dr_c, dr_d) = (
        &dest_rooms[0],
        &dest_rooms[1],
        &dest_rooms[2],
        &dest_rooms[3],
    );
    println!(
        r"#############
#{}#
###{}#{}#{}#{}###
  #{}#{}#{}#{}#
  #{}#{}#{}#{}#
  #{}#{}#{}#{}#
  #########",
        hallway,
        dbg_opt_amphipod(dr_a[0]),
        dbg_opt_amphipod(dr_b[0]),
        dbg_opt_amphipod(dr_c[0]),
        dbg_opt_amphipod(dr_d[0]),
        dbg_opt_amphipod(dr_a[1]),
        dbg_opt_amphipod(dr_b[1]),
        dbg_opt_amphipod(dr_c[1]),
        dbg_opt_amphipod(dr_d[1]),
        dbg_opt_amphipod(dr_a[2]),
        dbg_opt_amphipod(dr_b[2]),
        dbg_opt_amphipod(dr_c[2]),
        dbg_opt_amphipod(dr_d[2]),
        dbg_opt_amphipod(dr_a[3]),
        dbg_opt_amphipod(dr_b[3]),
        dbg_opt_amphipod(dr_c[3]),
        dbg_opt_amphipod(dr_d[3]),
    );
    println!();
}

fn make_moves(state: &State, memo: &mut HashMap<State, Option<usize>>) -> Option<usize> {
    if is_complete(state) {
        return Some(0);
    }
    if let Some(previous_result) = memo.get(state) {
        return *previous_result;
    }
    let result = {
        all_possible_moves(state)
            .iter()
            .filter_map(|m| {
                let Move(_, _, _, move_cost) = m;
                let new_state = apply_move(m, state);
                make_moves(&new_state, memo).map(|sub_cost| move_cost + sub_cost)
            })
            .min()
    };
    memo.insert(state.clone(), result);
    result
}

fn part1(input: &[[Option<Amphipod>; 4]; 4]) -> usize {
    let hallway: Hallway = [
        None, None, None, None, None, None, None, None, None, None, None,
    ];
    let state = State {
        hallway,
        dest_rooms: *input,
    };
    make_moves(&state, &mut HashMap::new()).unwrap()
}

fn part2(mut input: [[Option<Amphipod>; 4]; 4]) -> usize {
    let hallway: Hallway = [
        None, None, None, None, None, None, None, None, None, None, None,
    ];
    input[0][3] = input[0][1];
    input[1][3] = input[1][1];
    input[2][3] = input[2][1];
    input[3][3] = input[3][1];
    input[0][1] = Some(Desert);
    input[0][2] = Some(Desert);
    input[1][1] = Some(Copper);
    input[1][2] = Some(Bronze);
    input[2][1] = Some(Bronze);
    input[2][2] = Some(Amber);
    input[3][1] = Some(Amber);
    input[3][2] = Some(Copper);
    let state = State {
        hallway,
        dest_rooms: input,
    };
    make_moves(&state, &mut HashMap::new()).unwrap()
}

fn get_input() -> Result<DestRooms, std::io::Error> {
    let input = utils::get_input("input/day23")?;
    parse_input(&input)
}

fn parse_input(input: &str) -> Result<DestRooms, std::io::Error> {
    let (a1, b1, c1, d1, a2, b2, c2, d2): (char, char, char, char, char, char, char, char) =
        serde_scan::scan!(r"#############
#...........#
###{}#{}#{}#{}###
  #{}#{}#{}#{}#
  #########" <- input)
        .unwrap();

    let v = [
        [
            Some(parse_amphipod(a1)),
            Some(parse_amphipod(a2)),
            Some(Amber),
            Some(Amber),
        ],
        [
            Some(parse_amphipod(b1)),
            Some(parse_amphipod(b2)),
            Some(Bronze),
            Some(Bronze),
        ],
        [
            Some(parse_amphipod(c1)),
            Some(parse_amphipod(c2)),
            Some(Copper),
            Some(Copper),
        ],
        [
            Some(parse_amphipod(d1)),
            Some(parse_amphipod(d2)),
            Some(Desert),
            Some(Desert),
        ],
    ];
    Ok(v)
}
