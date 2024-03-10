fn main() {
    //Declaration Implicite Variable Immuable(Immutable)to immutable variable
    let _old = 100;
    //Declaration Implicite Variable Muable(Mutable)
    let mut _taux = 2750;
    _taux = 2800;
    /*
     ****Declaration Explicite variable Immuable entier signé****
     */
    //i8 min -128, max 128
    let _temperature: i8 = 127;
    //i16 min -32768, max 32767
    let _vitesse: i16 = 10_291;
    //i32 min -2147483648, max 2147483647
    let _distance: i32 = 40_000_000;
    //i64 min -((2^64)/2),max ((2^64)/2) - 1
    let _budget: i64 = 140_000_000_000_0000_000;
    //i128 min  -((2^128)/2),max ((2^128)/2) - 1
    let _blockchain_case_lambda: i128 = 10_000_000_000_000_000_000_000_000_000_000;
    //isize min -9223372036854775808, max 9223372036854775807
    let _large_max: isize = 9_223_372_036_854_775_807;
    /*
     ****Declaration Explicite variable Immuable entier non signé****
     */
    //u8 max 2^8
    let _age: u8 = 19;
    //u16 max 2^16
    let _vitesse: u16 = 10_291;
    //u32 max 2^32
    let _distance_parc: u32 = 40_000_000;
    //u64 max 2^64
    let _fortune: u64 = 140_000_000_000_0000_000;
    //u128 max 2^128
    let _blockchain_case_crack: u128 = 10_000_000_000_000_000_000_000_000_000_000;
    //isize
    let _large_max: usize = 9_223_372_036_854_775_807;
    /*
     ***Declaration Explicite variable Mutable char
     */
    let mut _etat_civil: char = 'C';
    _etat_civil = 'M';
    /*
     ****Declaration Explicite variable Mutable string
     */
    let mut _language_system: String = String::from("C");
    // a conversion with method to_string()
    _language_system = "RUST".to_string();
    /*
     ****Declaration Explicite variable Immuable float
     */
    let _taille: f32 = 1.89;
    let _superficie: f64 = 23813920.122;
    /*
     ***Declaration Explicite variable Mutable bool
     */
    let mut _is_active: bool = true;
    _is_active = false;
    /*
     ***Masquage(shadow) variable
     */
    let mut _price = 250;
    {
        let _price = _price + 2; //_price output 252
    } //_price output 250
}
