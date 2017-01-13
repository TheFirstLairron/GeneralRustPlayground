// Trait stuff
trait Building
{
  fn toggle(&mut self);
}

#[derive(Debug)]
struct Factory
{
  is_on: bool,
  is_powered: bool
}

impl Factory
{
  fn new() -> Factory
  {
    Factory
    {
      is_on: false,
      is_powered: false
    }
  }
}

impl Building for Factory
{
  fn toggle(&mut self)
  {
    self.is_on = !self.is_on;
  }
}

pub fn trait_demo()
{
  let mut fact = Factory::new();

  println!("*****Trait Demo*****");

  println!("Pre Toggle: \n{0:?}", fact);
  fact.toggle();
  println!("Post Toggle: \n{0:?}", fact);

  println!("");

}