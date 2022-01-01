use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum PodLocation {
    Room { index: i8, depth: i8 },
    Corridor { index: i8 },
}

impl PodLocation {
    fn compress(self) -> CompressedLocation {
        CompressedLocation(match self {
            Self::Room { index, depth } => index * 4 + depth,
            Self::Corridor { index } => -1 - index,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct CompressedLocation(i8);

impl CompressedLocation {
    fn decompress(self) -> PodLocation {
        if self.0 < 0 {
            PodLocation::Corridor { index: -1 - self.0 }
        } else {
            PodLocation::Room {
                index: self.0 / 4,
                depth: self.0 % 4,
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State {
    energy: i32,
    pods: [CompressedLocation; 16],
}

fn corridor_for_room(index: i8) -> i8 {
    index * 2 + 2
}
fn room_for_pod(pod: usize) -> i8 {
    pod as i8 / 4
}

const POD_ENERGY: [i32; 16] = [
    1, 1, 1, 1, 10, 10, 10, 10, 100, 100, 100, 100, 1000, 1000, 1000, 1000,
];
const CORRIDOR_LEN: i8 = 11;

impl State {
    fn display(&self) {
        println!("#############");
        print!("#");
        for i in 0..11 {
            if let Some(pod) = self.pod_at_location(PodLocation::Corridor { index: i }) {
                print!(
                    "{}",
                    char::from_digit(room_for_pod(pod) as u32 + 10, 16).unwrap()
                );
            } else {
                print!(".");
            }
        }
        println!("#");
        for depth in 0..4 {
            if depth == 0 {
                print!("###");
            } else {
                print!("  #")
            }
            for room in 0..4 {
                if let Some(pod) = self.pod_at_location(PodLocation::Room { index: room, depth }) {
                    print!(
                        "{}#",
                        char::from_digit(room_for_pod(pod) as u32 + 10, 16).unwrap()
                    );
                } else {
                    print!(".#");
                }
            }
            if depth == 0 {
                println!("##");
            } else {
                println!("  ")
            }
        }
        println!("  #########  ");
    }

    fn estimated_cost(&self) -> i32 {
        self.pods
            .iter()
            .enumerate()
            .map(|(pod, &loc)| {
                let desired_room = room_for_pod(pod);
                POD_ENERGY[pod]
                    * (match loc.decompress() {
                        PodLocation::Room { index, .. } if index == desired_room => 0,
                        PodLocation::Room { index, depth } => {
                            (index - desired_room).abs() * 2 + depth + 2
                        }
                        PodLocation::Corridor { index } => {
                            (corridor_for_room(desired_room) - index).abs() + 1
                        }
                    } as i32)
            })
            .sum()
    }
    fn cost(&self) -> i32 {
        self.energy + self.estimated_cost()
    }
    fn is_location_free(&self, loc: PodLocation) -> bool {
        let loc = loc.compress();
        self.pods.iter().find(|&&x| x == loc).is_none()
    }
    fn pod_at_location(&self, loc: PodLocation) -> Option<usize> {
        let loc = loc.compress();
        self.pods
            .iter()
            .enumerate()
            .find(|&(_, &x)| x == loc)
            .map(|x| x.0)
    }
    fn possible_next_states(&self) -> Vec<State> {
        let mut res = Vec::new();
        'pod: for (pod, &loc) in self.pods.iter().enumerate() {
            match loc.decompress() {
                PodLocation::Room { index, depth } => {
                    let mut all_correct = true;
                    for other_depth in depth..4 {
                        let other_pod = self
                            .pod_at_location(PodLocation::Room {
                                index,
                                depth: other_depth,
                            })
                            .unwrap();
                        if room_for_pod(other_pod) != index {
                            all_correct = false;
                            break;
                        }
                    }
                    if all_correct {
                        continue;
                    }
                    for other_depth in 0..depth {
                        if !self.is_location_free(PodLocation::Room {
                            index,
                            depth: other_depth,
                        }) {
                            continue 'pod;
                        }
                    }
                    let start_index = corridor_for_room(index);
                    if !self.is_location_free(PodLocation::Corridor { index: start_index }) {
                        continue;
                    }
                    let mut corridor_state = *self;
                    corridor_state.pods[pod] =
                        PodLocation::Corridor { index: start_index }.compress();
                    corridor_state.energy += POD_ENERGY[pod] * (depth as i32 + 1);
                    for i in (0..start_index).rev() {
                        if i != 0 && i % 2 == 0 {
                            continue;
                        }
                        let new_loc = PodLocation::Corridor { index: i };
                        if !self.is_location_free(new_loc) {
                            break;
                        }
                        let mut new_state = corridor_state;
                        new_state.pods[pod] = new_loc.compress();
                        new_state.energy += POD_ENERGY[pod] * ((start_index - i) as i32);
                        res.push(new_state);
                    }
                    for i in start_index + 1..CORRIDOR_LEN {
                        if i != 10 && i % 2 == 0 {
                            continue;
                        }
                        let new_loc = PodLocation::Corridor { index: i };
                        if !self.is_location_free(new_loc) {
                            break;
                        }
                        let mut new_state = corridor_state;
                        new_state.pods[pod] = new_loc.compress();
                        new_state.energy += POD_ENERGY[pod] * ((i - start_index) as i32);
                        res.push(new_state);
                    }
                }
                PodLocation::Corridor { index } => {
                    let desired_room = room_for_pod(pod);
                    let desired_index = corridor_for_room(desired_room);
                    if desired_index < index {
                        for i in (desired_index..index).rev() {
                            if !self.is_location_free(PodLocation::Corridor { index: i }) {
                                continue 'pod;
                            }
                        }
                    } else if desired_index > index {
                        for i in index + 1..desired_index + 1 {
                            if !self.is_location_free(PodLocation::Corridor { index: i }) {
                                continue 'pod;
                            }
                        }
                    }
                    for other_depth in (0..4).rev() {
                        if let Some(other_pod) = self.pod_at_location(PodLocation::Room {
                            index: desired_room,
                            depth: other_depth,
                        }) {
                            if room_for_pod(other_pod) != desired_room {
                                continue 'pod;
                            }
                        } else {
                            let mut new_state = *self;
                            new_state.pods[pod] = PodLocation::Room {
                                index: desired_room,
                                depth: other_depth,
                            }
                            .compress();
                            new_state.energy += POD_ENERGY[pod]
                                * (((desired_index - index).abs() + other_depth + 1) as i32);
                            res.push(new_state);
                            break;
                        }
                    }
                }
            }
        }
        res
    }
    fn is_done(&self) -> bool {
        self.pods.iter().enumerate().all(|(pod, &loc)| {
            if let PodLocation::Room { index, .. } = loc.decompress() {
                if index == room_for_pod(pod) {
                    return true;
                }
            }
            false
        })
    }
}

fn main() {
    let mut backrefs = HashMap::new();
    let mut visited_states = HashSet::new();
    let mut queue = BinaryHeap::new();
    let initial_state = State {
        energy: 0,
        pods: [
            // As
            PodLocation::Room { index: 1, depth: 0 }.compress(),
            PodLocation::Room { index: 2, depth: 2 }.compress(),
            PodLocation::Room { index: 3, depth: 1 }.compress(),
            PodLocation::Room { index: 1, depth: 3 }.compress(),
            // Bs
            PodLocation::Room { index: 2, depth: 3 }.compress(),
            PodLocation::Room { index: 1, depth: 2 }.compress(),
            PodLocation::Room { index: 3, depth: 3 }.compress(),
            PodLocation::Room { index: 2, depth: 1 }.compress(),
            // Cs
            PodLocation::Room { index: 2, depth: 0 }.compress(),
            PodLocation::Room { index: 1, depth: 1 }.compress(),
            PodLocation::Room { index: 3, depth: 0 }.compress(),
            PodLocation::Room { index: 3, depth: 2 }.compress(),
            // Ds
            PodLocation::Room { index: 0, depth: 1 }.compress(),
            PodLocation::Room { index: 0, depth: 2 }.compress(),
            PodLocation::Room { index: 0, depth: 0 }.compress(),
            PodLocation::Room { index: 0, depth: 3 }.compress(),
        ],
    };
    queue.push((Reverse(initial_state.cost()), initial_state, None));

    let mut steps = 0;
    while let Some((_, state, prev_state)) = queue.pop() {
        steps += 1;
        if visited_states.insert(state.pods) {
            if let Some(prev_state) = prev_state {
                backrefs.insert(state.pods, prev_state);
            }
            if state.is_done() {
                let mut cur_pods = state.pods;
                while let Some(prev) = backrefs.remove(&cur_pods) {
                    cur_pods = prev;
                    State {
                        pods: cur_pods,
                        energy: 0,
                    }
                    .display();
                }
                println!("{}", steps);
                println!("{}", state.energy);
                break;
            }
            for next_state in state.possible_next_states() {
                queue.push((Reverse(next_state.cost()), next_state, Some(state.pods)));
            }
        }
    }
}
