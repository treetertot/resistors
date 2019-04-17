fn main() {
    let mut out = Vec::new();
    let r = Parallel(vec!(Series(vec!(Real(10.0), Real(10.0))), Real(10.0)));
    info(9.0, 9.0/resistance(&r), &mut out, &r);
    for i in out {
        println!("{} {}", i.0, i.1)
    }
}

enum Resistor {
    Series(Vec<Resistor>),
    Parallel(Vec<Resistor>),
    Real(f64),
}
use Resistor::*;

fn resistance(r: &Resistor) -> f64 {
    match r {
        Real(ohms) => *ohms,
        Series(inside) => {
            let mut sum = 0.0;
            for res in inside.iter() {
                sum += resistance(res);
            }
            sum
        },
        Parallel(inside) => {
            let mut sum = 0.0;
            for res in inside.iter() {
                sum += 1.0/resistance(res);
            }
            1.0/sum
        }
    }
}

fn info(v: f64, i: f64, out: &mut Vec<(f64, f64)>, res: &Resistor) {
    match res {
        Real(_ohms) => out.push((v, i)),
        Series(inside) => for res in inside {
            info(i * resistance(res), i, out, res)
        },
        Parallel(inside) => for res in inside {
            info(v, v/resistance(res), out, res)
        }
    }
}