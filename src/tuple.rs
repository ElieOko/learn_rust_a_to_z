fn main() {
    let user: (String, u8, bool, String) =
        (String::from("Elie"), 19, true, String::from("Developper"));
    let (_name, _age, _is_active, _profession) = &user;
    println!("{}-|{}|-{}-{}", _name, _age, _is_active, _profession);
    //access into tuple with indice
    println!("Username :{}\nOld :{}\n", user.0, user.1);
}
