# Structural

Structural typing in rust


# Example
```rs
use structural::{Struct, HasAttr};


#[derive(Struct)]
pub struct User {
    name: String
}

pub fn print_name<T: HasAttr<"name", Ty = String>>(t: T) {
    println!("{}", t.get())
}
```
