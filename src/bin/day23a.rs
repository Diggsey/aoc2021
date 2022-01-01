use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum PodLocation {
    Room { index: i32, bottom: bool },
    Corridor { index: i32 },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct State {
    energy: i32,
    pods: [PodLocation; 8],
}

fn corridor_for_room(index: i32) -> i32 {
    index * 2 + 2
}
fn room_for_pod(pod: usize) -> i32 {
    pod as i32 / 2
}

const POD_ENERGY: [i32; 8] = [1, 1, 10, 10, 100, 100, 1000, 1000];
const CORRIDOR_LEN: i32 = 11;

impl State {
    fn estimated_cost(&self) -> i32 {
        self.pods
            .iter()
            .enumerate()
            .map(|(pod, &loc)| {
                let desired_room = room_for_pod(pod);
                POD_ENERGY[pod]
                    * match loc {
                        PodLocation::Room { index, .. } if index == desired_room => 0,
                        PodLocation::Room {
                            index,
                            bottom: true,
                        } => (index - desired_room).abs() * 2 + 3,
                        PodLocation::Room {
                            index,
                            bottom: false,
                        } => (index - desired_room).abs() * 2 + 2,
                        PodLocation::Corridor { index } => {
                            (corridor_for_room(desired_room) - index).abs() + 1
                        }
                    }
            })
            .sum()
    }
    fn cost(&self) -> i32 {
        self.energy + self.estimated_cost()
    }
    fn is_location_free(&self, loc: PodLocation) -> bool {
        self.pods.iter().find(|&&x| x == loc).is_none()
    }
    fn pod_at_location(&self, loc: PodLocation) -> Option<usize> {
        self.pods
            .iter()
            .enumerate()
            .find(|&(_, &x)| x == loc)
            .map(|x| x.0)
    }
    fn possible_next_states(&self) -> Vec<State> {
        let mut res = Vec::new();
        'pod: for (pod, &loc) in self.pods.iter().enumerate() {
            match loc {
                PodLocation::Room { index, bottom } => {
                    if room_for_pod(pod) == index {
                        if bottom {
                            continue;
                        } else if let Some(other_pod) = self.pod_at_location(PodLocation::Room {
                            index,
                            bottom: true,
                        }) {
                            if room_for_pod(other_pod) == index {
                                continue;
                            }
                        }
                    }
                    if bottom
                        && !self.is_location_free(PodLocation::Room {
                            index,
                            bottom: false,
                        })
                    {
                        continue;
                    }
                    let start_index = corridor_for_room(index);
                    if !self.is_location_free(PodLocation::Corridor { index: start_index }) {
                        continue;
                    }
                    let mut corridor_state = *self;
                    corridor_state.pods[pod] = PodLocation::Corridor { index: start_index };
                    corridor_state.energy += POD_ENERGY[pod] * if bottom { 2 } else { 1 };
                    for i in (0..start_index).rev() {
                        let new_loc = PodLocation::Corridor { index: i };
                        if !self.is_location_free(new_loc) {
                            break;
                        }
                        if i != 0 && i % 2 == 0 {
                            continue;
                        }
                        let mut new_state = corridor_state;
                        new_state.pods[pod] = new_loc;
                        new_state.energy += POD_ENERGY[pod] * (start_index - i);
                        res.push(new_state);
                    }
                    for i in start_index + 1..CORRIDOR_LEN {
                        let new_loc = PodLocation::Corridor { index: i };
                        if !self.is_location_free(new_loc) {
                            break;
                        }
                        if i != 10 && i % 2 == 0 {
                            continue;
                        }
                        let mut new_state = corridor_state;
                        new_state.pods[pod] = new_loc;
                        new_state.energy += POD_ENERGY[pod] * (i - start_index);
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
                    if !self.is_location_free(PodLocation::Room {
                        index: desired_room,
                        bottom: false,
                    }) {
                        continue;
                    }
                    let bottom = if let Some(other_pod) = self.pod_at_location(PodLocation::Room {
                        index: desired_room,
                        bottom: true,
                    }) {
                        if room_for_pod(other_pod) != desired_room {
                            continue;
                        }
                        false
                    } else {
                        true
                    };
                    let mut new_state = *self;
                    new_state.pods[pod] = PodLocation::Room {
                        index: desired_room,
                        bottom,
                    };
                    new_state.energy += POD_ENERGY[pod]
                        * ((desired_index - index).abs() + if bottom { 2 } else { 1 });
                    res.push(new_state);
                }
            }
        }
        res
    }
    fn is_done(&self) -> bool {
        self.pods.iter().enumerate().all(|(pod, &loc)| {
            if let PodLocation::Room { index, .. } = loc {
                if index == room_for_pod(pod) {
                    return true;
                }
            }
            false
        })
    }
}

fn main() {
    let mut visited_states = HashSet::<State>::new();
    let mut queue = BinaryHeap::new();
    let initial_state = State {
        energy: 0,
        pods: [
            PodLocation::Room {
                index: 1,
                bottom: true,
            },
            PodLocation::Room {
                index: 1,
                bottom: false,
            },
            PodLocation::Room {
                index: 2,
                bottom: true,
            },
            PodLocation::Room {
                index: 3,
                bottom: true,
            },
            PodLocation::Room {
                index: 2,
                bottom: false,
            },
            PodLocation::Room {
                index: 3,
                bottom: false,
            },
            PodLocation::Room {
                index: 0,
                bottom: true,
            },
            PodLocation::Room {
                index: 0,
                bottom: false,
            },
        ],
    };
    queue.push((Reverse(initial_state.cost()), initial_state));

    let mut prev_cost = 0;
    while let Some((Reverse(cost), state)) = queue.pop() {
        // if (state.pods[3] == PodLocation::Corridor { index: 3 }
        //     && state.pods[4] == PodLocation::Corridor { index: 5 })
        // {
        //     dbg!(&state);
        //     dbg!(&state.possible_next_states());
        // }
        if cost > prev_cost + 100 {
            dbg!(cost);
            prev_cost = cost;
        }
        if state.is_done() {
            println!("{}", state.energy);
            break;
        }
        if visited_states.insert(state) {
            for next_state in state.possible_next_states() {
                queue.push((Reverse(next_state.cost()), next_state));
            }
        }
    }
}
