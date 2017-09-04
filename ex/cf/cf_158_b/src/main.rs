// http://codeforces.com/problemset/problem/158/B
use std::io;
use std::io::{Read, Write};

fn solve(input: &mut Read, output: &mut Write) {
    let mut sin = String::new();
    input.read_to_string(&mut sin).unwrap();
    let mut lens = [0, 0, 0, 0];
    sin.lines().skip(1).next().unwrap().trim().split(' ')
        .map(|x| {
            let y = x.parse::<usize>().unwrap();
            lens[y - 1] += 1;
            y
        })
        .count();
    let mut res = lens[3] + lens[2]; // grupos de 4 e de 3
    lens[0] = if lens[2] <= lens[0] { lens[0] - lens[2] } else { 0 }; // grupos de 1 entram com os de 3
    res += lens[1] / 2; // grupos de 2 vao de dois a dois
    if lens[1] % 2 == 1 {
        res += 1; // último grupo de 2 entra, se tiver
        lens[0] = if lens[0] <= 2 { 0 } else { lens[0] - 2 }; // um ou dois do grupo de 1 vao com este ultimo de 2
    }
    res += lens[0] / 4; // grupos de 1 vao de quatro a quatro
    res += if lens[0] % 4 != 0 { 1 } else { 0 }; // os ultimos do grupo de 1 entram

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
        (1..2 + 1).map(|x| {
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
