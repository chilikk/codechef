use std::io::BufRead;
use std::ops::{Mul, SubAssign};

struct StdinReader {
    stdin: std::io::Stdin,
}

impl StdinReader {
    fn new() -> Self {
        StdinReader{
            stdin: std::io::stdin(),
        }
    }
    fn line(&self) -> String {
        let lock = self.stdin.lock();
        let mut lines = lock.lines();
        lines.next().unwrap().unwrap()
    }
    fn skip_line(&self) {
        let lock = self.stdin.lock();
        let mut lines = lock.lines();
        lines.next();
    }
    fn usize(&self) -> usize {
        self.line().parse::<usize>().unwrap()
    }
    fn vec_usize(&self) -> Vec<usize> {
        self.line().split(' ').map(|s| s.parse::<usize>().unwrap()).collect()
    }
}

#[derive(Clone,Debug)]
struct Polynom(Vec<i32>);

impl Mul for Polynom {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let mut p: Polynom = Polynom(vec![0;self.0.len()+other.0.len()-1]);
        for (i, c1) in self.0.iter().enumerate() {
            for (j, c2) in other.0.iter().enumerate() {
                p.0[i+j] += c1*c2
            }
        }
        p
    }
}

impl SubAssign for Polynom {
    fn sub_assign(&mut self, other: Polynom) {
        self.0.resize(self.0.len().max(other.0.len()), 0);
        for (i, c) in other.0.iter().enumerate() {
            self.0[i] -= c
        }
    }
}

#[derive(Clone,Debug)]
struct Equation(Vec<f64>);

impl Equation {
    fn from(p: Polynom) -> Self {
        let mut eq = vec![0f64;p.0.len()];
        eq[0] = p.0[0] as f64 - 0.5;
        for i in 1..p.0.len() {
            eq[i] = p.0[i] as f64;
        }
        Equation(eq)
    }

    fn solve_0_1(&self) -> f64 {
        let derivative = self.derivative();
        let mut dist = 1.0;
        let range = 10;
        let mut initial_guess = 0.5;
        for i in 0..range {
            let f = i as f64/range as f64;
            let value = self.eval(f);
            if value.abs() < dist {
                initial_guess = f;
                dist = value.abs();
            }
        }
        let mut xprev = initial_guess;
        let mut control_prev = (xprev * 100000f64).round() as i32;
        loop {
            let value = self.eval(xprev);
            if value == 0f64 {
                break
            }
            let derivative_value = derivative.eval(xprev);
            let xnew = xprev - value/derivative_value;
            let control_new = (xnew * 100000f64).round() as i32;
            if control_prev < 0 && control_new < control_prev || control_prev > 100000 && control_new > control_prev {
                xprev = 1.0/0.0;
                break
            } else if control_new == control_prev {
                break
            }
            xprev = xnew;
            control_prev = control_new;
        }
        xprev
    }

    fn derivative(&self) -> Self {
        let mut eq = vec![0f64;self.0.len()-1];
        for (i, dc) in eq.iter_mut().enumerate() {
            *dc = self.0[i+1]*(i+1) as f64
        }
        Equation(eq)
    }

    fn eval(&self, x: f64) -> f64 {
        let mut acc = 0f64;
        for c in self.0.iter().rev() {
            acc *= x;
            acc += c;
        }
        acc
    }
}

fn main() -> std::io::Result<()> {
    let identity_polynom: Polynom = Polynom(Vec::from([0,1]));
    let stdin = StdinReader::new();
    let n = stdin.usize();
    stdin.skip_line();
    for _ in 0..n {
        let ninp = stdin.usize();
        let mut inputs = vec![identity_polynom.clone();ninp as usize];
        for inp in 0..ninp as usize {
            match (stdin.vec_usize())[..] {
                [0] => (),
                [1, i, j] => {
                    let mut p0i = Polynom(vec![0;inputs[i-1].0.len()]);
                    p0i.0[0] = 1;
                    p0i -= inputs[i-1].clone();
                    let mut p0j = Polynom(vec![0;inputs[j-1].0.len()]);
                    p0j.0[0] = 1;
                    p0j -= inputs[j-1].clone();
                    let p0ij = p0i * p0j;
                    inputs[inp] = Polynom(vec![0;p0ij.0.len()]);
                    inputs[inp].0[0] = 1;
                    inputs[inp] -= p0ij;
                },
                [2, i, j] => {
                    inputs[inp] = inputs[i-1].clone() * inputs[j-1].clone()
                }
                _ => panic!("bad input"),
            }
        }
        stdin.skip_line();
        let solution = Equation::from(inputs[inputs.len()-1].clone()).solve_0_1();
        println!("{:.5}", solution)
    }
    Ok(())
}
