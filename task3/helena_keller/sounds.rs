pub trait AnimalSound {
	fn sound(&self);
	fn make_sound_twice(&self) {
		self.sound();
		self.sound();
	}
}

struct Dog;

impl AnimalSound for Dog {
	
	fn sound(&self) {
		println!("Wuff!");
	}
}

struct Cat;

impl AnimalSound for Cat {
	
	fn sound(&self) {
		println!("Meow!");
	}
}

fn foo<T: AnimalSound>(x: T) {

	x.sound();
}

fn main() {

	let cat = Cat;
	let dog = Dog;

	cat.sound();
	cat.make_sound_twice();

	dog.sound();
	dog.make_sound_twice();
	
	foo(Cat);
}
