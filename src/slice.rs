fn main(){
    //Slices are refernce to a contingous sequernce 
    //element in a collection

    let tweet: String= String:: from( "this is my tweet and its very very long");

    let trimmed_tweet:&str =trim_tweet(&tweet);
       
    let tweet2:&str=  "this is my tweet and its very very long";
    let trimmed_tweet2:&str =trim_tweet(tweet2);

   
    println!("{trimmed_tweet}");
    println!("{trimmed_tweet2}");


    let a:[i32 ; 6] =[1,2,4,5,3, 6];
    let a_slice=&a[..3];
    println!("{:?}", a_slice);
}
fn trim_tweet(tweet:&str) ->&str {
    &tweet[..10]
}