fn main(){
    let mut i = 0;
    let title = "--SHUT--";
    let content = "#@@FLAG@@#";
    // Loop no condition 
    loop {
        i+=1;
        println!("****{}****",i);
        if i == 10 {
            println!("{}", title);
            break;
        }
    }
    // While with condition
    while i < 20 {
        println!("___{}___", i);
        if i == 18 {
            println!("{}", content);
            break;
        }
        i+=1;
    }

    let tab = [2,9,4,7,8,1,0];
    //for iteration into collection
    println!("****");
    for indice in tab {
        println!("{}",indice);
    }
    println!("@@@@@@");
    //~||~
    for nombre in (1..8).rev(){
        println!("{}",nombre);
    }

    //iter().enumerate() destructuration index and element
    for (index,element) in tab.iter().enumerate(){
      println!("a[{}]=>{}",index, element)  
    }
    // detructuration directe avec tuple
    let _tab_collection = vec![(1,"ElieOko"),(2,"MusimbiTerence"),(3,"EthyMuzola")];
    for (id,name) in _tab_collection{
        println!("id:{}\nname:{}",id,name);
    }










}