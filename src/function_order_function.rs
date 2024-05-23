fn double(x:i32)->i32{
    x * 2
}

fn main(){
    //variable function
    let doublure = double;
    let result_double = doublure(4);
    println!("{}", result_double);
}
