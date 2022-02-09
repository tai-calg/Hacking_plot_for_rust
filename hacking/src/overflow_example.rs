
fn main (){
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <message>", args[0]);
        std::process::exit(1);
    }

    unsafe{
        //let mut buf1:[char;8] ;
        //let pbuf1 = &mut buf1 as *mut [char;8];
        //let mut buf2:[char;8] ;
//
        //libc::strcpy(pbuf1,"one");
        //let mut buf1: * mut &str = libc::malloc(8) as *mut &str; //8bytes
        //let mut buf2: * mut &str = libc::malloc(8) as *mut &str; //8bytes
        let mut num = [2,4,6,10];
        let num2 = [1,3,7,11];
        let buf1 = &mut num as *mut i32;
        let buf2 = & num2 as *const i32;
        //*buf1.offset(4) = 999 as i32; //let num2のインデックス０に999を代入（脆弱性）
        *buf1.offset(4) = (args[1].clone() as String).parse::<i32>().unwrap() ;

        
        

        println!("buf1は {:p}番地で、値は{}です",buf1,*buf1);
        println!("buf2は {:p}番地で、値は{}です",buf2,*buf2);
    }
    
    //*buf1 = args[1].as_str();

}