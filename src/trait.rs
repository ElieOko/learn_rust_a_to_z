trait Affichable{
    fn afficher(&self);
}
impl Affichable for i32{
    fn afficher(&self){
        println!("value {}",self);
    }
}
//or impl par defaut
trait Affichable2{
    fn afficher2(&self) where Self: std::fmt::Display{
        println!("is number : {}",self)
    }
}

impl Affichable2 for i32{}

//trait associés comporte des methodes qui sont appelés depuis le type
trait Utilitaire{
    fn ajouter(a:i32, b:i32) -> i32;
}
impl Utilitaire for i32{
    fn ajouter(a:i32, b:i32) -> i32{
        a + b
    }
}

fn main(){
    let i : i32 = 100;
    i.afficher();
    i.afficher2();
    let x : i32 = 12;
    let y : i32 = 14;
    let sum = i32::ajouter(x,y);
    println!("Somme :{}",sum);
}