fn main(){
    let name = "scrum";
    let mut _data_by_key = "uid";
    //Ombrage
    {
        //move transfert de proprièté
        let name_change = name;
        //transmission de la valeur avec précision du pointeur
        _data_by_key = &name_change;
    }
    //name est liberé au niveau de la memoire
    println!("___{}___", _data_by_key);
    println!("****************************");
    //partage de la memoire avec avec reference
    let chaine_copy = _data_by_key;
    println!("ref valeur :{}\nPointeur parent valeur:{}",chaine_copy, _data_by_key);
    //copie profonde(copy deep) associer au niveau de la reservation memoire nouvelle reservation dans une zone de la memoire
    let origin_value = String::from("Microservice");
    let chaine_deep_copy = origin_value.clone();
    println!("___{}___", chaine_deep_copy);

    //copy un implementer uniquement avec le type numerique elle se fait automatiquement
    //shalow copy (copy superficielle)
    let n1 = 42;
    let n2 = n1;
    println!("{}\n{}",n1,n2);

}


