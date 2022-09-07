use std::fmt::Display;

use structural::{Struct, HasAttr};


#[derive(Struct)]
pub struct User<T> {
    name: String,
    id: T
}

pub fn print_name<U: Display, T: HasAttr<"name", Ty = U>>(t: &T) {
    let name = t.get();
    println!("{name}");
}

pub fn print_id<U: Display, T: HasAttr<"id", Ty = U>>(t: &T) {
    println!("{}", t.get())
}

fn main() {
    let user = User {
        name: "Zomatreee".to_string(),
        id: 0
    };

    print_name(&user);
    print_id(&user);
}
