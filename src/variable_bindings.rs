fn variable_shadowing() {
    let long_lived_variable = 10;

    {
        let short_lived_variable = 20;

        println!("short_lived_variable: {}", short_lived_variable);
        println!("long_lived_variable: {}", long_lived_variable);

        // This is shadowing
        let long_lived_variable = 30.0f32;
        println!("long_lived_variable inner block: {}", long_lived_variable as f32);
    }

    // It returns the original one: 10
    println!("long_lived_variable after block is over {}", long_lived_variable);
    // This should raise an error (and it does)
    // println!("shot lived variable: {}", short_lived_variable);
}

pub fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);
    println!("Original integer: {:?}", an_integer);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
    variable_shadowing()
}

