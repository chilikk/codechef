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
    fn u16(&self) -> u16 {
        self.line().parse::<u16>().unwrap()
    }
}

fn main() -> std::io::Result<()> {
    let stdin = StdinReader::new();
    let n_testcases = stdin.u16();
    for _ in 0..n_testcases {
        stdin.skip_line();
        let input = stdin.line();
        let mut maybe_max = true;
        let mut count = 0;
        for c in input.chars() {
            match (c, maybe_max) {
                ('0', true) => (),
                ('1', true) => {
                    count += 1;
                    maybe_max = false;
                },
                ('0', false) => maybe_max = true,
                ('1', false) => (),
                (_, _) => panic!("cannot happen"),
            }
        }
        if maybe_max {
            count += 1
        }
        println!("{}", count);
    }
    Ok(())
}
