pub fn fizzbuzz(min: i32, max: i32)
{
  println!("*****Fizz Buzz*****");

  for num in min..max
  {
    match num
    {
      num if num % 15 == 0 => println!("fizzbuzz"),
      num if num % 3 == 0 => println!("fizz"),
      num if num % 5 == 0 => println!("buzz"),
      _ => println!("{}", num)
    }
  }

  println!("");
}