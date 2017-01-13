// An exercise presented by this tutorial: http://aml3.github.io/RustTutorial/html/01.html
pub fn state_checker(val: (i8, bool))
{
  println!("*****State Checker*****");
  match val
  {
    (num, true) if num > 20 && num < 26 => 
    {
      println!("True and in range.")
    },
    
    (num, true) if num < 20 || num > 26 =>
    {
      println!("True and out of range.")
    },
    
    (num, _) if num > 40 && num < 49 =>
    {
      println!("Special Range Case")
    },
    
    (_, _) => 
    {
      println!("Invalid State!")
    }
  }

  println!("");
}