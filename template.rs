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
    fn usize(&self) -> usize {
        self.line().parse::<usize>().unwrap()
    }
    fn vec_usize(&self) -> Vec<usize> {
        self.line().split(' ').map(|s| s.parse::<usize>().unwrap()).collect()
    }
}

fn main() -> std::io::Result<()> {
    let stdin = StdinReader::new();
    //
    Ok(())
}
