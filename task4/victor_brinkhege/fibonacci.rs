struct MyType {
    current: usize,
    next: usize,
}

impl MyType {
    fn new() -> Self {
        MyType {
            current: 1,
            next: 1,
        }
    }
}

impl Iterator for MyType {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let following = self.current + self.next;

        self.current = self.next;
        self.next = following;

        Some(self.current)

    }
}

fn main() {
    for i in MyType::new().take(20) {
        println!("{}", i);
    }
}
