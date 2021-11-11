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
}
