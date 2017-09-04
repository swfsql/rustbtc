use std::io;
use std::io::{Read, Write};

fn solve(input: &mut Read, output: &mut Write) {
    let mut sin = String::new();
    input.read_to_string(&mut sin).unwrap();

    let res =
        sin
        .lines()
        .skip(1)
        .map(|x| x
            .trim()
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .enumerate()
        .filter(|&(i, _)| i % 3 != 0)
        .scan(vec![], |st, (i, x)| {
            if i % 3 == 1 {
                *st = x;
                Some(None)
            } else {
                Some(Some(x
                .iter()
                .zip(st)
                .fold(0, |acc, (a, b)| acc + *a * *b)))
            }
        })
        .filter(|&x| x.is_some())
        .map(|x| x.unwrap().to_string() + "\n")
        .collect::<String>();



        /*
        fn iter_split<I, P>(iterador? I, clojure_pra_dar_split? P)
        -> I
        where
            I: ,
            P: FnMut(),
        {
            iterador.scan(Vec::new(), |estado, elemento| {
                if clojure_pra_dar_split(elemento) {
                    let mut estado2 = estado
                    estado = Vec::new();
                    Some(Some(estado2.iter()))
                } else {
                    estado.push(elemento);
                    Some(None)
                }
            })
            .filter(|&x| x.is_some() && x.unwrap().is_some())
            .map(|x| x.unwrap().unwrap())
        }


        fn filter<P>(self, predicate: P) -> Filter<Self, P>
        where
            P: FnMut(&Self::Item) -> bool,

        iter_split(sin.lins(), |&x| x == ' ')

        */



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



