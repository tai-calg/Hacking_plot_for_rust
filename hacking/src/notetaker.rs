mod hacking;
//use libc::*;
use std::io::Write;
use std::{path::Path};
use std::fs::{File, OpenOptions};
use std::fs;
use users::{Users, Groups, UsersCache};

fn main (){
    let args: Vec<String> = std::env::args().collect();

    let buf = Box::new(args[1].clone());
    let datafile = "/var/notes";

    // open file
    let path = Path::new(datafile);
    //let display = path.display(); //描画できるあんぜんなコードに変換
    let mut fd  = match File::open(&path){
        Err(_) => File::create(datafile).unwrap(),
        Ok(_) => OpenOptions::new().write(true).open(&path).unwrap(),
    };

    
    // write to file
    //メモの前にユーザーIDを書き込む
    let userid = UsersCache::new().get_effective_uid(); //[bug] joseで実行しても1000を返さない。
    println!("{}", userid);
    fd.write(&userid.to_le_bytes()).unwrap(); //書き込み、詰まった場所
    fd.write(b"\n").unwrap();
    println!("userid :{:?}", userid.to_be_bytes());


    fd.write(&buf.as_bytes()).unwrap();
    fd.write(b"\n").unwrap();
    println!("contents: {:?}", buf.as_bytes());


    fd.write(&buf.len().to_be_bytes()).unwrap();
    fd.write(b"\n").unwrap();
    println!("length of contents: {}", buf.len());
    
    //close file
    println!("メモが保存されました");

let mut cache = UsersCache::new();
let uid = cache.get_effective_uid();
let user = cache.get_user_by_uid(uid).unwrap();
println!("Hello again, {} with {}!", user.name().to_string_lossy(),uid);

    




    /*
    unsafe{
        let buffer : *const c_char = libc::malloc(100) as *const c_char;
        if buffer.is_null(){
            eprintln!("メモリの確保に失敗しました");
        }
        //libc::free(buffer as *mut c_void);

        let mut data_file = libc::malloc(20) as *mut c_char;
        let val = "/var/notes";
        //let var: *const c_char = &val as *const c_char;
        libc::strcpy(data_file, val as *const c_char);

    }
*/

}