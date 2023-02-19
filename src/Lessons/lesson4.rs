fn lesson4() {  //  ARRAY
    let arrray = [1, 2, 3, 4, 5, 6];
    let arr = [15; 45];  // заполняяет массив числом 15 45 раз
    println!("{:?}", arrray);  //  :? вывод массива
    println!("{:?}", arr);

    for i in arrray.iter() {  //прохождение по элементам массива через for
        println!("{}", i);
    }
}
fn lesson4_part2()
{
    // let mut array:[i32;5];
    // for i in 0..array.len()
    // {
    //     let mut buf:String=String::new();
    //     match io::stdin().read_line(&mut buf){
    //         Ok(_)=>{
    //             array[i]=buf.trim().parse().unwrap();
    //         },
    //         Err(e)=>{
    //             println!("error {}",e);
    //         }
    //     }
    // }
    // println!("{:?}",array);
}

