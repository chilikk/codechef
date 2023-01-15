use std::io::BufRead;

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
    fn usize(&self) -> usize {
        self.line().parse::<usize>().unwrap()
    }
    fn vec_i32(&self) -> Vec<i32> {
        self.line().split(' ').map(|s| s.parse::<i32>().unwrap()).collect()
    }
}

fn get_max_dist(vec_compact: &Vec<(i32, i32)>, n: usize, min_cand: (i32, i32)) -> i32 {
    let mut max_dist = 0;
    let min_cand_orig = min_cand.0 + min_cand.1;
    for i in 0..n {
        let dista = vec_compact[i].0 + vec_compact[i].1 - min_cand_orig;
        let distb = vec_compact[i+n].0 + vec_compact[i+n].1 - min_cand_orig;
        let dist = if dista < 0 {
            distb
        } else if distb < 0 {
            dista
        } else if dista < distb {
            dista
        } else {
            distb
        };
        max_dist = max_dist.max(dist)
    }
    max_dist
}

fn main() -> std::io::Result<()> {
    let stdin = StdinReader::new();
    let nt = stdin.usize();
    for _ in 0..nt {
        let n = stdin.usize();
        let mut vec: Vec<i32> = Vec::with_capacity(2*n);
        vec.extend(stdin.vec_i32());
        vec.extend(stdin.vec_i32());
        if n == 1 {
            println!("0");
            continue
        }
        let mut vec_compact: Vec<(i32, i32)> = vec![(0,0);2*n];
        let mut vec_sorted: Vec<(i32, i32)> = vec.iter().copied().enumerate().map(|(ix,v)|(ix as i32,v)).collect();
        vec_sorted.sort_unstable_by_key(|(_, k)|*k);
        let mut value = -1;
        let mut premium = 0;
        for (ix, v) in vec_sorted.iter_mut() {
            if *v > value + premium {
                value += 1;
                premium = *v - value;
            }
            vec_compact[*ix as usize] = (value, premium);
            *ix = value;
            *v = premium;
        }
        vec_sorted.dedup();
        let mut up_cand: (i32, i32) = vec_compact[0].max(vec_compact[n]);
        for i in 0..n {
            up_cand = up_cand.min(vec_compact[i].max(vec_compact[i+n]));
        }
        let mut min_dist = get_max_dist(&vec_compact, n, up_cand);
        let endpos = vec_sorted.iter().position(|&x|x == up_cand).unwrap();
        for next_cand in vec_sorted[0..endpos].iter() {
            let next_cand_dist = get_max_dist(&vec_compact, n, *next_cand);
            min_dist = min_dist.min(next_cand_dist);
        }
        println!("{}", min_dist)
    }
    Ok(())
}
