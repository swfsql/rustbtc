// http://codeforces.com/problemset/problem/112/A
use std::io;
use std::io::{Read, Write};
use std::ascii::AsciiExt;

fn solve(input: &mut Read, output: &mut Write) {
    let mut sin = String::new();
    input.read_to_string(&mut sin).unwrap();
    let mut s = sin.lines().map(|x| x.trim().chars().map(|y| y.to_ascii_lowercase()));

    let l1 = s.next().unwrap();
    let l2 = s.next().unwrap();

    let mut res = "0".to_string();
    for (u1, u2) in l1.zip(l2) {
        let u: i32 = (u1.to_string().as_bytes()[0] as i32) - (u2.to_string().as_bytes()[0] as i32);
        if u == 0 {
            continue;
        } else if u > 0 {
            res = "1".to_string();
            break;
        } else {
            res = "-1".to_string();
            break;
        }
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
        (1..3 + 1).map(|x| {
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
