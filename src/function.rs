fn main() {
    let mut name = String::new();
    let mut username = String::new();
    let _a = 12;
    let _b = 13;
    let result = addition(_a, _b);
    // println!("{0} + {1} = {2}", _a, _b, result);
    // println!("Writte your name please:");
    // std::io::stdin()
    //     .read_line(&mut name)
    //     .expect("Exception detecté");
    // greeting(&name);
    username = "elds";
    let mut name = transform_user(username);
    println!("{}",name);
}

//Fonction retournant une valeur en retour
fn addition(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

//fonction ne retournant pas de valeur avec un parametre de reference
fn greeting(name: &str) {
    // let x = side_effect;
    println!("Hi {}", name);
}

fn transform_user(mut name : &str) -> &str{
    name = "ElieOko";
    name
}