
fn main (){
    let args: Vec<String> = std::env::args().collect();
    //println!("{:?}",args);

    if args.len() < 3 {
        usage(&args[0]);

    }

    for i in 0..args[2].parse::<i32>().unwrap() {
        println!("{}回目: {}", i,args[1]);

    }

}

fn usage(program_name: &String){
    println!("文法が違います");
    println!("Usage: {} <メッセージ> <繰り返し回数>", program_name);
    std::process::exit(1);
}
