fn main(){

    let str_a:&str  = "Hello World!";
    let b:[i32;5] = [1,3,5,7,10];
    let sliceb :&[i32] = &b;

    let pstr_a : *const str = str_a as *const str;
    let psliceb : *const i32 = &sliceb[0] as *const i32;
    

        

    println!("{}",str_a);
    println!("{:?}",b);
    println!("{:?}",sliceb);
    
    println!("{:?}",pstr_a);
    println!("{:?}",psliceb);


}