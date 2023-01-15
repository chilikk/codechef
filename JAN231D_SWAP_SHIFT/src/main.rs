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
    fn vec_usize(&self) -> Vec<usize> {
        self.line().split(' ').map(|s| s.parse::<usize>().unwrap()).collect()
    }
}

fn find_best_with_123(a: &Vec<usize>, n: usize, positions: &Vec<usize>, pre_swaps: usize, best: &mut Vec<usize>) {
    for start_before_i in 0..3 {
        if start_before_i >= n {
            continue
        }
        let start_i = (n + positions[start_before_i] - start_before_i) % n;
        let mut candidate = Vec::with_capacity(n);
        candidate.extend(&a[start_i..]);
        candidate.extend(&a[..start_i]);
        let mut cand_positions: Vec<usize> = positions.iter().map(|pos|(n+pos-start_i)%n).collect();
        let mut swaps = 2 - pre_swaps;
        let mut i = 0;
        while swaps > 0 && i < n {
            if candidate[i] != i + 1 {
                let cur = candidate[i];
                candidate.swap(i, cand_positions[i]);
                cand_positions.swap(cur-1, i);
                swaps -= 1;
            }
            i += 1;
        }
        if candidate < *best {
            *best = candidate;
        }
    }
}

fn main() -> std::io::Result<()> {
    let stdin = StdinReader::new();
    let nt = stdin.usize();
    let possible_pre_swap_combinations = vec![
        Vec::new(),           // 0 1 2
        vec![(0, 1)],         // 1 0 2
        vec![(0, 2)],         // 2 1 0
        vec![(1, 2)],         // 0 2 1
        vec![(0, 1), (0, 2)], // 2 0 1
        vec![(0, 1), (1, 2)]  // 1 2 0
    ];
    for _ in 0..nt {
        let n = stdin.usize();
        let a = stdin.vec_usize();
        let mut best: Vec<usize> = vec![n;n];
        if n <= 3 {
            best[..].clone_from_slice(&a[..]);
            best.sort();
        } else {
            let mut positions = vec![n;n];
            for i in 0..n {
                positions[a[i]-1] = i;
            }
            for pre_swaps in possible_pre_swap_combinations.iter() {
                let mut acopy = a.clone();
                let mut poscopy = positions.clone();
                let n_pre_swaps = pre_swaps.len();
                for pre_swap in pre_swaps {
                    acopy.swap(poscopy[pre_swap.0], poscopy[pre_swap.1]);
                    poscopy.swap(pre_swap.0, pre_swap.1);
                }
                find_best_with_123(&acopy, n, &poscopy, n_pre_swaps, &mut best);
            }
        }
        print!("{}", best[0]);
        for i in 1..n {
            print!(" {}", best[i])
        }
        println!();
    }
    Ok(())
}
