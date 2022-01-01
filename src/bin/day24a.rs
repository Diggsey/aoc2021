use std::{fmt::Display, rc::Rc};

const INPUT: &str = include_str!("../../inputs/day24.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Literal(i64),
    Input(usize),
    Add(Rc<Expr>, Rc<Expr>),
    Mul(Rc<Expr>, Rc<Expr>),
    Div(Rc<Expr>, Rc<Expr>),
    Mod(Rc<Expr>, Rc<Expr>),
    Eql(Rc<Expr>, Rc<Expr>),
}

impl Expr {
    fn range(&self, bindings: &[i64]) -> (i64, i64) {
        match self {
            &Expr::Literal(x) => (x, x),
            &Expr::Input(i) if i < bindings.len() => (bindings[i], bindings[i]),
            Expr::Input(_) => (1, 9),
            Expr::Add(x, y) => {
                let (xr, yr) = (x.range(bindings), y.range(bindings));
                (xr.0 + yr.0, xr.1 + yr.1)
            }
            Expr::Mul(x, y) => {
                let (xr, yr) = (x.range(bindings), y.range(bindings));
                let mut res_min = (xr.0 * yr.0)
                    .min(xr.1 * yr.0)
                    .min(xr.0 * yr.1)
                    .min(xr.1 * yr.1);
                if xr.0 < 0 && xr.1 > 0 || yr.0 < 0 && yr.1 > 0 {
                    res_min = res_min.min(0);
                }
                let mut res_max = (xr.0 * yr.0)
                    .max(xr.1 * yr.0)
                    .max(xr.0 * yr.1)
                    .max(xr.1 * yr.1);
                if xr.0 < 0 && xr.1 > 0 || yr.0 < 0 && yr.1 > 0 {
                    res_max = res_max.max(0);
                }
                (res_min, res_max)
            }
            Expr::Div(x, y) => {
                let (xr, mut yr) = (x.range(bindings), y.range(bindings));
                if yr.0 == 0 {
                    yr.0 += 1;
                }
                if yr.1 == 0 {
                    yr.1 -= 1;
                }
                let mut res_min = (xr.0 / yr.0)
                    .min(xr.1 / yr.0)
                    .min(xr.0 / yr.1)
                    .min(xr.1 / yr.1);
                let mut res_max = (xr.0 / yr.0)
                    .max(xr.1 / yr.0)
                    .max(xr.0 / yr.1)
                    .max(xr.1 / yr.1);
                if xr.0 < 0 && xr.1 > 0 {
                    res_min = res_min.min(0);
                    res_max = res_max.min(0);
                }
                if yr.0 < 0 && yr.1 > 0 {
                    res_min = res_min.min(xr.0).min(-xr.1);
                    res_max = res_max.max(xr.1).max(-xr.0);
                }
                (res_min, res_max)
            }
            Expr::Mod(x, y) => {
                let (xr, yr) = (x.range(bindings), y.range(bindings));
                if xr.0 == xr.1 && yr.0 == yr.1 {
                    return (xr.0 % yr.0, xr.0 % yr.0);
                }
                (0, xr.1.min(yr.1 - 1))
            }
            Expr::Eql(x, y) => {
                let (xr, yr) = (x.range(bindings), y.range(bindings));
                if xr.1 < yr.0 || yr.1 < xr.0 {
                    (0, 0)
                } else if xr.0 == xr.1 && yr.0 == yr.1 {
                    (1, 1)
                } else {
                    (0, 1)
                }
            }
        }
    }
    fn simplify_div_mod(self: &Rc<Self>, divisor: i64, bindings: &[i64]) -> (Rc<Self>, Rc<Self>) {
        match &**self {
            Expr::Literal(x) => (
                Rc::new(Expr::Literal(x / divisor)),
                Rc::new(Expr::Literal(x % divisor)),
            ),
            Expr::Add(x, y) => {
                let (sx, sy) = (
                    x.simplify_div_mod(divisor, bindings),
                    y.simplify_div_mod(divisor, bindings),
                );
                (
                    Rc::new(Expr::Add(sx.0, sy.0)).simplify(15, bindings),
                    Rc::new(Expr::Add(sx.1, sy.1)).simplify(15, bindings),
                )
            }
            Expr::Mul(x, y) => {
                if divisor > self.range(bindings).1 {
                    return (Rc::new(Expr::Literal(0)), self.clone());
                }
                let (sx, sy) = (
                    x.simplify_div_mod(divisor, bindings),
                    y.simplify_div_mod(divisor, bindings),
                );
                (
                    Rc::new(Expr::Add(
                        Rc::new(Expr::Mul(
                            Rc::new(Expr::Mul(sx.0.clone(), sy.0.clone())),
                            Rc::new(Expr::Literal(divisor)),
                        )),
                        Rc::new(Expr::Add(
                            Rc::new(Expr::Mul(sx.0, sy.1.clone())),
                            Rc::new(Expr::Mul(sx.1.clone(), sy.0)),
                        )),
                    ))
                    .simplify(15, bindings),
                    Rc::new(Expr::Mul(sx.1, sy.1)).simplify(15, bindings),
                )
            }
            _ => (Rc::new(Expr::Literal(0)), self.clone()),
        }
    }
    fn simplify(self: &Rc<Self>, depth: usize, bindings: &[i64]) -> Rc<Self> {
        if depth == 0 {
            return self.clone();
        }
        let range = self.range(&[]);
        if range.0 == range.1 {
            return Rc::new(Expr::Literal(range.0));
        }
        match &**self {
            &Expr::Input(i) if i < bindings.len() => Rc::new(Expr::Literal(bindings[i])),
            Expr::Add(a, b) => {
                let a2 = a.simplify(depth - 1, bindings);
                let b2 = b.simplify(depth - 1, bindings);
                match (&*a2, &*b2) {
                    (Expr::Literal(0), _) => b2,
                    (_, Expr::Literal(0)) => a2,
                    (Expr::Literal(av), Expr::Literal(bv)) => Rc::new(Expr::Literal(av + bv)),
                    (Expr::Add(aa, ab), Expr::Literal(bv)) => match (&**aa, &**ab) {
                        (_, Expr::Literal(abv)) => {
                            Rc::new(Expr::Add(aa.clone(), Rc::new(Expr::Literal(abv + bv))))
                        }
                        (Expr::Literal(aav), _) => {
                            Rc::new(Expr::Add(ab.clone(), Rc::new(Expr::Literal(aav + bv))))
                        }
                        _ => {
                            if Rc::ptr_eq(&a, &a2) && Rc::ptr_eq(&b, &b2) {
                                self.clone()
                            } else {
                                Rc::new(Expr::Add(a2, b2))
                            }
                        }
                    },
                    _ => {
                        if Rc::ptr_eq(&a, &a2) && Rc::ptr_eq(&b, &b2) {
                            self.clone()
                        } else {
                            Rc::new(Expr::Add(a2, b2))
                        }
                    }
                }
            }
            Expr::Mul(a, b) => {
                let a2 = a.simplify(depth - 1, bindings);
                let b2 = b.simplify(depth - 1, bindings);
                match (&*a2, &*b2) {
                    (Expr::Literal(0), _) | (_, Expr::Literal(0)) => Rc::new(Expr::Literal(0)),
                    (Expr::Literal(1), _) => b2,
                    (_, Expr::Literal(1)) => a2,
                    (Expr::Literal(av), Expr::Literal(bv)) => Rc::new(Expr::Literal(av * bv)),
                    // (Expr::Add(aa, ab), _) => Rc::new(Expr::Add(
                    //     Rc::new(Expr::Mul(aa.clone(), b2.clone())),
                    //     Rc::new(Expr::Mul(ab.clone(), b2)),
                    // )),
                    // (_, Expr::Add(ba, bb)) => Rc::new(Expr::Add(
                    //     Rc::new(Expr::Mul(ba.clone(), a2.clone())),
                    //     Rc::new(Expr::Mul(bb.clone(), a2)),
                    // )),
                    (Expr::Mul(aa, ab), Expr::Literal(bv)) => match (&**aa, &**ab) {
                        (_, Expr::Literal(abv)) => {
                            Rc::new(Expr::Mul(aa.clone(), Rc::new(Expr::Literal(abv * bv))))
                        }
                        (Expr::Literal(aav), _) => {
                            Rc::new(Expr::Mul(ab.clone(), Rc::new(Expr::Literal(aav * bv))))
                        }
                        _ => {
                            if Rc::ptr_eq(&a, &a2) && Rc::ptr_eq(&b, &b2) {
                                self.clone()
                            } else {
                                Rc::new(Expr::Mul(a2, b2))
                            }
                        }
                    },
                    _ => {
                        if Rc::ptr_eq(&a, &a2) && Rc::ptr_eq(&b, &b2) {
                            self.clone()
                        } else {
                            Rc::new(Expr::Mul(a2, b2))
                        }
                    }
                }
            }
            Expr::Div(a, b) => {
                let a2 = a.simplify(depth - 1, bindings);
                let b2 = b.simplify(depth - 1, bindings);
                match (&*a2, &*b2) {
                    (Expr::Literal(0), _) => Rc::new(Expr::Literal(0)),
                    (_, Expr::Literal(1)) => a2,
                    (Expr::Literal(av), Expr::Literal(bv)) => Rc::new(Expr::Literal(av / bv)),
                    (_, &Expr::Literal(bv)) => {
                        let div_mod = a2.simplify_div_mod(bv, bindings);
                        if let Expr::Literal(0) = &*div_mod.0 {
                            Rc::new(Expr::Div(div_mod.1, Rc::new(Expr::Literal(bv))))
                        } else {
                            Rc::new(Expr::Add(
                                div_mod.0,
                                Rc::new(Expr::Div(div_mod.1, Rc::new(Expr::Literal(bv)))),
                            ))
                        }
                    }
                    _ => {
                        if Rc::ptr_eq(&a, &a2) && Rc::ptr_eq(&b, &b2) {
                            self.clone()
                        } else {
                            Rc::new(Expr::Div(a2, b2))
                        }
                    }
                }
            }
            Expr::Mod(a, b) => {
                let a2 = a.simplify(depth - 1, bindings);
                let b2 = b.simplify(depth - 1, bindings);
                match (&*a2, &*b2) {
                    (Expr::Literal(0), _) | (_, Expr::Literal(1)) => Rc::new(Expr::Literal(0)),
                    (Expr::Literal(1), _) => Rc::new(Expr::Literal(1)),
                    (Expr::Literal(av), Expr::Literal(bv)) => Rc::new(Expr::Literal(av % bv)),
                    (_, &Expr::Literal(bv)) if a2.range(&[]).1 < bv => a2,
                    (_, &Expr::Literal(bv)) => {
                        let remainder = a2.simplify_div_mod(bv, bindings).1;
                        let divisor = Rc::new(Expr::Literal(bv));
                        if remainder.range(bindings).1 >= bv {
                            match &*remainder {
                                Expr::Add(aa, bb) => Rc::new(Expr::Add(
                                    Rc::new(Expr::Mod(aa.clone(), divisor.clone())),
                                    Rc::new(Expr::Mod(bb.clone(), divisor)),
                                )),
                                Expr::Mul(aa, bb) => Rc::new(Expr::Mul(
                                    Rc::new(Expr::Mod(aa.clone(), divisor.clone())),
                                    Rc::new(Expr::Mod(bb.clone(), divisor)),
                                )),
                                _ => Rc::new(Expr::Mod(remainder, divisor)),
                            }
                        } else {
                            remainder
                        }
                    }
                    _ => {
                        if Rc::ptr_eq(&a, &a2) && Rc::ptr_eq(&b, &b2) {
                            self.clone()
                        } else {
                            Rc::new(Expr::Mod(a2, b2))
                        }
                    }
                }
            }
            Expr::Eql(a, b) => {
                let a2 = a.simplify(depth - 1, bindings);
                let b2 = b.simplify(depth - 1, bindings);

                match (&*a2, &*b2) {
                    (Expr::Literal(av), Expr::Literal(bv)) => {
                        Rc::new(Expr::Literal(if av == bv { 1 } else { 0 }))
                    }
                    _ => {
                        let arange = a2.range(&[]);
                        let brange = b2.range(&[]);
                        if a2 == b2 {
                            Rc::new(Expr::Literal(1))
                        } else if arange.0 > brange.1 || arange.1 < brange.0 {
                            Rc::new(Expr::Literal(0))
                        } else if Rc::ptr_eq(&a, &a2) && Rc::ptr_eq(&b, &b2) {
                            self.clone()
                        } else {
                            Rc::new(Expr::Eql(a2, b2))
                        }
                    }
                }
            }
            _ => self.clone(),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(x) => x.fmt(f),
            Expr::Input(i) => write!(f, "inp{}", i),
            Expr::Add(x, y) => write!(f, "({} + {})", x, y),
            Expr::Mul(x, y) => write!(f, "({} * {})", x, y),
            Expr::Div(x, y) => write!(f, "({} / {})", x, y),
            Expr::Mod(x, y) => write!(f, "({} % {})", x, y),
            Expr::Eql(x, y) => write!(f, "({} = {})", x, y),
        }
    }
}

