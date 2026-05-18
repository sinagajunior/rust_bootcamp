use std::rc::Rc;
use std::cell::RefCell;



fn main() {
    println!("Memory management Demo in Rust !");


    // Ownership example
    let s1 = String::from("Ownership example");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    println!("Ownership transfered {}", s2);
   // println!("s1 is no longer valid: {}", s1); // This will cause a compile error

    // Borrowing example
    let s3 = String::from("Borrowing example");
    borrow_demo(&s3); // s3 is borrowed, not moved
   println!("After borrowing, s3 is still valid: {}", s3);

    // Mutable browong example
   let mut s4 = String::from("Hello");
   mutable_demo(&mut s4);
   println!("After mutable demo: {}", s4);

   // Lifetimes 
   let result;
   let a = String::from("Abcd"); 
    {
     let b = String::from("xyz");
     result = longest(&a,&b);
     println!("Longest string is: {}", result);
    }

    //Box (heap allocation)
       let boxed = Box::new(42);
       println!("Boxed value: {}", boxed);

    
    //Rc (reference counting pointer)
    let rc_val = Rc::new(String::from("Shared"));
      let rc_clone = Rc::clone(&rc_val);
    println!("Rc values: {},{}",rc_val,rc_clone);
    println!("Reference count: {}", Rc::strong_count(&rc_val));
     
     
     // RefCell (interior mutability)
     let cell = RefCell::new(100);
      *cell.borrow_mut() +=50;
      println!("RefCell value: {}", cell.borrow()); 
}

fn borrow_demo(data: &String){
    println!("Borrowed data: {}", data);
}

 fn mutable_demo(data: &mut String) {
    data.push_str(" World");
 }

 fn longest<'a>(x: &'a str, y:&'a str)-> &'a str {
    if x.len() > y.len() { x } else { y }
 }



