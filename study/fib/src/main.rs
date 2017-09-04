/*use std::ops::Deref;

impl Deref for Hue {
  type Target = String;

  fn deref(&self) -> &String {
    &self.value
  }
}
*/


struct Fibonacci<T>{
    curr: T,
    next: T,
}

impl <T> Iterator for Fibonacci <T>
     where T: std::ops::Add<Output = T>
     + Clone
    {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}
/*
struct Hue {
  val: String,
}

impl Clone for Hue {
  fn clone(&self) -> Hue {
    Hue { value: self.value}

  }
}*/

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci<&'static str> {
    Fibonacci { curr: "HUEHUE", next: "BR"}
    //Fibonacci { curr: 1, next: 1 }
}

fn Fib2() -> Fibonacci<f32> {
  Fibonacci { curr: 2.1, next: 2.2}
}


fn main() {
    // `0..3` is an `Iterator` that generates: 0, 1, and 2.
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for` works through an `Iterator` until it returns `None`.
    // Each `Some` value is unwrapped and bound to a variable (here, `i`).
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}