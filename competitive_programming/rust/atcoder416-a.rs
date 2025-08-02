use std::io;

fn main(){
  
  let (mut input_nums, mut input_string) : (String, String) = (String::new(), String::new());
  
  let _ = io::stdin().read_line(&mut input_nums);
  let _ = io::stdin().read_line(&mut input_string);
    
  match input_nums.trim().parse() {
    Ok(value) => input_nums = value,
    Err(error) => panic!("invalid input: {}", error),
  }
  match input_string.trim().parse() {
    Ok(value) => input_string = value,
    Err(error) => panic!("invalid input: {}", error),
  }
  
  let (mut l_string, r_string) : (&str, &str);
  
  (_,        l_string) = input_nums.split_once(char::is_whitespace).unwrap();
  (l_string, r_string) = l_string.split_once(char::is_whitespace).unwrap();
  
  let (l, r) : (i8, i8) = (l_string.parse().expect("error getting l"), 
                           r_string.parse().expect("error getting r"));
                                  
  let mut validation : bool = true;
  
  for index in l..=r {
    if input_string.chars().nth( (index-1) as usize ).unwrap() != 'o' {
      validation = false;
      break;
    }
  }
  
  match validation {
    true => println!("Yes"),
    false => println!("No"),
  }
  
}
