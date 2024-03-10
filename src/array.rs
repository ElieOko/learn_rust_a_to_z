fn main() {
    /*
     **** Array string
     */
    let mut _collection_language = [
        "C",
        "RUST",
        "KOTLIN",
        "TypeScript",
        "JavaScript",
        "Java",
        "Python",
        "PHP",
    ];
    //taille array
    let _size_collection_language = _collection_language.len();
    // Tri
    _collection_language.sort();
    println!("{:?}", _collection_language);
    // Slice array
    let _first = &_collection_language[..2];
    // println!("{:?}", _first);
    // println!("{:?}", _collection_language);
    /*
     ***Vecteur
     */
    //Vecteur vide
    let mut _items_article = Vec::new();
    //append new value into vecteur
    _items_article.push("Tomato");
    _items_article.push("Milk");
    _items_article.push("Rice");
    //remove element with position
    _items_article.remove(0);
    println!("{:?}", _items_article);
    //vecteur one dimension
    let _collection_cote: Vec<i32> = vec![19, 18, 16, 15, 20, 13];
    //Vecteur mutable two dimension
    let mut _item_stock = vec![vec![0; 4]; 4];
    _item_stock[2][2] = 5;
    println!("{:?}", _item_stock);
}
