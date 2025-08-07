#![allow(dead_code)]
// *******************
//
// Morning
//
// *******************


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
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

// *******************
//
// Afternoon
//
// *******************

fn arrays(){
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[2] = 0;
    println!("a: {a:?}");
}

fn tuples(){
    let t: (i8, bool) = (7, true);
    dbg!(t.0);
    dbg!(t.1);
}

fn loop_arrays(){
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
}

fn check_order(tuple: (i32, i32, i32)) -> bool {
    let (left, middle, right) = tuple;
    left < middle && middle < right
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3{
        for j in 0..3{
            result[j][i] = matrix[i][j];
        }
    }    
    result
}

fn shared_references(){
    let a = 'A';
    let b = 'B';

    let mut r: &char = &a;
    dbg!(r);

    r = &b;
    dbg!(r);
}

fn exclusive_references(){
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:?}");
}

fn slice(){
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];

    println!("s: {s:?}");
}

fn strings(){
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");

    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[2..9];
    println!("s3: {s3}");
}

/* 
fn reference_validity(){
    let x_ref = {
        let x = 10;
        &x
    };
    dbg!(x_ref);
}
*/

// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.
/* 
fn magnitude(v: &[f64]) -> f64 {
    let mut mag_squared = 0.0;
    for coord in v {
        mag_squared += coord * coord;
    }
    mag_squared.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.
fn normalize(v: &mut [f64]) {
    let mag = magnitude(v);
    for item in v {
        *item /= mag;
    }
}
*/

fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut mag_squared = 0.0;
    for coord in vector {
        mag_squared += coord * coord;
    }
    mag_squared.sqrt()
}

/// Change the magnitude of the vector to 1.0 without changing its direction.
fn normalize(vector: &mut [f64; 3]) {
    let mag = magnitude(vector);
    for item in vector {
        *item /= mag;
    }
}

struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

// tuple structs
struct Point(i32, i32);

// enum
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

// type alias
enum CarryableConcreteItem {
    Left,
    Right,
}

type Item = CarryableConcreteItem;

// Aliases are more useful with long, complex types:
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;

// const
const DIGEST_SIZE: usize = 3;
const FILL_VALUE: u8 = calculate_fill_value();

const fn calculate_fill_value() -> u8 {
    if DIGEST_SIZE < 10 { 42 } else { 13 }
}

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [FILL_VALUE; DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

// Static variables will live during the whole execution of the program, and therefore will not move
static BANNER: &str = "Welcome to RustOS 3.14";

// elevator exercise 

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    /// A button was pressed.
    ButtonPressed(Button),

    /// The car has arrived at the given floor.
    CarArrived(Floor),

    /// The car's doors have opened.
    CarDoorOpened,

    /// The car's doors have closed.
    CarDoorClosed,
}

/// A floor is represented as an integer.
type Floor = i32;

/// A user-accessible button.
#[derive(Debug)]
enum Button {
    /// A button in the elevator lobby on the given floor.
    LobbyCall(Direction, Floor),

    /// A floor button within the car.
    CarFloor(Floor),
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}

fn main() {    
    // Morning
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

    // Afternoon
    arrays();
    tuples();
    loop_arrays();

    let tuple = (1, 5, 3);
    println!(
        "{tuple:?}: {}",
        if check_order(tuple) { "ordered" } else { "unordered" }
    );

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    dbg!(matrix);
    let transposed = transpose(matrix);
    dbg!(transposed);
    
    shared_references();
    exclusive_references();
    slice();
    strings();
    //reference_validity(); // uncomment to see the error

    // exercise geometry
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));

    // named structs
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    describe(&peter);
    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);


    // tuple structs
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);

    // enum
    let dir = Direction::Up;
    let player_move: PlayerMove = PlayerMove::Run(dir);
    println!("On this turn: {player_move:?}");

    // const
    let digest = compute_digest("Hello");
    println!("digest: {digest:?}");

    // static
    println!("{BANNER}");

    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}


fn def_vars2(){
    let mut x: i32 = 30;
    println!("x: {x}");
    x = 40;
    println!("x: {x}");
}