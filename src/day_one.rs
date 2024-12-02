use std::collections::HashMap;
// use std::{collections::HashSet};
use std::fs::{self, File};
use std::io::{BufReader, BufRead};



pub fn run() {  
  let file_name = "./inputs/day_one.txt";
  
  let splitter = "   ";

  let file = File::open(&file_name).unwrap();

  let reader = BufReader::new(file);

  // let right_list          : Vec<i32>  = vec![4,3,5,3,9,3];
  // let left_list           : Vec<i32>  = vec![3,4,2,1,3,3];
  let mut right_list          : Vec<i32> = vec![];
  let mut left_list           : Vec<i32> = vec![];
  let mut similarity_score: HashMap<i32, i32>= HashMap::new();
  for line in reader.lines(){
      let line = line.unwrap();

      let numbers: Vec<i32> = line
      .split(splitter)
      .filter_map(|word|{
          word.parse().ok()
      })
      .collect();

      if numbers.len() == 2{

          left_list.push(numbers[0]);
          right_list.push(numbers[1]);
          
          match similarity_score.get(&numbers[1]){
              Some(x) => {
                  similarity_score.insert(numbers[1], x+1);
              },
              None => {
                  similarity_score.insert(numbers[1], 1);
              }
          }
      }
  }

  //left - right
  let mut distances      : Vec<i32>  = vec![];
  let mut total_distance : i32       = 0;
  // let hash_right: HashSet<&i32> = HashSet::from_iter(right_list.iter());
  // let hash_left: HashSet<&i32> = HashSet::from_iter(left_list.iter());

  assert_eq!(right_list.len(),left_list.len());

  left_list.sort();
  right_list.sort();


  for ii in 0..right_list.len()
  {
      let mut lefty = left_list[ii];
      let righty = right_list[ii];

      let mut delta: i32 = match similarity_score.get(&lefty)
      {
          Some(x) => {
              println!("{lefty:6} has apeared {x} times");
              x * lefty
          },
          None => {
              // lefty
              println!("{lefty:6} has apeared 0 times");
              0
          }
      };
      // if n > p{
      //     delta = lefty-righty;
          
      // }else {
      //     delta = righty-lefty;  
      // }
      total_distance += delta;
      distances.push(delta);
  }



  // for p in &right_list
  // {
  //     for n in &left_list{
  //         let mut delta=0;
  //         if n > p{
  //             delta = n-p;
              
  //         }else {
  //             delta = p-n;  
  //         }
  //         total_distance += delta;
  //         distances.push(delta);
  //     }
  // }
  // for (index, distance) in distances.into_iter().enumerate(){
  //     println!("[{:4}] {distance:6}: [{:6} -{:6}]",index+1,left_list[index], right_list[index]);
  // }
  println!("\n [{total_distance}] ");
}