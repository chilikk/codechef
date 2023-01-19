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
    fn i16(&self) -> i16 {
        self.line().parse::<i16>().unwrap()
    }
    fn vec_i16(&self) -> Vec<i16> {
        self.line().split(' ').map(|s| s.parse::<i16>().unwrap()).collect()
    }
}

fn main() -> std::io::Result<()> {
    let stdin = StdinReader::new();
    let nt = stdin.i16();
    for _ in 0..nt {
        let vec = stdin.vec_i16();
        let mut has_positive = false;
        let mut has_negative = false;
        for i in vec {
            if i > 0 {
                has_positive = true
            } else if i < 0 {
                has_negative = true
            }
        }
        if has_positive && has_negative {
            println!("YES")
        } else {
            println!("NO")
        }
    }
    Ok(())
}
