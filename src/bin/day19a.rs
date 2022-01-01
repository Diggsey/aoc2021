use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day19.txt");

#[derive(Debug, Copy, Clone)]
struct Rotation([[i32; 3]; 3]);

impl Rotation {
    const IDENTITY: Rotation = Rotation([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    fn apply(&self, coord: [i32; 3]) -> [i32; 3] {
        [
            self.0[0][0] * coord[0] + self.0[0][1] * coord[1] + self.0[0][2] * coord[2],
            self.0[1][0] * coord[0] + self.0[1][1] * coord[1] + self.0[1][2] * coord[2],
            self.0[2][0] * coord[0] + self.0[2][1] * coord[1] + self.0[2][2] * coord[2],
        ]
    }
}

#[derive(Debug, Copy, Clone)]
struct Translation([i32; 3]);

impl Translation {
    const ZERO: Translation = Translation([0, 0, 0]);
    fn apply(&self, coord: [i32; 3]) -> [i32; 3] {
        [
            self.0[0] + coord[0],
            self.0[1] + coord[1],
            self.0[2] + coord[2],
        ]
    }
    fn offset(a: [i32; 3], b: [i32; 3]) -> Self {
        Self([b[0] - a[0], b[1] - a[1], b[2] - a[2]])
    }
}

#[derive(Debug)]
struct Scanner {
    translation: Translation,
    rotation: Rotation,
}

impl Scanner {
    fn transform(&self, coord: [i32; 3]) -> [i32; 3] {
        self.translation.apply(self.rotation.apply(coord))
    }
}

fn generate_rotations() -> Vec<Rotation> {
    fn elem(perm: usize, signs: usize, index: usize) -> i32 {
        if perm == index {
            if (signs >> index) & 1 != 0 {
                1
            } else {
                -1
            }
        } else {
            0
        }
    }
    // Calculate the determinant of a 3x3 matrix.
    fn det(mat: &[[i32; 3]; 3]) -> i32 {
        mat[0][0] * (mat[1][1] * mat[2][2] - mat[2][1] * mat[1][2])
            + mat[0][1] * (mat[1][2] * mat[2][0] - mat[2][2] * mat[1][0])
            + mat[0][2] * (mat[1][0] * mat[2][1] - mat[1][1] * mat[2][0])
    }
    // We generate rotations by first taking every permutation
    // of the X/Y/Z axis...
    (0..3)
        .permutations(3)
        // And for each permutation, there are 2^3 = 8 ways we can flip
        // the signs, so that each axis can be positive or negative.
        .cartesian_product(0..8)
        .map(|(perm, signs)| {
            [
                [
                    elem(perm[0], signs, 0),
                    elem(perm[0], signs, 1),
                    elem(perm[0], signs, 2),
                ],
                [
                    elem(perm[1], signs, 0),
                    elem(perm[1], signs, 1),
                    elem(perm[1], signs, 2),
                ],
                [
                    elem(perm[2], signs, 0),
                    elem(perm[2], signs, 1),
                    elem(perm[2], signs, 2),
                ],
            ]
        })
        // This gives us every possible rotation or reflection, so
        // filter out the reflections by filtering for matrices where the
        // determinant is 1. Reflections will have a determinant of -1.
        .filter(|mat| det(mat) == 1)
        .map(Rotation)
        .collect()
}

fn test_scanner(candidate: &Scanner, coords: &[[i32; 3]], beacons: &HashSet<[i32; 3]>) -> bool {
    let mut matched_count = 0;
    for &coord in coords {
        let tx_coord = candidate.transform(coord);
        if beacons.contains(&tx_coord) {
            matched_count += 1;
            if matched_count >= 3 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let rotations = generate_rotations();

    let mut beacons = HashSet::new();
    let mut scanners = Vec::new();

    let mut parsed_inputs: Vec<Vec<_>> = INPUT
        .split("\n\n")
        .map(|scanner_input| {
            scanner_input
                .lines()
                .skip(1)
                .map(|line| {
                    let coord: Vec<_> =
                        line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
                    [coord[0], coord[1], coord[2]]
                })
                .collect()
        })
        .collect();

    while !parsed_inputs.is_empty() {
        let mut made_progress = false;
        parsed_inputs.retain(|coords| {
            let scanner = if scanners.is_empty() {
                Scanner {
                    translation: Translation::ZERO,
                    rotation: Rotation::IDENTITY,
                }
            } else {
                'found: loop {
                    // Only look at the first 7 points, because that's enough...
                    for &coord in &coords[0..7] {
                        for &rotation in &rotations {
                            let rotated_coord = rotation.apply(coord);
                            for &beacon in &beacons {
                                let candidate = Scanner {
                                    translation: Translation::offset(rotated_coord, beacon),
                                    rotation,
                                };
                                if test_scanner(&candidate, &coords, &beacons) {
                                    break 'found candidate;
                                }
                            }
                        }
                    }
                    println!("skip");
                    return true;
                }
            };

            println!("{:?}", scanner);
            for &coord in coords {
                let tx_coord = scanner.transform(coord);
                beacons.insert(tx_coord);
            }

            scanners.push(scanner);
            made_progress = true;
            false
        });

        if !made_progress {
            panic!("Beacons could not be matched!");
        }
    }

    println!("{:?}", beacons);
    println!("{}", beacons.len());
}
