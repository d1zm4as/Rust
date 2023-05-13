// use std::collections::{HashMap, hash_map};



// fn fib(num:i32)->i32{
//     let mut memo: HashMap<i32,i32>=HashMap::new();
//     memo.insert(1,1);
//     memo.insert(2,1);

//     if memo.contains_key(&num){
        
//         return memo.get(num);
//     }
//     if num<=2{
//         return 1;
//     }else{
//         return fib(num-1)+fib(num-2);
//     }



// }
fn fib(num:i32)-> i32{if num<=2{return 1;}return fib(num-1)+fib(num-2);}


fn main() {
    println!("{}",fib(1));
    println!("{}",fib(2));
    println!("{}",fib(3));
    println!("{}",fib(4));
    println!("{}",fib(5));
    println!("{}",fib(6));
    println!("{}",fib(7));
    println!("{}",fib(8));
    println!("{}",fib(9));



}
