fn main() {
  println!("===== Memory Allocation");
  {
    // String literal - a know fixed size variable allocated on the stack, it is fast, but immutable
    let _s = "hello";
    // String type - the size can be adapted in runtime, allocated on a heap. It is slower, nut mutable.
    // when we call String::from, its implementation requests the memory it needs
    // The double colon (::) is an operator that allows us to namespace this particular from function under the String type
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
  } // this scope is now over, and s is no longer valid
  //Rust calls drop automatically at the closing curly bracket.

  println!("===== Copy concept");
  {
    // When talkong about variables in stack memory, theri is no difference between shallow and deep copy
    // bind the value 5 to x
    let x = 5;
    // make a copy of the value in x and bind it to y
    let y = x;
    // This is indeed what is happening, because integers are simple values with a known,
    // fixed size, and these two 5 values are pushed onto the stack.
    println!("x = {}, y = {}", x, y);
  }

  println!("===== Move concept");
  {
    // When allocating part of memory on the heap
    // the value of s1 gets places in a heap memory
    // but group of data which describes the allocated memory is stored on stack
    // Stack -> s1(ptr, len, capacity)
    // Heap -> value
    let s1 = String::from("hello");
    // When assigning s1 to s2 only the data which is stored on stack is copied
    // pointing to the same memory location on the heap
    let s2 = s1;
    // when s2 and s1 go out of scope, they will both try to free the same memory.
    // This is known as a double free error and is one of the memory safety bugs we mentioned previously.
    // Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
    // Solution: After let s2 = s1, Rust considers s1 to no longer be valid.
    // this will not work -> println!("{}, world!", s1);
    println!("{}, world!", s2);
  }

  println!("===== Clone concept");
  {
    // If we do want to deeply copy the heap data of the String, not just the stack data,
    // we can use a common method called clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
  }

  println!("===== Ownership and Functions");
  {
    // Calling a function transfares ownership
    fn takes_ownership(some_string: String) {
      println!("{}", some_string);
    }
    
    
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves intothe function ...
    // ... and so is no longer valid in this scope
  
    fn makes_copy(some_integer: i32) {  // some_integer comes into scope
      println!("{}", some_integer);
    } // Here, some_integer goes out of scope.
  
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy,
    // so it's okay to still use x afterward
  } // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

  println!("===== Retunrning the Ownership");
  {
    // Returning values from a function can also transfer ownership.
    fn gives_ownership() -> String {
      let some_string = String::from("hello"); // some_string comes into scope
      some_string // some_string is returned and moves out to the calling function
    }

    let s1 = gives_ownership(); // gives_ownership moves its return value to s1
    println!("{}", s1);

    fn takes_and_gives_back(a_string: String) -> String {
      a_string
    }

    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("{}", s3);
  } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

  println!("===== Retunrning multiple values");
  {
    fn calculate_length(s: String) -> (String, usize) {
      let length = s.len(); // len() returns the length of a String
      (s, length)
    }

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
  }
  // But this is too much ceremony and a lot of work for a concept that should be common.
  // Luckily for us, Rust has a feature for this concept, called references.

  println!("===== Reference and borrowing");
  {
    // & - reference operator
    // * - dereference operator
    // These ampersands means that function takes references of a variable as an argument
    fn calculate_length(s: &String) -> usize {
      s.len() // s would be a pointer to a pointer in C
    }

    let s1 = String::from("Hello");
    // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
    // Because it does not own it, the value it points to will not be dropped when the reference stops being used.
    // We call the action of creating a reference borrowing
    let len = calculate_length(&s1);
    // Just as variables are immutable by default, so are references
    // You cannot alter what is borrowed
    println!("s1 = {}, len = {}", s1, len );
  }

  println!("===== Mutable References");
  {
    // But mutable references have one big restriction: you can have only
    // one mutable reference to a particular piece of data at a time.
    // The benefit of having this restriction is that Rust can prevent data races at compile time. 
    fn change(some_string: &mut String){
      some_string.push_str(" Eyo");
    }

    let mut s = String::from("Hello");
    // the change function will mutate the value it borrows.
    change(&mut s);
    println!("s1 = {}", s);
  }

  println!("===== Multiple Mutable references");
  {
    let mut s = String::from("Hello");
    {
      let _r1 = &mut s;
    } // r1 goes out of scope, so we can meke new reference with no problem
    let _r2 = &mut s;
  }

  println!("===== Mutable and immutable references together");
  {
    // We also cannot have a mutable reference while we have an immutable one.
    let s = String::from("Hello");
    let _r1 = &s; // no problem
    let _r2 = &s; // no problem
    // let _r3 = &mut s; // BIG PROBLEM
    println!("{}, {}, {}", _r1, _r2, s);
    // Users of an immutable reference don’t expect the values to suddenly change out from under them!
    // However, multiple immutable references are okay because no one who is just reading the data
    // has the ability to affect anyone else’s reading of the data.

    {
      // a reference’s scope starts from where it is introduced
      // and continues through the last time that reference is used. 
      let mut s = String::from("hello");

      let r1 = &s; // no problem
      let r2 = &s; // no problem
      println!("{} and {}", r1, r2);
      // variables r1 and r2 will not be used after this point
  
      let r3 = &mut s; // no problem
      println!("{}", r3);
    }
  }

  println!("===== Dangling reference");
  {
    // a dangling pointer, a pointer that references a location in memory that may have been
    // given to someone else, by freeing some memory while preserving a pointer to that memory
    // In Rust, the compiler guarantees that references will never be dangling references:
    // if you have a reference to some data, the compiler will ensure that the
    // data will not go out of scope before the reference to the data does.

    //fn dangle() -> &String { // dangle returns a reference to a String      
    fn dangle() -> String { // solution is to return String directly
      let s = String::from("Salamaleikum"); // s is a new String
      s // we return a reference to the String, s
      // &s // we return a reference to the String, s
    } // Here, s goes out of scope, and is dropped. Its memory goes away.

    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing)

  }
  // Let’s recap what we’ve discussed about references:
  // * At any given time, you can have either one mutable reference or any number of immutable references.
  // * References must always be valid.
}
