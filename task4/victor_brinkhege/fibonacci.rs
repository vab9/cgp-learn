struct MyType {
    current: u32,
    next: u32,
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
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
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
