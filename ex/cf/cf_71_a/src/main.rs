// http://codeforces.com/problemset/problem/71/A
use std::io;
use std::io::{Read, Write};

fn solve(input: &mut Read, output: &mut Write) {
    let mut sin = String::new();
    input.read_to_string(&mut sin).unwrap();
    let mut s = sin.lines();

    let n: i32 = s.next().unwrap().trim().parse().unwrap();

    let res = (0..n)
        .map(|_| {
            let line = s.next().unwrap().trim();
            let mut chars = line.chars();
            let count = line.chars().count();
            return if count <= 10 {
                chars.collect::<String>() + "\r\n"
            } else {
                chars.next().unwrap().to_string() +
                    &(count - 2).to_string() + &chars.last().unwrap().to_string() + "\r\n"
            };
        }).collect::<String>();


    output.write(
        res.to_string().trim().as_bytes()
    ).unwrap();
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
        (1..2).map(|x| {
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