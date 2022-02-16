fn main (){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <パスワード>", args[0]);
        std::process::exit(1);
    }

    if check_authentication(&args[1]) {

        println!("=====================");
        println!("アクセスを許可します");
        println!("=====================");
    }
    else {
        println!("=====================");
        println!("アクセスを拒否します");
        println!("=====================");
    }

}

fn check_authentication(password: &str)-> bool{
    let password_buffer:String = password.to_string();
    let mut auth_flg : bool = false;
    

    if password_buffer == "brillig"{
        auth_flg = true;
    }

    if password_buffer == "outgrade"{
        auth_flg = true;
    }
    
    return auth_flg;
    

}
