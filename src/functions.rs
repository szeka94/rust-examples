use std::mem;

// How does closures take refference, or ownership on the outer scope variables
pub fn main_capturing() {
    // This takes & borrow for the variable
    let color = "green";
    let print = || println!("Color: {}", color);
    print();
    print();

    // Changing a variable value from a closure can be done using &mut borrow,
    // or by taking ownership over the variable. &mut borrow is less restrictive so this one is gonna be
    let mut count = 0;
    // We need a mut keyword before inc because, it stores a &mut. So by calling the closure it mutates
    // the closure itself
    let mut inc = || {
        count += 1;
        println!("Count increased: {}", count);
    };
    inc();
    inc();

    // For some weird reason none of these will work. The mutable borrow from the closure is still alive
    // let _reborrow = &mut count;
    // println!("count: {}", &count);

    // Non-copy type
    let movable = Box::new(3);

    // mem::drop will take T by value. A copy type would copy into the closure leaving the original
    // untouched. A non-copy type will move into the closure
    let consume = || {
        println!("Movable: {}", movable);
        mem::drop(movable);
    };
    consume();
    // Since movable is a non-copy type the value was moved into the consume closure. Non of the
    // followings will work.
    // consume();
    // println!("Movable: {}", movable);

    // Using `move` keyword on closures forces the closure to take ownership from the captured variable
    let haystack = vec![1, 2, 3];
    let take_own = move |needle| haystack.contains(needle);

    println!("Contains 1: {}", take_own(&1));
    println!("Contains 5: {}", take_own(&5));

    // This line will cause a compile error, cause the `haystack` variable was moved
    // into the closure `take_own`. Removing the `move` kw results in the closure taking
    // read-only (&) refference so the line below would work.
    // println!("There are in total {} elements in the vec", haystack.len());

    // Testing the same stuff with copy variable
    // RESULT: the following would work because the num is a copy variable. The `take_own_num`
    // closure will get a copy of the variable.
    let num: i32 = 10;
    let take_own_num = move || println!("The number is: {}", num);

    take_own_num();
    take_own_num();

    println!("num: {}", num);
}


fn apply<F>(f: F) where F: FnOnce() {
    f();
}

fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(3)
}


pub fn main_clos_input() {
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let dairy = || {
        // This only requires Fn
        println!("I said {} to him", greeting);

        // Mutating an outside variable requires FnMut
        farewell.push_str("!!!");
        println!("And then I screamed: {}", farewell);

        // Manually calling drop forces `farewell` to be captured by value
        // This requires FnOnce()
        mem::drop(farewell);
    };

    apply(dairy);

    let double = |x: i32| x * 2;

    println!("Lets see 3 doubled: {}", apply_to_3(double));
}

fn call_me<F: Fn()>(f: F) {
    f();
}

pub fn input_fn() {
    fn simple_fn() {
        println!("I was called");
    }

    let clos_fn = || println!("Closure was also called");

    call_me(simple_fn);
    call_me(clos_fn);
}

// Functions returned inside a functione

fn create_fn() -> Box<Fn()> {
    let text = "data".to_string();
    Box::new(move || println!("Moving out some {}", text))
}

pub fn output_fn() {
    let test_fn = create_fn();
    test_fn();
}