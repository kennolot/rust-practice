
fn def_vars(){
    let mut x: i32 = 10;
    println!("x: {x}");
    x = 20;
    println!("x: {x}");
}

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

// exercise: fibonacci
fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        return n;
    } else {
        // The recursive case.
        return fib(n-1) + fib(n-2);
    }
}


fn blocks(){
    let z = 13;
    let x = {
        let y = 10;
        dbg!(y);
        z - y
    };
    dbg!(x);
}

fn if_statements(){
    let x = 10;
    if x == 0 {
        println!("zero!");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }
}

fn if_as_expression(){
    let x = 10;
    let size = if x < 20 { "small" } else { "large" };
    println!("number size: {}", size);
}

fn match_statement(){
    let val = 1;
    match val {
        1 => println!("one"),
        10 => println!("ten"),
        100 => println!("one hundred"),
        _ => {
            println!("something else");
        }
    }
}

fn while_loop(){
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    dbg!(x);
}

fn for_loops(){
    for x in 1..5 {
        dbg!(x); // end not included
    }

    for elem in [2, 4, 8, 16, 32] {
        dbg!(elem); // end included
    }
}

fn loop_statement(){
    let mut i = 0;
    loop {
        i += 1;
        dbg!(i);
        if i > 100 {
            break;
        }
    }
}

fn labels(){
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    // 3x3 array
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    dbg!(elements_searched);
}

/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    if n == 1 {

    }
    else if n % 2 == 0 { // even


    }

    else { // odd
        n = 3 * n + 1;
    }
}

fn main() {    
    def_vars();
    def_vars2();
    println!("result: {}", interproduct(2, 1, 1));

    // exercise fibonacci:
    let n = 20;
    println!("fib({n}) = {}", fib(n));
    
    blocks();


    if_statements();
    if_as_expression();
    match_statement();
    while_loop();
    for_loops();
    loop_statement();
    
    labels();

    println!("Length: {}", collatz_length(11)); // should be 15
}


fn def_vars2(){
    let mut x: i32 = 30;
    println!("x: {x}");
    x = 40;
    println!("x: {x}");
}