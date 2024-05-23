fn main(){
    let mut s : String = String::from("Hello");
    //emprunt mutable un seul
    let r1 : &mut String = &mut s;
    r1.push_str(", world");
    println!("{}", s);
    //emprunt immuable
    let n = String::from("borrowing");
    let x = &n;
    let y = &x;
    println!("{}\n{}\n{}",x,y,n);
}