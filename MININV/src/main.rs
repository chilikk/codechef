use std::io::BufRead;
use std::collections::HashMap;

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
    fn vec_u32(&self) -> Vec<u32> {
        self.line().split(' ').map(|s| s.parse::<u32>().unwrap()).collect()
    }
}

fn main() -> std::io::Result<()> {
    let stdin = StdinReader::new();
    let nt = stdin.usize();
    for _ in 0..nt {
        let n = stdin.usize();
        let a = stdin.vec_u32();
        assert_eq!(a.len(), n);
        let mut freq: HashMap<u32, u32> = HashMap::with_capacity(n);
        let mut invs1 = vec![0;n];
        let mut invsum = 0;
        for i in 0..n {
            let inc = freq.get(&(a[i]+1)).unwrap_or(&0);
            invs1[i] += inc;
            invsum += inc;
            freq.entry(a[i]).and_modify(|cnt|*cnt+=1).or_insert(1);
        }
        if invsum == 0 {
            println!("0");
            continue
        }
        let mut sum = 0;
        let mut max = invs1[n-1];
        let mut freqr: HashMap<u32, u32> = HashMap::with_capacity(n);
        let mut invsr;
        for i in (0..n).rev() {
            invsr = freqr.get(&(a[i]-1)).unwrap_or(&0);
            sum += invs1[i] - invsr;
            freqr.entry(a[i]).and_modify(|cnt|*cnt+=1).or_insert(1);
            if sum > max {
                max = sum;
            }
        }
        println!("{}", max)
    }
    Ok(())
}
