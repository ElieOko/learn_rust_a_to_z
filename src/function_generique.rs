fn get_bigger_number<T>(x : T, y : T) -> T
where 
    T: std::cmp::PartialOrd,
{
    if x > y{
        x
    }
    else{
        y
    }
}
fn afficher_contenu<T>(vecteur : Vec<T>)
where
    T: std::fmt::Debug,
{
    for element in &vecteur{
        println!("{:?}",element);
    }
}
// or
fn afficher_contenu2<T: std::fmt::Debug>(vecteur : Vec<T>){
    for element in &vecteur{
        println!("{:?}",element);
    }
}

fn main(){
    let a = 4;
    let b = 8;
    let max = get_bigger_number(a,b);
    println!("{}",max);
    println!("_____________");
    let tab = vec![48,23,84,20,94,3];
    let tab_ref_origin = &tab;
    afficher_contenu(tab_ref_origin.to_vec());
    println!("*************");
    afficher_contenu2(tab);
} 