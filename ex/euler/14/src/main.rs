use std::io;
use std::io::{Read, Write};


struct Seq {
    val : u64;
}

impl Iterator for Seq {
    fn next() ->  {
        if val
    }
}

fn solve(input: &mut Read, output: &mut Write) {
    let mut sin = String::new();
    input.read_to_string(&mut sin).unwrap();

    let mut s = sin.lines();

    let res = "resposta".to_string();

    output.write(res.to_string().trim().as_bytes()).unwrap();
}


fn main() {
    solve(&mut io::stdin(), &mut io::stdout());
}




