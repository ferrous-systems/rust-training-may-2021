#[allow(arithmetic_overflow)]

use std::mem::size_of;

fn main() {
    println!("Hello, class!");
    // let x = Arc::new(3);
    let y: u64 = foo();

    let a = [42u8, 123, 55, 76];

    let mut b = 255u8;
    b = b.saturating_add(1);

    //                   00  01  02  03
    let arr: [i32; 4] = [10, 20, 30, 40];
    let sli = &arr[..3];
    // ptr to the first element
    // valid length

    // size in bytes of ONE instance of an Actions enum
    // ???
    // 2 * size_of::<i32>() + 2
    // 2 * size_of::<i32>() + 1?
    // size_of::<i32>() == 4
    // 12
    println!("{}", size_of::<Actions>());

    // 4
    // 0
    println!("{}", size_of::<()>());

    let x = String::from("Hello");

    // String "hello" is OWNED by x, which is an
    // immutible binding of the data.

    let mut y = x;

    // String "hello" is OWNED by y, which is a
    // mutable binding of the data

    let z = y;

    // String "hello" is OWNED by z, which is an
    // immutible binding of the data.

    let a = HoldsStuff {
        x: 5, y: 10
    };

    let mut b = HoldsStuff {
        x: 50, y: 100
    };
}

fn foo() -> u64 {
    1_000_000
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct HoldsStuff {
    x: u32,
    y: u32,
}

fn enums() {
    let direction: Direction = Direction::Left;
}

enum Actions {
    StickAround,
    MoveTo { x: i32, y: i32},
    Silly1([u8; 1000]),
    Silly2([u8; 1000]),
    Silly3([u8; 1000]),
}


// PSUEDOCODE
/*
struct Actions {
    variant: u32,           // 4 bytes
    values: union {
        (),                 // 0 bytes
        { x: i32, y: i32 }, // 8 bytes
        [u8; 1000],
    }
}
*/

fn prints_but_returns_nothing(data: &str) -> () {
    println!("passed string: {}", data);
}

// immutible - "read only"
// mutable   - "read/write"
