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
    fn i32(&self) -> i32 {
        self.line().parse::<i32>().unwrap()
    }
    fn vec_i32(&self) -> Vec<i32> {
        self.line().split(' ').map(|s| s.parse::<i32>().unwrap()).collect()
    }
}

fn main() -> std::io::Result<()> {
    let stdin = StdinReader::new();
    let nt = stdin.i32();
    for _ in 0..nt {
        let input = stdin.vec_i32();
        let d1sq = input[0]*input[0] + input[1]*input[1];
        let d2sq = input[2]*input[2] + input[3]*input[3];
        if d1sq > d2sq {
            println!("ALEX")
        } else if d1sq == d2sq {
            println!("EQUAL")
        } else {
            println!("BOB")
        }
    }
    Ok(())
}
