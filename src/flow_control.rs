pub fn main_if() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, half the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);
}

pub fn main_loop() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

pub fn main_while() {
    // The fizz buzz game
    let mut counter = 1;

    while counter < 101 {
        if counter % 15 == 0 {
            println!("FizzBuzz");
        } else if counter % 5 == 0 {
            println!("Buzz");
        } else if counter % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", counter);
        }
        counter += 1;
    }
}

pub fn main_for() {
    let mut names = vec!["Szeka", "Feri", "Bela"];

    // Using .iter on a vec: .iter borrows the elements from the list
    for name in names.iter() {
        match name {
            &"Szeka" => println!("Hey it's me"),
            _ => println!("Isn't me")
        }
    }
    println!("{:?}", names);

    // Using .iter_mut() gives a mutable refferece to each element
    for name in names.iter_mut() {
        match name {
            &mut "Szeka" => println!("It's me"),
            _ => println!("Not me")
        }
    }

    // Using .into_iter(). It consumes all the elements from the collection. (Gives ownership awy)
    for name in names.into_iter() {
        match name {
            "Szeka" => println!("Hey it's me"),
            _ => println!("Isn't me")
        }
    }
    // This won't work because "The value is moved". The into_iter() method consumed the list
    // println!("{:?}", names);
}

pub fn main_match() {
    let number = 13;

    match number {
        1 => println!("IS ONE"),
        3 | 5 | 7 | 11 => println!("Prime number"),
        // 13..19 => println!("It's a teenager"),
        _ => println!("Nothing special here {}", number)
    }
}