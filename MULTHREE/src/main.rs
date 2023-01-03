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
    fn u16(&self) -> u16 {
        self.line().parse::<u16>().unwrap()
    }
    fn vec_u64(&self) -> Vec<u64> {
        self.line().split(' ').map(|s| s.parse::<u64>().unwrap()).collect()
    }
}

fn main() {
    let stdin = StdinReader::new();
    let n_testcases = stdin.u16();
    for _ in 0..n_testcases {
        if let [k, d0, d1] = stdin.vec_u64()[..] {
            let mut head = vec![d0, d1];
            let mut i = 2;
            let mut d = (d0+d1) % 10;
            while i < k && ! head[2..].contains(&d) {
                head.push(d);
                d *= 2;
                d %= 10;
                i += 1;
            }
            let div3 = if i == k {
                head.iter().sum::<u64>() % 3 == 0
            } else {
                let period_start = head[2..].iter().position(|x|*x==d).unwrap();
                let headref = &head[..2+period_start];
                let periodref = &head[2+period_start..];
                let head_mod3: u64 = headref.iter().sum::<u64>() % 3;
                let k_rem = k - headref.len() as u64;
                let period_mod3: u64 = periodref.iter().sum::<u64>() % 3;
                let period_len = periodref.len() as u64;
                let period_repetitions_mod3 = k_rem / period_len % 3;
                let rem_digits = k_rem % period_len;
                let tail_mod3: u64 = periodref[..rem_digits as usize].iter().sum::<u64>() % 3;
                (head_mod3 + period_mod3*period_repetitions_mod3 + tail_mod3) % 3 == 0
            };
            match div3 {
                true => println!("YES"),
                false => println!("NO"),
            }
        }
    }
}
