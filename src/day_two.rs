
use std::fs::File;
use std::io::{BufRead,BufReader};
// use colored::Colorize;

#[allow(unused_assignments)]
pub fn _run(){
    
    let file_name = "./inputs/day_two.txt";
    let file = File::open(&file_name).unwrap();
    let reader = BufReader::new(file);
    
    // let mut content:Vec<Vec<i32>> = Vec::new();
    let mut num_safe = 0;
    let splitter = " ";
    // let temp = 0;
    for line in reader.lines(){
        let mut curr = 0;
        let mut safe = true;
        let mut was_increasing: bool = true;
        let mut assigned = false;
        let mut x = ' ';
        // let mut lines_nums: Vec<i32> =; 
        // println!("{:10}\n",line.as_ref().unwrap());
        for (index, element) in line.as_ref().unwrap().split(splitter).enumerate(){
            let num:i32 = element.parse().unwrap();
            // print!("{element }");
            if index == 0{
                curr = num;
            }else {
                if !assigned {
                    if num > curr{
                        was_increasing = true;
                        assigned = true;
                        x =  '↑';
                    }else if num < curr{
                        was_increasing = false;
                        assigned = true;
                        x =  '↓';
                    }else {
                        safe = false;
                        break;
                        //2 values are the same
                    }
                    print!("[{}]:  ", x);
                }
                if num > curr && !was_increasing{
                    // was_increasing = true;
                    // assigned = true;
                    safe = false;
                }else if num < curr && was_increasing{
                    // was_increasing = false;
                    // assigned = true;
                    safe = false;
                    break;
                }else {
                    let delta = (num - curr).abs();
                    if  delta> 3{
                        
                        safe = false;
                        break
                    }
                    else if delta < 1{
                        safe = false;
                        break
                    }
                    else {
                        print!("{curr:2} |{:2}| ",delta);
                    }
                    //2 values are the same

                }
                
            }
            curr = num;
        }
        if safe{
            print!("{curr:2}");
            // content
            // .push(line
            //     .unwrap()
            //     .split(splitter)
            //     .map(|x| {
            //         print!("\t=> [{}]",x);
            //         let x:i32 = x.parse().unwrap();
            //         x}).collect());

            num_safe += 1;
            println!();
        }
        
    }
    println!("\n{num_safe}");
    
}

