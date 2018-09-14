#[derive(Copy, Clone)]
struct M2 {
    data: [[u32; 2]; 2]
}

impl M2 {
    pub fn mult(self, b: &M2) -> M2 {
        let mut res = M2 {
            data: [[0;2], [0;2]]
        };

        let ubound = self.data.len();

        for i in 0..ubound {
            for j in 0..ubound {
                for k in 0..ubound {
                    res.data[i][j] += self.data[i][k] * b.data[k][j];
                }
            }
        }

        res
    }

    pub fn pow(self, n: u32) -> M2 {
        if n == 0 || n == 1 {
            self
        } else {
            let p = self.pow(n / 2);
            let p = p.mult(&p);

            if n % 2 != 0 { self.mult(&p) } else { p }
        }
    }

    pub fn at(self, i: usize, j: usize) -> u32 { self.data[i][j] }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return
    }

    let n: u32 = args[1].trim().parse().expect("non-numeric input argument");

    println!("Fib({}) -> {}", n, fib(n));
    println!("FibDP({}) -> {}", n, fib_dp(n))
}

fn fib(n: u32) -> u32 {
    let res = M2 { data: [[1,1], [1,0]] };

    if n <= 2 {
        res
    } else {
        res.pow(n - 1)
    }.at(0, 0)
}

fn fib_dp(n: u32) -> u32 {
    if n <= 2 {
        return 1
    }

    let mut dp_ary: Vec<u32> = vec![0; n as usize];

    fn fib_dp_int(i: u32, interim: &mut Vec<u32>) -> u32 {
        if i <= 2 {
            return 1
        }

        let j = (i - 1) as usize;

        if interim[j] != 0 {
            return interim[j]
        }

        interim[j] = fib_dp_int(i - 1, interim) + fib_dp_int(i - 2, interim);
        interim[j]
    };

    fib_dp_int(n, &mut dp_ary)
}