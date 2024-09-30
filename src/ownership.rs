fn main(){
    let s1:String = String::from("Rust");
    let s2:String = s1.clone();


    println!("S1 is {s1}");

    //calling of a function and set the ownership to the fuction
    print_string( s1.clone());

    let s3:String = generate_string();
    println!("s3  is :{s3}");

    let s4:String= add_to_string(s2);
    println!("s4 is {s4}");

    let x:i32 = 10;
    print_intefer(x);
    
}
fn print_intefer(i:i32){
    println!("i is :{i}")
}
fn add_to_string(mut p1:String)->String{
    p1.push_str(" is Awesome");
    p1
}
fn generate_string()->String{
    String::from("Ferris")
}
fn print_string(p1:String){
    println!("{p1}")
}