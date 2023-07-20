mod card_list;


use card_list::*;


use rand::Rng;

use std::fs;

use std::io::BufRead;

use std::{collections::HashMap, hash::Hash};



// Later refactor to clean up and make clear (functions)

fn main() {
    println!("Hello, world!");

    //let mut rand_list = [87, 18, 6, 42, 27, 14, 64, 76, 5, 109, 12, 90, 7, 74, 17, 57, 45, 48, 39, 83, 26, 2, 100, 28, 34, 50, 10, 86, 20, 11, 36, 21, 41, 105, 53, 9, 81, 77, 59, 70, 22 ,78, 44, 111, 55, 95, 79, 38, 91, 93, 69, 32, 107, 66, 106, 15, 37, 33, 4, 82];
    //rand_list.sort_unstable();
    //println!("rand_list sorted: {:?}", rand_list);

    let mut num_list: Vec<i32> = vec![];

    for _ in 1..60 {
        let num = rand::thread_rng().gen_range(1..111);
        num_list.push(num);
    }
    num_list.sort();

    println!("{:?}", num_list);

/* 
    for number in num_list {
        match number {
            1..=19 => println!("White card"),
            20..=38 => println!("Blue card"),
            39..=57 => println!("Black card"),
            58..=76 => println!("Red card"),
            77..=95 => println!("Green card"),
            96..=105 => println!("Artifact card"),
            106..=111 => println!("Land card"),
            _ => println!("Not valid"),
        }
    }

*/
    let mut card_map: HashMap<i32, String> = HashMap::new();

    // open card_list.txt
    // for each (i, card_name) in card_list.iter.enumerate
    //   card_map.insert(i, String::from(card_name));
    let file_path = "/Users/bradfordarmstrong/Projects/rust_space/mtg_work/rev_parser/src/card_list/card_list.txt";
    println!("file_path: {}", file_path); 

    //let card_list_contents = fs::read_to_string(file_path).expect("Should be able to read the file");

    //println!("card_list_contents: {}", card_list_contents);

    // Fix the for loop to extract card_name as a String, not a Resiult<String, Error> to prepare against future errors?

    let file = fs::File::open(file_path).expect("Should be able to find the file");
    let file_reader = std::io::BufReader::new(file);

    for (i, card_name) in file_reader.lines().enumerate() {
        //println!("number, card_name: {}: {}", i+1, card_name.unwrap());
        card_map.insert((i+1) as i32, card_name.unwrap());
    }

    //println!("card_map: {:?}", card_map);

    let randocard = card_map.get(&3).unwrap();
    println!("randocard: {:?}", randocard);

    for key in num_list.iter() {
        let mut color = "";
        match key {
            1..=19 => color = "White", //println!("White card"),
            20..=38 => color = "Blue", //println!("Blue card"),
            39..=57 => color = "Black", //println!("Black card"),
            58..=76 => color = "Red", //println!("Red card"),
            77..=95 => color = "Green", //println!("Green card"),
            96..=105 => color = "Artifact", //println!("Artifact card"),
            106..=111 => color = "Land", //println!("Land card"),
            _ => println!("Not valid"),
        }
        let keyed_card = card_map.get(&key).unwrap();
        println!("keyed_card is {}: {}", color, keyed_card);
    }

}
