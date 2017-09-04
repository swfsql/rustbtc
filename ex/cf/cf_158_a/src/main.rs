// http://codeforces.com/problemset/problem/158/A
use std::io;
use std::io::{Read, Write};

fn solve(input: &mut Read, output: &mut Write) {
    let mut sin = String::new();
    input.read_to_string(&mut sin).unwrap();
    let mut s = sin.lines();
    let l1 = s.next().unwrap().trim().split(' ').map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let l2 = s.next().unwrap().trim().split(' ').map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

    /*
    let n = l1[0] as usize;
    let k = l1[1] as usize;
    let flag: i32 = l2[k - 1];
    let (flag, contagem, inicio, fim) = if flag > 0 { (flag - 1, k, k, n) } else { (0, 0, 0, k) };
    // no primeiro já tem k e procura depois de k pra frente. No segundo nao tem nenhum e procura do inicio até antes de k

    let res = contagem + &l2[inicio..fim].iter().filter(|&x| *x > flag).fuse().count();


    let pos = l1[1] as usize;
    let min = l2[pos];
    let res = l2.iter().skip(pos).filter(|&e| e >= &min && e > &0 ).count() + if min > 0 { pos } else { 0 };
    */

    let pass = 5;
    let mut lower: i32 = 0;

    let res = l2.iter().enumerate().fold(0, |acc, (index, &ele)| {
        if ele == 0 { acc }
        else if index < pass { acc + 1 }
        else if index == pass { lower = ele; acc + 1 }
        else if ele == lower { acc + 1 }
        else { acc }
    });


    // let mut sin = String::new();
    // input.read_to_string(&mut sin).unwrap();



    // let pos = sin.chars().next().split(' ').unwrap();


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
