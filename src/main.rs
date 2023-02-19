use std::{io, collections::hash_map::RandomState};

fn main() {
    lesson8(67);
    lesson5();
}


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


fn lesson7() {
    let mut a = String::from("TEXT");
    lesson7_part2(&mut a); //передача ссылки в функцию(обязательно mut, для того чтобы изменить)
    println!("{}", a);
}

fn lesson7_part2(str: &mut String) {
    str.push_str(" end");
}

fn bubble_sort(arr: &mut [i32]) {
    for _ in 0..arr.len() {
        for j in 1..arr.len() {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
            }
        }
    }
}

fn lesson6() {
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

/*fn lesson5() {  //кортеж
    let tuples = (3, 15.6, "S");
    print!("{:?}", tuples);
    let (int_value, float_value, string_value) = tuples;
    println!("{},{},{}", int_value, float_value, string_value);
    let int_secondValue = tuples.0;
}*/





