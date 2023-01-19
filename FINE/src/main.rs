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
    fn u8(&self) -> u8 {
        self.line().parse::<u8>().unwrap()
    }
}

fn main() -> std::io::Result<()> {
    let stdin = StdinReader::new();
    let nt = stdin.u8();
    for _ in 0..nt {
        let speed = stdin.u8();
        let fine = if speed <= 70 {
            0
        } else if speed <= 100 {
            500
        } else {
            2000
        };
        println!("{}", fine)
    }
    Ok(())
}
