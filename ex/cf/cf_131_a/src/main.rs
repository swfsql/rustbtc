// http://codeforces.com/problemset/problem/131/A
use std::io;
use std::io::{Read, Write};

fn solve(input: &mut Read, output: &mut Write) {
    let mut sin = String::new();
    input.read_to_string(&mut sin).unwrap();
    let sin = sin.lines().next().unwrap().trim();
    let res: String;
    if sin.chars().skip(1).all(|x| x.is_uppercase()) || (sin.chars().count() == 1 && sin.chars().next().unwrap().is_uppercase()) {
        let mut chars = sin.chars();
        let first = chars.next().unwrap();
        let first: String = if first.is_uppercase() { first.to_lowercase().collect() } else { first.to_uppercase().collect() };
        res = first + &chars.collect::<String>().to_lowercase();
    } else {
        res = sin.to_string();
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
        (1..1 + 1).map(|x| {
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
