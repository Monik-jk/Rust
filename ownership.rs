fn main(){
  let mut my_string:String = String::from("my name is shelcy");
  let new_owner = takes_ownership(my_string);
  let new_owner2 = takes_ownership(new_owner);
  my_string = takes_ownership(new_owner2);
  println!("{}",my_string);
}

fn takes_ownership(new_string:String) -> String{
  println!("{}",new_string);
  return new_string;
}
