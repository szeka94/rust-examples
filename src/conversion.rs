pub fn main() {
    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);

    // EXCERCISE: Implement FromStr trait for this custem number type
    use std::str::FromStr;
    use std::num::ParseIntError;

    impl FromStr for Number {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let num = s.parse::<i32>().unwrap();
            Ok(Number { value: num })
        }
    }

    println!("My number is {:?}", Number::from_str("9"));

}

pub fn main_conv_strings() {
    use std::string::ToString;

    struct Circle {
        radius: i32
    }

    impl ToString for Circle {
        fn to_string(&self) -> String {
            format!("Circle of radius {:?}", self.radius)
        }
    }
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // Parsing a string into a number
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};

}