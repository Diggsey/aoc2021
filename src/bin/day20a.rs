const INPUT: &str = include_str!("../../inputs/day20.txt");

fn sample(image: &[Vec<bool>], x: i32, y: i32, bg: bool) -> bool {
    if x < 0 || y < 0 || x >= image[0].len() as i32 || y >= image.len() as i32 {
        bg
    } else {
        image[y as usize][x as usize]
    }
}

fn apply_rule(bits: [bool; 9], rules: &[bool]) -> bool {
    let mut index = 0;
    for bit in bits {
        index <<= 1;
        if bit {
            index |= 1;
        }
    }
    rules[index]
}

fn enhance_image(image: &[Vec<bool>], rules: &[bool], bg: bool) -> (Vec<Vec<bool>>, bool) {
    (
        (0..image.len() as i32 + 2)
            .map(|y| {
                (0..image[0].len() as i32 + 2)
                    .map(|x| {
                        apply_rule(
                            [
                                sample(image, x - 2, y - 2, bg),
                                sample(image, x - 1, y - 2, bg),
                                sample(image, x, y - 2, bg),
                                sample(image, x - 2, y - 1, bg),
                                sample(image, x - 1, y - 1, bg),
                                sample(image, x, y - 1, bg),
                                sample(image, x - 2, y, bg),
                                sample(image, x - 1, y, bg),
                                sample(image, x, y, bg),
                            ],
                            rules,
                        )
                    })
                    .collect()
            })
            .collect(),
        rules[if bg { 511 } else { 0 }],
    )
}

fn display(image: &[Vec<bool>]) {
    for row in image {
        let mut line = String::new();
        for &b in row {
            line.push(if b { '#' } else { '.' });
        }
        println!("{}", line);
    }
    println!();
}

fn main() {
    let (rules, image) = INPUT.split_once("\n\n").unwrap();
    let rules: Vec<_> = rules.chars().map(|c| c == '#').collect();
    let mut image_bg: (Vec<Vec<_>>, bool) = (
        image
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect(),
        false,
    );

    for _ in 0..2 {
        display(&image_bg.0);
        image_bg = enhance_image(&image_bg.0, &rules, image_bg.1);
    }
    display(&image_bg.0);

    let lit = image_bg.0.iter().flatten().filter(|b| **b).count();

    println!("{}", lit);
}
