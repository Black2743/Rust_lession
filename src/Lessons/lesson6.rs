pub fn lesson6() {
    lesson6_part1(32, 15);
    println!("{}", lesson6_part2(1, 9));
    let a = lesson6_part3(32, 18);
    println!("{:?}", a);
}

fn lesson6_part1(a: i32, b: i32) {
    println!("{}", a + b);  //наз. инструкция
}

fn lesson6_part2(a: i32, b: i32) -> i32 {
    a + b  //наз. выражение или можно написать return a+b;
}

fn lesson6_part3(a: i32, b: i32) -> (i32, i32) {
    return (a + b, a * b); //наз. выражение или можно написать return a+b;
}

