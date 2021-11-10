fn main() {

  println!(" === Control Flow");
  {
    let number = 3;
    // Rust expected a bool
    // Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
  }

  println!(" === Multiple conditions");
  {
    // Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
  }

  println!(" === if in a let Statement");
  {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
  }

  println!(" === Repetition with Loops");
  //loop, while, and for
  {
    // infinite loop
    // break - stop executing the loop
    // continue - skip over any remaining code in this iteration
    // label - 'counting_up
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
  }

  println!(" === Returning a value from loop");
  {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
  }

  println!(" === Conditional loops");
  {
    let mut number = 3;

    while number != 0 {
      println!("{}!", number);

      number -= 1;
    }
    println!("Liftoff!!!");
  }

  println!(" === Looping through a collection with while");
  {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
      println!("the value is: {}", a[index]);

      index += 1;
    }
  }

  println!(" === Looping through a collection with for");
  {
    let a = [10, 20, 30, 40, 50];

    for element in a {
      println!("the value is: {}", element);
    }
  }

  println!(" === Looping through a collection with for with reverse range");
  {
    for number in (1..4).rev() {
      println!("the value is: {}", number);
    }
    println!("Liftoff!");
  }

  let temp: f32 = temp_conversion(55.0, false);
  println!("55 °F = {} °C ", temp);

  let temp: f32 = temp_conversion(30.0, true);
  println!("30 °C = {} °F ", temp);

  for i in 1..10 {
    let fib = gen_fib_number(i);
    println!("{}th number of Fibonacci is {}", i, fib);
  }

  sing();

}

// Exercises
// Convert temperatures between Fahrenheit and Celsius.
fn temp_conversion(input: f32, celcius: bool  ) -> f32  {

  // input type is Celcius
  // X°F = (input°C × 9/5) + 32
  // X°C = (input°F − 32) × 5/9
  if celcius {
    input * (9.0 / 5.0) + 32.0 
  } else {
    (input - 32.0) * (5.0 / 9.0) 
  }
}

// Generate the nth Fibonacci number
fn gen_fib_number(n: i32) -> i32 {
  if n <= 1 {
    return n;
  }
  return gen_fib_number(n - 1) + gen_fib_number(n - 2);
}

// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
fn sing() {

  println!();
  // 4 sections
  for s in 1..5 {
    for l in 1..5 {
      if ((s == 1) || (s == 2)) && (l == 1) {
        println!("Have yourself a merry little Christmas,");
      } else if (s == 1) && (l == 2) {
        println!("Let your heart be light");
      } else if ((s == 1)|(s == 2)) && (l == 3) {
        println!("From now on,");
      } else if (s == 2) && (l == 2) {
        println!("Make the Yule-tide gay,");
      } else if ((s == 1)|(s == 2)) && (l == 4) {
        print!("Our troubles will be ");
        if s == 1 { print!("out of sight \n"); }
        else { print!("miles away.\n"); }
      } else if (s == 3) && (l == 1) {
        println!("Here we are as in olden days,\n\
          Happy golden days of yore.\n\
          Faithful friends who are dear to us\n\
          Gather near to us once more.");
      } else if (s == 4) && (l == 1) {
        println!("Through the years\n\
        We all will be together,\n\
        If the Fates allow\n\
        Hang a shining star upon the highest bough.\n\
        And have yourself A merry little Christmas now.");
      }
    }
    println!();
  }
}
