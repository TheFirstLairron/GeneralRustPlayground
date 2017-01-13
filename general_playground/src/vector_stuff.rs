// Vector Increment exercise from: http://aml3.github.io/RustTutorial/html/02.html
fn increment_vectors(list: &Vec<i32>) -> Vec<i32>
{
  let mut new_list: Vec<i32> = Vec::new();
  
  for num in list
  {
    new_list.push(num + 1);
  }
  
  new_list
}

fn increment_vector_in_place(list: &mut Vec<i32>)
{
  for mut num in list
  {
    *num += 1;
  }
}

pub fn vec_stuff()
{
  println!("*****VECTOR STUFF*****");
  
  let mut vec = vec![1, 2, 3, 4];
  
  let new_vec = increment_vectors(&vec);
  
  println!("New Vector: \n{0:?}", new_vec);
  
  println!("");
  
  increment_vector_in_place(&mut vec);

  println!("In Place Vector: \n{0:?}", vec);
  
  
  
  println!("");
}