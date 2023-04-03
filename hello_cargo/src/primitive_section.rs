
    fn main(){
        println!("Og");
        let a : u16 = 10;
        let d : u8 = 30;
        let twenty_one : i32 = 21;
        let twenty_two = 21i32;
        let create_array = [1,412,51i32, 53];
        let one_milion = 1_000_000i64;
        let i = -24;
        println!("{}, {}", twenty_one, twenty_two);
        println!("{}",one_milion.pow(2));
        println!("{}", i);
        println!("{}", create_array[2]);
        if a>( d as u16) {
           println!("Hy {} and {} are much more", a, d); 
        }
}

