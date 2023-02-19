pub fn lesson2() {
    let mut a: i32 = 31;
    let mut b: i32 = 31;
    if a == b
    {
        println!("true");
    } else {
        println!("false");
    }
    a += b;
    match a {
        1 => println!("a=1"),
        _ => println!("no value"),
    }
}
