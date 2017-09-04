// http://codeforces.com/problemset/problem/339/A
use std::io;
use std::io::{Read, Write};

fn solve(input: &mut Read, output: &mut Write) {
    let mut sin = String::new();
    input.read_to_string(&mut sin).unwrap();
    let mut s = sin.lines()
        .next().unwrap().trim()
        .split('+').map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    s.sort();
    let string: String = s.iter().map(|x| x.to_string() + "+").collect();
    let res = &string[..string.len()-1];

    output.write(res.to_string().trim().as_bytes()).unwrap();
}

iterator
clojure
vetor
string
result/option

Fira Code

result: OK, ERR
option: SOME, NONE

lala::haha
abc::abc
!=
<=
>=
www.haha.com
->- -   -aaa---a-a-a-a-a-->aaaa

nossa_func_que_abre_arquivo() -> Result<...>  {
    file = open("arquivo.txt")   result
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
