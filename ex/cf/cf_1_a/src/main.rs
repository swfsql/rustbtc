// http://codeforces.com/problemset/problem/1/A
use std::io;
use std::io::{Read, Write};
fn solve(input: &mut Read, output: &mut Write) {
    let mut sin = String::new();
    input.read_to_string(&mut sin).unwrap();
    let mut s = sin.trim().split(' ');

    let n: i32 = s.next().unwrap().parse().unwrap();
    let m: i32 = s.next().unwrap().parse().unwrap();
    let a: i32 = s.next().unwrap().parse().unwrap();

    let mut res: i64 = (n / a + if n % a > 0 { 1 } else { 0 }) as i64;
    res += res * ((m - a) / a + if (m - a) % a > 0 { 1 } else { 0 }) as i64;

    output.write(res.to_string().as_bytes()).unwrap();
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
        (1..7).map(|x| {
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
