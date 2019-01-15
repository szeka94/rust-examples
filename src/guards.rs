pub fn main() {
    let pair = (3, -2);

    match pair {
        (x, y) if x == y => println!("Hey there, these are equal"),
        (x, y) if x + y == 0 => println!("Antimatter"),
        (x, _) if x > 0 => {
            print!(" differenet syntax");
            println!("lets try this");
        },
        _ => println!("Anything else")

    }
}

fn outside_func() -> i32 {
    15
}

pub fn main_binding() {
    match outside_func() {
        0 => println!("Not borned yet"),
        // The following is the binding syntax
        n @ 1 ... 12 => println!("It's just a child"),
        n @ 13 ... 19 => println!("Hard years, it's a teenager"),
        _ => println!("Anything else")
    }
}


enum Foo {
    A,
    B,
    C(u32, u32, u32)
}

pub fn main_iflet() {
    let some_number: Option<i32> = Some(10);
    let some_val_missing: Option<i32> = None;

    if let Some(i32) = some_number {
        println!("some_number: {}", some_number.unwrap());
    }

    if let Some(i32) = some_val_missing {
        println!("Won't happen");
    } else {
        println!("Value should be missing");
    }

    let some_enum = Foo::C(10, 10, 10);

    if let Foo::A = some_enum {
        println!("This is some Enum of A type");
    } else if let Foo::B = some_enum {
        println!("This is of type B");
    } else if let Foo::C(10, 10, 10) = some_enum {
        println!("This is the correct type");
    } else {
        println!("Nevermind");
    }
}

pub fn main_whilelet() {
    let mut some_value: Option<i32> = Some(10);

    while let Some(i) = some_value {
        if i > 20 {
            println!("Max number count achieved: {}", i);
            some_value = None;
        } else {
            some_value = Some(i + 2);
        }
        println!("End of one circe");
    }
}