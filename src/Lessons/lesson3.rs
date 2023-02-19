
pub fn lesson3() {
    let mut name = String::new();
    println!("Set your name");
    match io::stdin().read_line(&mut name) { //обработка ошибки при неверном значении
        Ok(_) => {
            println!("Hello {}", name.trim());
        }
        Err(e) => {
            println!("error {}", e);
        }
    }
    let mut age_str = String::new();
    println!("Set your age");
    match io::stdin().read_line(&mut age_str) { //обработка ошибки при неверном значении
        Ok(_) => {
            println!("You age {}", age_str.trim());
        }
        Err(e) => {
            println!("error {}", e);
        }
    }
    let mut age: i32 = age_str.trim().parse().unwrap();  //trim уберает пробелы, unwrap вскрывает строку чтоб взять число
    age = 2023 - age;
    println!("Year of birth: {}", age);
}
