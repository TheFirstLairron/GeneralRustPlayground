mod fizzbuzz;
mod linq_style_stuff;
mod state_checker;
mod struct_stuff;
mod trait_demo;
mod vector_stuff;
mod complex_type_borrowing;

fn main() {
  // state_checker::state_checker((25, true));
  
  // fizzbuzz::fizzbuzz(1, 10);
  
  // linq_style_stuff::linq_style_stuff();
  
  // vector_stuff::vec_stuff();
  
  // struct_stuff::struct_stuff();

  // trait_demo::trait_demo();

  let mut manager = complex_type_borrowing::Manager::new();

  manager.add_factory(complex_type_borrowing::Factory::new());
  manager.add_factory(complex_type_borrowing::Factory::new());
  manager.add_plant(complex_type_borrowing::Powerplant::new());

  println!("{:?}", manager.readonly_factory_list());
  println!("{:?}", manager.readonly_plant_list());
}
