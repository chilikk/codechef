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
    fn vec_i64(&self) -> Vec<i64> {
        self.line().split(' ').map(|s| s.parse::<i64>().unwrap()).collect()
    }
}

fn main() -> std::io::Result<()> {
    let stdin = StdinReader::new();
    let nt = stdin.usize();
    for _ in 0..nt {
        let n = stdin.usize();
        let mut a = stdin.vec_i64();
        let mut maxi = n;
        while maxi > 0 {
            maxi = 0;
            let mut subtotal = a[0];
            let mut nelem = 1i64;
            let mut max_partial_avg = subtotal;
            let mut save_subtotal = subtotal;
            for i in 1..n {
                subtotal += a[i];
                nelem += 1;
                let mut partial_avg = subtotal/nelem;
                if subtotal % nelem > 0{
                    partial_avg += 1;
                }
                if partial_avg > max_partial_avg {
                    maxi = i;
                    max_partial_avg = partial_avg;
                    save_subtotal = subtotal;
                }
            }
            let nelem = (maxi+1) as i64;
            let newv = save_subtotal / nelem;
            let mut extra = save_subtotal % nelem;
            for i in 0..maxi+1 {
                if extra > 0 {
                    a[i] = newv+1;
                    extra -= 1
                } else {
                    a[i] = newv;
                }
            }
        }
        println!("{}", a[0])
    }
    Ok(())
}
