fn fatal (message: &str){
    let error_message :&str = "致命的なエラー";

    //error_message = error_message +message;
    eprintln!("{}:{}",error_message,message);

    std::process::exit(1);

}

