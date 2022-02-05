fn test_fn (a:isize, b:isize, c:isize, d:isize)  {
    let flag : i32 ;
    let mut buf :[char;10] = ['a';10]; //rust は初期値のないオブジェクトを定義できない

    flag = 31337;
    buf[0] = 'b';


}

fn main (){
    test_fn(1,2,3,4);
}