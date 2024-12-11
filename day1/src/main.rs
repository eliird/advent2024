use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::BinaryHeap;
use std::collections::HashMap;


fn load_input_file(file_path: &str) -> io::Result<String> {
   
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);
    
    let mut contents = String::new();

    for line in reader.lines() {
        match line{
            Ok(line) => {
                contents.push_str(&format!("{} ", line));
                contents.push('\n');
            },
            Err(e) => return Err(e),
        }
    }

    Ok(contents)
}


fn part_one(contents: &str){
    let mut heap1: BinaryHeap<i32> = BinaryHeap::new();
    let mut heap2: BinaryHeap<i32> = BinaryHeap::new();
 
    for line in contents.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        heap1.push(words[0].parse().unwrap());
        heap2.push(words[1].parse().unwrap());
    }

    println!("{}, {}", heap1.len(), heap2.len());

    let mut sum = 0;

    while !heap1.is_empty() {
        let val1 = heap1.pop().unwrap();
        let val2 =  heap2.pop().unwrap();
        let distance = (val1 - val2).abs();
        sum += distance;
    }

    println!("{}", sum);
}


fn part_two(contents: &str) {
    let mut data_map: HashMap<i32, i32> = HashMap::new();
    let mut data_list: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let value: i32 = words[0].parse().unwrap();
        data_list.push(value);
        let key: i32 = words[1].parse().unwrap();
        
        match data_map.get(&key){
            Some(val) => {
                data_map.insert(key, val + 1);
             },
             None => {
                data_map.insert(key, 1);
             }
        }
    }

    let mut sum = 0;
    for num in data_list.iter(){
        match data_map.get(num){
            Some(val) => {
                sum += num * val;
             },
             None => {}
        }
    }
    println!("\n{}", sum);
}


fn main() -> io::Result<()> {
    let contents = load_input_file("src/input.txt")?;

    part_one(&contents);

    part_two(&contents);
    Ok(())
    // part_two()

}
