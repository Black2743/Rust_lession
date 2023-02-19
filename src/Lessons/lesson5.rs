fn lesson5(){  //кортеж
    let tuples =(3,15.6,"S");
    print!("{:?}",tuples);
    let (int_value,float_value,string_value)=tuples;
    println!("{},{},{}",int_value,float_value,string_value);

}