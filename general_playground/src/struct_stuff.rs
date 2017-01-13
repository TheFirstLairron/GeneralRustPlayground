#[derive(Debug)]
struct Point2D
{
  x: i32,
  y: i32
}

// Builder Pattern
impl Point2D
{
  fn new() -> Point2D
  {
    Point2D
    {
      x: 0,
      y: 0
    }
  }
  
  fn x(&mut self, num: i32) -> &mut Point2D
  {
    self.x = num;
    self
  }
  
  fn y(&mut self, num: i32) -> &mut Point2D
  {
    self.y = num;
    self
  }
  
  fn build(&mut self) -> Point2D
  {
    Point2D
    {
      x: self.x,
      y: self.y
    }
  }
}

pub fn struct_stuff()
{
  println!("*****STRUCT STUFF*****");
  
  let x = 1;
  let y = 1;
  
  // Cause it to leave scope, so that i can test the values passed in later
  {
    let point = Point2D::new().x(1).y(1).build();
  
    println!("{0:?}", point);
  }
  
  println!("X: {}, Y: {}", x, y);
  println!("");
}