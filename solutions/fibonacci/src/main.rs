struct Fibonacci {
    curr: u128,
    next: u128,
}

impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr.checked_add(self.next)?;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self { curr: 0, next: 1 }
    }
}

fn main() {
    let fibb = Fibonacci { curr: 0, next: 1 };

    for (idx, i) in fibb.enumerate() {
        println!("{}: {}", idx, i)
    }
}

#[test]
fn test_fibonacci_creation() {
    let fib = Fibonacci::default();
    assert_eq!(fib.curr, 0);
    assert_eq!(fib.next, 1);
}

#[test]
fn test_fibonacci_vector() {
    let coll: Vec<u128> = Fibonacci::default().take(10).collect();
    assert_eq!(coll, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
}
