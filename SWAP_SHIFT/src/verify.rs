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

fn main() -> std::io::Result<()> {
    let stdin = StdinReader::new();
    let nt = stdin.usize();
    for _ in 0..nt {
        let n = stdin.usize();
        let a = stdin.vec_usize();
        let pos1 = a.iter().position(|&x|x==1).unwrap();
        let mut best: Vec<usize> = Vec::with_capacity(n);
        best.extend_from_slice(&a[pos1..]);
        best.extend_from_slice(&a[..pos1]);
        let mut cand = vec![n;n];
        for swap1 in 0..n {
            for swap2 in 0..n {
                if swap1 == swap2 {
                    continue
                }
                let mut acopy = a.clone();
                acopy.swap(swap1, swap2);
                let newpos1 = if pos1 == swap1 {
                    swap2
                } else if pos1 == swap2 {
                    swap1
                } else {
                    pos1
                };
                assert_eq!(acopy[newpos1], 1);
                cand[..n-newpos1].clone_from_slice(&acopy[newpos1..]);
                cand[n-newpos1..].clone_from_slice(&acopy[..newpos1]);
                if cand < best {
                    best = cand.clone();
                }
                for swap3 in 0..n {
                    for swap4 in 0..n {
                        if swap3 == swap4 {
                            continue
                        }
                        let mut acopy2 = acopy.clone();
                        acopy2.swap(swap3, swap4);
                        let newpos3 = if newpos1 == swap3 {
                            swap4
                        } else if newpos1 == swap4 {
                            swap3
                        } else {
                            newpos1
                        };
                        assert_eq!(acopy2[newpos3], 1);
                        cand[..n-newpos3].clone_from_slice(&acopy2[newpos3..]);
                        cand[n-newpos3..].clone_from_slice(&acopy2[..newpos3]);
                        if cand < best {
                            best = cand.clone();
                        }
                    }
                }
            }
        }
        print!("{}", a[0]);
        for i in 1..n {
            print!(" {}", a[i])
        }
        print!(" -> ");
        print!("{}", best[0]);
        for i in 1..n {
            print!(" {}", best[i])
        }
        println!();
    }
    Ok(())
}
