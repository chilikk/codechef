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
    fn skip_line(&self) {
        let lock = self.stdin.lock();
        let mut lines = lock.lines();
        lines.next();
    }
    fn u32(&self) -> u32 {
        self.line().parse::<u32>().unwrap()
    }
}

fn main() -> std::io::Result<()> {
    let stdin = StdinReader::new();
    //
    Ok(())
}
