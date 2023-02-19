pub fn lesson7() {
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




