trait Talk {
    fn magic_words() {
        println!("Hellow world");
    }
    
    fn hello(&self) {
        println!("Hi");
    }
}

struct Generic<T> {
    thing: T
}

struct Ayt;
impl Talk for Generic<String> {}

pub fn myFunc() {
    // static
    Generic::<String>::magic_words();

    let t = Generic { thing: 42.to_string() };
    t.hello();
}
