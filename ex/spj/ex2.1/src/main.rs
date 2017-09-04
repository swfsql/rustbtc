use std::io;
use std::io::{Read, Write};

struct Primos {
    elementos: Vec<u32>,
    ultimo: u32,
}

impl Primos {
    fn new() -> Primos {
        Primos { elementos: vec![], ultimo: 1,}
    }
}

impl Iterator for Primos {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let inicial: u32 = self.ultimo + 1;
        let proximo = (inicial..)
            .filter(|&y|
                !(self.elementos
                    .iter()
                    .map(|x| if y % x == 0 {1} else {*x})
                    .take_while(|&x| y / x > x)
                    .any(|x| y % x == 0)
                )
            )
            .next().unwrap();

        self.elementos.push(proximo);
        self.ultimo = proximo;
        Some(proximo)
    }
}



fn solve(input: &mut Read, output: &mut Write) {
    let mut sin = String::new();
    input.read_to_string(&mut sin).unwrap();

    let res = sin
        .lines()
        .skip(1)
        .map(|x| x
            .trim()
            .split(' ')
            .map(|y| y.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
        )
        .map(|x|
            Primos::new()
                .skip_while(|&y| y < x[0])
                .take_while(|&y| y <= x[1])
                .map(|y| y.to_string() + "\n")
                .collect::<String>() + "\n"
        )
        .collect::<String>();


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



