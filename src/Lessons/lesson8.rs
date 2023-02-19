fn test8_new(A: i32, B: i32, operation: fn(&str)) {
    println!("{}", A + B);
    operation("Вызов operation");
}

fn test8(stroka: &str) {
    println!("{}", stroka);
}

fn lesson8(i: i32) {
    let cube = |i: i32| {   //анонимные выражения
        println!("Куб числа {} равен {}", i, i * i * i);
    };
    cube(i);
    test8("кубик");
    let new_test = test8; //копия метода
    new_test("2 кубик");
    test8_new(4, 7, new_test);
}

