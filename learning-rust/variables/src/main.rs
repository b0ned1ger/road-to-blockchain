fn main() {
  do_variables();
  do_data_types();

  println!(" ===== Functions");
  // Functions
  // snake case - conventional style for function and variable names. All letters are lowercase and underscores separate words
  {
    // definition of a function
    fn another_function(x: i32) {
      println!("The value of function parameter is: {}", x);
    }
    // a call to a function
    another_function(5);

    fn print_labeled_measurement(value: i32, unit_label: char) {
      println!("The measurement is: {}{}", value, unit_label);
    }

    print_labeled_measurement(5, 'h');
  }

  println!(" ===== Function statements and expresions");
  {
    // Statements do not return values (statement - let x = 5;)
    // Therefore, you can’t assign a let statement to another variable
    // Expressions evaluate to a value (expression - 5 + 6)
    // Expressions can be part of statement.
    let y = {
      let x = 3;
      x + 1
    };
    // The block that we use to create new scopes, {}, is an expression
    // { let x = 3; x + 1 } is an expression
    println!("The value of y is: {}", y);
  }

  println!(" ===== Functions with Return values");
  {
    // we don’t name return values, but we do declare their type after an arrow (->)
    // You can return early from a function by using the return keyword and specifying a value,
    // but most functions return the last expression implicitly
    fn five() -> i32 {
      5
    }

    let x = five();
    println!("The value of x is: {}", x);

    fn plus_one(x: i32) -> i32 {
      x + 1
    }
    let x = plus_one(5);
    println!("The value of x is: {}", x);
  }

}

// Note that we defined another_function after the main function in the source code
// Rust doesn’t care where you define your functions, only that they’re defined somewhere.
fn do_variables() {
  println!(" ===== Variables - immutable by default");
  // immutable variable can only get assigned once, value of x cannot be changed
  let x = 5;
  println!("The value of x is {}, and it cannot be changed!", x);
  // mutable variable can get reassigned
  let mut y = 5;
  println!("The value of y is: {}", y);
  y = 6;
  println!("The value of y is: {}", y);

  println!(" ===== Constants - allways immutable");
  // * bound to a name and are not allowed to change;
  // * constants may be set only to a constant expression, not the result of a value that could only be computed at runtime;
  // * Constants can be declared in any scope, including the global scope;
  // * the type of the value must be annotated;
  // * use all uppercase with underscores between words;
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  println!("This is a constant {}!", THREE_HOURS_IN_SECONDS);

  println!(" ===== Shadowing");
  // * declaring a new variable with the same name as a previous variable
  // * first variable is shadowed by the second

  // binds x to a value of 5
  let x = 5;

    // shadows x by taking the original value and adding 1
    let x = x + 1;

    // an inner scope
    {
      // shadows x by multiplying the previous value by 2
      let x = x * 2;
      println!("The value of x in the inner scope is: {}", x);
    }

  // When that scope is over, the inner shadowing ends
  println!("The value of x is: {}", x);

  // Shadowing enables to change the value of a varialbe
  // string type
  let spaces = "   ";
  println!("The value of spaces is: {}", spaces);
  // number type
  let spaces = spaces.len();
  println!("The value of spaces is: {}", spaces);
}

fn do_data_types(){

  println!(" ===== Scalar types - integers, floating-point numbers, Booleans, and characters");
  {
    // Integer - An integer is a number without a fractional component
    // * signed integer types start with i, instead of u
    let x: i8 = -128;
    let y: u8 = 255;
    println!("The value of x | y is: {} | {}", x, y);
  }

  println!(" ===== Literals");
  {
    // * Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000
    let x: i16 = 1_024;
    let y: u16 = 0xffff;
    let z: u32 = 0o77;
    let g: i32 = 0b1111_0000;
    let h: u8 = b'A'; // only u8
    println!("The value of x | y | z | g | h is: {} | {} | {} | {} | {} ", x, y, z, g, h);
  }

  println!(" ===== Floating point type");
  {
    let x = 2.159; // f64
    let y: f32 = 3.578; // f32
    println!("The value of x | y is: {} | {}", x, y);

    println!(" ===== Numeric operations");
    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {} ", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {} ", difference);

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {} ", product);

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    println!("The value of quotient | floored  is: {} | {} ", quotient, floored);

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {} ", remainder);
  }

  println!(" ===== The Boolean type");
  {
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("The value of t | t  is: {} | {} ", t, f);
  }

  println!(" ===== The Character type");
  {
    // char type is four bytes in size and represents a Unicode Scalar Value
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c | z | heart_eyed_cat is: {} | {} | {} ", c, z, heart_eyed_cat);
  }

  println!(" ===== The Compound Types");
  println!(" ===== The Tuple Type");
  {
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // to get individual values need to destructure a tuple
    let (x, y, z) = tup;
    println!("The value of x | y | z is: {} | {} | {} ", x, y, z);
    // or access a tuple variable using a period
    println!("The value of second element of tuple is: {} ", tup.1);
  }

  println!(" ===== The array type");
  {
    // Unlike a tuple, every element of an array must have the same type
    let a = [1, 2, 3, 4, 5];
    println!("The value of second element of array is: {} ", a[1]);
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
    println!("The value of second element of months is: {} ", months[1]);

    // i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of second element of array is: {} ", a[1]);

    // is the same as writing let a = [3, 3, 3, 3, 3];
    let a = [3; 5];
    println!("The value of second element of array is: {} ", a[1]);
  }
  
}
