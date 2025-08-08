// https://www.codechef.com/problems/PRFSM

fn get_input_as_number() -> i32 {
  
  let mut string : String = String::new();
  
  std::io::stdin().read_line(&mut string).expect("No input detected");
  
  string.trim().parse().expect("Input received isnt a single number")
  
}

fn get_input_as_list() -> Vec<i32> {
  
  let mut string : String = String::new();
  
  std::io::stdin().read_line(&mut string).expect("No input detected");
  
  string.trim().split(" ").map(|s| s.parse().expect("Input is not a valid integer")).collect()
  
}

use get_input_as_number as getInpNum;
use get_input_as_list as getInpList;

fn main() {
  
  let t : i32 = getInpNum();
  
  for _ in 1..=t {
  
    let _ : i32 = getInpNum();
    
    let list_nums : Vec<i32> = getInpList();
    let mut list_prefix : Vec<i64> = vec![0];
    
    for i in 1..=list_nums.len() {
      list_prefix.push( list_prefix[i-1] + (list_nums[i-1]) as i64 );
    }
    
    let q : i32 = getInpNum();
    
    for _ in 1..=q {
      
      let lr_vec : Vec<i32> = getInpList();
      let (l, r) : (usize, usize) = (lr_vec[0] as usize, lr_vec[1] as usize);
      
      print!("{} ", ( list_prefix[r] - list_prefix[l-1] ));
      
    }
  
  }
    
}