fn lvalue(s: &str) -> usize {
    match s {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => panic!("Unknown variable: {}", s),
    }
}

fn rvalue(s: &str, registers: &[Rc<Expr>]) -> Rc<Expr> {
    match s {
        "w" => registers[0].clone(),
        "x" => registers[1].clone(),
        "y" => registers[2].clone(),
        "z" => registers[3].clone(),
        _ => Rc::new(Expr::Literal(s.parse().unwrap())),
    }
}

fn main() {
    let mut input_index = 0;
    let zero = Rc::new(Expr::Literal(0));
    let mut registers: [Rc<Expr>; 4] = [zero.clone(), zero.clone(), zero.clone(), zero.clone()];
    for line in INPUT.lines() {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        match parts.as_slice() {
            ["inp", a] => {
                registers[lvalue(a)] = Rc::new(Expr::Input(input_index));
                input_index += 1;
            }
            ["add", a, b] => {
                let av = rvalue(a, &registers);
                let bv = rvalue(b, &registers);
                registers[lvalue(a)] = Rc::new(Expr::Add(av, bv)).simplify(150, &[]);
            }
            ["mul", a, b] => {
                let av = rvalue(a, &registers);
                let bv = rvalue(b, &registers);
                registers[lvalue(a)] = Rc::new(Expr::Mul(av, bv)).simplify(150, &[]);
            }
            ["div", a, b] => {
                let av = rvalue(a, &registers);
                let bv = rvalue(b, &registers);
                registers[lvalue(a)] = Rc::new(Expr::Div(av, bv)).simplify(150, &[]);
            }
            ["mod", a, b] => {
                let av = rvalue(a, &registers);
                let bv = rvalue(b, &registers);
                registers[lvalue(a)] = Rc::new(Expr::Mod(av, bv)).simplify(150, &[]);
            }
            ["eql", a, b] => {
                let av = rvalue(a, &registers);
                let bv = rvalue(b, &registers);
                registers[lvalue(a)] = Rc::new(Expr::Eql(av, bv)).simplify(150, &[]);
            }
            _ => panic!("Unknown: {:?}", parts),
        }
    }
    let res = rvalue("z", &registers).simplify(1000, &[]);
    println!("{}", res);

    for item0 in (1..10).rev() {
        let range = res.range(&[item0]);
        if range.0 > 0 || range.1 < 0 {
            continue;
        }
        for item1 in (1..10).rev() {
            let range = res.range(&[item0, item1]);
            if range.0 > 0 || range.1 < 0 {
                continue;
            }
            for item2 in (1..10).rev() {
                let range = res.range(&[item0, item1, item2]);
                if range.0 > 0 || range.1 < 0 {
                    continue;
                }
                for item3 in (1..10).rev() {
                    let range = res.range(&[item0, item1, item2, item3]);
                    if range.0 > 0 || range.1 < 0 {
                        continue;
                    }
                    for item4 in (1..10).rev() {
                        //println!("{}{}{}{}{}", item0, item1, item2, item3, item4);
                        let res = res.simplify(100, &[item0, item1, item2, item3, item4]);
                        let range = res.range(&[item0, item1, item2, item3, item4]);
                        if range.0 > 0 || range.1 < 0 {
                            continue;
                        }
                        for item5 in (1..10).rev() {
                            let range = res.range(&[item0, item1, item2, item3, item4, item5]);
                            if range.0 > 0 || range.1 < 0 {
                                continue;
                            }
                            for item6 in (1..10).rev() {
                                let res = res.simplify(
                                    100,
                                    &[item0, item1, item2, item3, item4, item5, item6],
                                );
                                let range =
                                    res.range(&[item0, item1, item2, item3, item4, item5, item6]);
                                if range.0 > 0 || range.1 < 0 {
                                    continue;
                                }
                                for item7 in (1..10).rev() {
                                    let range = res.range(&[
                                        item0, item1, item2, item3, item4, item5, item6, item7,
                                    ]);
                                    if range.0 > 0 || range.1 < 0 {
                                        continue;
                                    }
                                    for item8 in (1..10).rev() {
                                        let range = res.range(&[
                                            item0, item1, item2, item3, item4, item5, item6, item7,
                                            item8,
                                        ]);
                                        if range.0 > 0 || range.1 < 0 {
                                            continue;
                                        }
                                        for item9 in (1..10).rev() {
                                            let range = res.range(&[
                                                item0, item1, item2, item3, item4, item5, item6,
                                                item7, item8, item9,
                                            ]);
                                            if range.0 > 0 || range.1 < 0 {
                                                continue;
                                            }
                                            for item10 in (1..10).rev() {
                                                let range = res.range(&[
                                                    item0, item1, item2, item3, item4, item5,
                                                    item6, item7, item8, item9, item10,
                                                ]);
                                                if range.0 > 0 || range.1 < 0 {
                                                    continue;
                                                }
                                                for item11 in (1..10).rev() {
                                                    let range = res.range(&[
                                                        item0, item1, item2, item3, item4, item5,
                                                        item6, item7, item8, item9, item10, item11,
                                                    ]);
                                                    if range.0 > 0 || range.1 < 0 {
                                                        continue;
                                                    }
                                                    for item12 in (1..10).rev() {
                                                        let range = res.range(&[
                                                            item0, item1, item2, item3, item4,
                                                            item5, item6, item7, item8, item9,
                                                            item10, item11, item12,
                                                        ]);
                                                        if range.0 > 0 || range.1 < 0 {
                                                            continue;
                                                        }
                                                        for item13 in (1..10).rev() {
                                                            let range = res.range(&[
                                                                item0, item1, item2, item3, item4,
                                                                item5, item6, item7, item8, item9,
                                                                item10, item11, item12, item13,
                                                            ]);
                                                            if range.0 > 0 || range.1 < 0 {
                                                                continue;
                                                            }
                                                            println!(
                                                                "{:?}",
                                                                [
                                                                    item0, item1, item2, item3,
                                                                    item4, item5, item6, item7,
                                                                    item8, item9, item10, item11,
                                                                    item12, item13,
                                                                ]
                                                            );
                                                            return;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
