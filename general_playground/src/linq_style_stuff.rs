pub fn linq_style_stuff()
{
  println!("*****Linq Style Stuff*****");
  
  let mut vec = (1..11).collect::<Vec<_>>();
  
  println!("Before Filter: \n{0:?}", vec);
  
  // filter numbers
  vec = vec.into_iter().filter(|x| *x % 2 == 0).collect::<Vec<_>>();
  
  println!("After Filter: \n{0:?}", vec);
  
  println!("")
}