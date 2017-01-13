mod fizzbuzz;
mod linq_style_stuff;
mod state_checker;
mod struct_stuff;
mod trait_demo;
mod vector_stuff;

fn main() {
  state_checker::state_checker((25, true));
  
  fizzbuzz::fizzbuzz(1, 10);
  
  linq_style_stuff::linq_style_stuff();
  
  vector_stuff::vec_stuff();
  
  struct_stuff::struct_stuff();

  trait_demo::trait_demo();
}
