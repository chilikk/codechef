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
        let p = stdin.vec_usize();
        let mut move1 = 0;
        let mut totaldist1 = 0;
        for i in 0..n {
            if p[i] > n {
                move1 += 1;
                totaldist1 += n-1-i;
            }
        }
        let mut move2 = 0;
        let mut totaldist2 = 0;
        for i in n..2*n {
            if p[i] <= n {
                move2 += 1;
                totaldist2 += i-n;
            }
        }
        assert_eq!(move1, move2);
        println!("{}", totaldist1+totaldist2+move1);
    }
    Ok(())
}
