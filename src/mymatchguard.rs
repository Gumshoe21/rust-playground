pub fn test_matchguard() {
    let pair = (2, -2);
    println!("Tell me about {pair:?}");
    match pair {
        (x, y) if x == y => println!("x == y"),
        (x, y) if x + y == 0 => println!("x + y == 0"),
        (x, y) if x % 2 == 1 => println!("x % 2 == 1"),
        _ => println!("_"),
    }
}

pub fn test_unreachable() {
    let number: u8 = 4;
    match number {
        i if i > 0 => println!("i > 0"),
        i if i == 0 => println!("i == 0"),
        i if i < 0 => println!("i < 0"),
        // _ => unreachable!("Should never happen."),
        // ^ uncomment to fix compilation
    }
}
