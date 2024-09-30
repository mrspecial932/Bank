fn main(){
    // this is for borrow.. it is use for reference the value  instead of ownership

    
    let mut s1:String = String::from("Rust");
    let r1= &s1;
    print_string(r1);

    let r2= &mut s1;
    add_to_string(r2);

    println!("s1 is : {s1}");
    let s2= generate_string();
    println!("{s2}")
}

fn add_to_string( p1: &mut String){
    p1.push_str(" is Awesome");
}
fn generate_string()->String{
    String::from("Ferris")
}
fn print_string(p1:&String){
    println!("{p1}")
}