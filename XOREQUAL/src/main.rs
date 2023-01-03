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
    fn u32(&self) -> u32 {
        self.line().parse::<u32>().unwrap()
    }
}

fn main() -> std::io::Result<()> {
    let stdin = StdinReader::new();
    let n_testcases = stdin.u32();
    for _ in 0..n_testcases {
        let mut n = stdin.u32() - 1;
        let mut res: u64 = 1;
        while n > 0 {
            let step = n.min(32);
            n -= step;
            res <<= step;
            res %= 1_000_000_007;
        }
        println!("{}", res);
    }
    Ok(())
}
