// http://codeforces.com/problemset/problem/50/A
use std::io;
use std::io::{Read, Write};

fn solve(input: &mut Read, output: &mut Write) {
    let mut sin = String::new();
    input.read_to_string(&mut sin).unwrap();
    let mut s = sin.lines();
    let l1 = s.next().unwrap().trim().split(' ').map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

    let mut m = l1[0];
    let mut n = l1[1];

    let mut res = 0;
    // vai fazendo uma espiral
    while m > 1 && n > 1 {
        res += m + n - 2;
        // diminui o tamanho da proxima espiral
        m -= 2;
        n -= 2;
    }
    // a regra muda quando m ou n = 1
    if m > 0 && n > 0 {
        res += std::cmp::max(m, n) / 2;
    }

    output.write(res.to_string().trim().as_bytes()).unwrap();
}

fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use solve;

    #[test]
    fn test() {
        (1..17 + 1).map(|x| {
            println!("test #{}", x);
            let mut fin = File::open("./src/in".to_string() + &x.to_string() + ".txt").unwrap();
            let mut buf: Vec<u8> = Vec::new();
            solve(&mut fin, &mut buf);
            let res = String::from_utf8(buf).unwrap();

            let mut fout = File::open("./src/out".to_string() + &x.to_string() + ".txt").unwrap();
            let mut sout = String::new();
            fout.read_to_string(&mut sout).unwrap();
            assert_eq!(res, sout.trim());

        }).count();

    }
}
