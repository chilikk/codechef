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
        let mut vec: Vec<usize> = Vec::with_capacity(2*n);
        vec.extend(stdin.vec_usize());
        vec.extend(stdin.vec_usize());
        vec.sort();
        let mut min = vec[2*n-1] - vec[0];
        for i in 0..n+1 {
            let diff = vec[n-1+i] - vec[i];
            if diff < min {
                min = diff
            }
        }
        println!("{}", min)
    }
    Ok(())
}
