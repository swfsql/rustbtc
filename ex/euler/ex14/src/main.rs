
use std::collections::HashMap;


struct Seq<'a> {
    hm: &'a mut HashMap<u32, u32>,
    n: u32,
    count: u32,
    vec: Vec<u32>,
}

impl <'a>Seq <'a> {
    fn new(hm: &'a mut HashMap<u32, u32>, n: u32) -> Seq<'a> {
        Seq {
            hm: hm,
            n: n,
            count: 0,
            vec: vec![],
        }
    }

    fn close(&self) -> &'a mut HashMap<u32, u32> {
        self.hm
    }

}

// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1


impl <'a> Iterator for Seq<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {

    println!("n: {:?}, count: {:?}", &self.n, &self.count);

        if let Some(c) = self.hm.get(&self.n)  {
            self.count += *c;
            self.vec.push(self.n);
            self.n = 1;
        }

        let mut hm = self.hm.clone();
        let mut count = self.count;

        if self.n == 1 {
            // monta o hash
            self.vec.into_iter().rev().map(move |x| {
                self.hm.entry(x).or_insert(count);
                //hm[x] = count;
                self.count -= 1;
                ()
            }).collect::<Vec<()>>();
            //self.count = count;
            None
        } else if self.n % 2 == 0 { // par
            self.count += 1;
            self.vec.push(self.n);
            self.n /= 2;
            Some(self.n)
        } else { // ímpar
            self.count += 1;
            self.vec.push(self.n);
            self.n = (self.n) * 3  + 1;
            Some(self.n)
        }


    }
}

fn solve() {

    let mut hm: HashMap<u32, u32> = HashMap::new();

    let it: Seq = Seq::new(&mut hm, 13);
    it.collect::<Vec<u32>>();
    let hm = it.close();
    println!("{:?}",hm.get(&13));
    println!("{:?}",hm);

}


fn main() {
    solve();
}




