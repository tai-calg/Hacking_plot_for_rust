fn main (){
    let a = Box::new(5);
    let pa : * const Box<i32>= &a as *const Box<i32>;
    println!("addr :{:?}",pa);
    println!("val :{}",a);
}