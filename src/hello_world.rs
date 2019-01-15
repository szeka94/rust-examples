pub fn formatting() {
    // Formatting by order
    println!("This is {1}, {1} is {0}s wife.", "Bob", "Alice");

    // Formatting by keyword arguments
    println!("{subject} {verb} {object}",
             subject="The lazy dog",
             verb="jumps over",
             object="the quick brown fox!");

    // Printing as binary
    println!("{} of {:b} people know binary, the other doesn't.", 1, 2);

    // Right allign based on width parameter
    println!("{number:>width$}", number=1, width=10);

    // Pad numbers with extra zeros
    println!("{number:>0width$}", number=1, width=10);

    // If there are not enough arguments passed in, it shouldn't compile
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Trying to print a complex type (as a Struct)
    #[derive(Debug)]
    struct Structure(i32);
    println!("This is my struct: {:?}", Structure(10));

    // TODO: print pi with decimals
    let pi = 3.141592;
    println!("{:.5}", pi);
}

pub fn debug_trait() {
    // Printing nested structures. By doing so we have no control over how the
    // result looks like
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?}", Deep(Structure(10)));

    // Pretty printing with {:#?}
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 24;
    let peter = Person {name, age};

    println!("{:#?}", peter);
}

pub fn display_trait() {
    use std::fmt;

    // Implementing the display trait manually for our struct
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    println!("Structure: {}", Structure(10));

    // Another Display trait implementation
    struct Minimax (i32, i32);

    impl fmt::Display for Minimax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({} {})", self.0, self.1)
        }
    }
    println!("Minimax: {}", Minimax(10, 20));

    // Last example
    struct Point {
        x: i32,
        y: i32
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {x_coord}, y: {y_coord}",
                   x_coord=self.x,
                   y_coord=self.y)
        }
    }
    let point = Point {x: 10, y: 15};
    println!("{}", point);

    // TODO: Add Debug, Display traits for printing complex numbers
    #[derive(Debug)]
    struct Complex {
        real: i32,
        imag: i32,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} +{}i", self.real, self.imag)
        }
    }
    let com_num = Complex {
        real: 10,
        imag: 6
    };
    println!("{:?}", com_num);
    println!("{}", com_num);
}

pub fn list_display_impl() {
    use std::fmt;

    struct List (Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", count, v)?;
            }
            write!(f, "]")
        }
    }

    println!("{}", List(vec![1,2,3,4]));
}