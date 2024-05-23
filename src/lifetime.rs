fn main(){
    let a : i32= 12;
    let b : i32= 24;
    let result : &i32 = get_bigger_number(&a, &b);
    println!("{}",result);
}

fn get_bigger_number<'a, 'b:'a>(x : &'a i32, y: &'b i32) -> &'a i32{
    if x > y{
        x
    }
    else{
        y
    }
}