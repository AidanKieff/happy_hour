use rand::Rng;
use std::io::{self, prelude::*, BufReader};
use std::fs::{File, OpenOptions};
use std::collections::HashMap;
//another way to randomly select:
/* 
//use rand::seq::SliceRandom;
    //in main, add:
   
    let another_way = people.choose(& mut rand::thread_rng()).unwrap();
    dbg!(another_way);
*/
fn main() {

    let mut file = new_file();

    //preload_names_to_file(&mut file); 
    //uncomment to preload file

    let mut people = preload_names(&mut file);



    userload_names(&mut file, &mut people);


    random_value(&people);
    dbg!(&people);

}



fn random_value(hashmap: &HashMap<String, String>) {
    let mut iter = hashmap.values();
    let random_index = rand::thread_rng().gen_range(0..=hashmap.len()-1);

    
    println!("the next host will be: {}", &iter.nth(random_index).unwrap());
}

fn duplicate_checker (hashmap: &mut HashMap<String, String>, name: &String) -> Result<(),()> {
    if name.as_str() == "" {
        return Err(())
    }
    if hashmap.contains_key(&name.to_lowercase()) {
        return Err(())
    }
    hashmap.insert(name.to_lowercase(), name.to_string());
    Ok(())
}

fn preload_names(file: &mut File) -> HashMap<String, String> {
    let mut string_buffer = String::new();

    file.read_to_string(&mut string_buffer);

    let hashmap = string_to_hashmap(&mut string_buffer);
    hashmap
}

fn string_to_hashmap(s: &String) -> HashMap<String, String> {
    let mut hashmap = HashMap::new();
    let bytes = s.as_bytes();
    let mut index = 0;
    for (i, &item) in bytes.iter().enumerate() {
        
        if item == b'\n' {
            let name = &s[index..i];
            index = i +1;
            if let Err(()) = duplicate_checker(&mut hashmap, &name.to_string()) {
                continue
            }
        }
    }
    hashmap
}


fn userload_names(file: &mut File, hashmap: &mut HashMap<String, String>) {
    loop {
        println!("enter a name to add to the pool. type \"done\" when complete.");
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let clean_input = input.trim().to_string();
        if clean_input.to_lowercase().as_str() == "done" {break}

        
        if let Err(()) = duplicate_checker(hashmap, &clean_input) {
            continue
        }

        file.write(input.as_bytes());

        dbg!(&hashmap);
    }
}



fn new_file() -> File {
    let file = OpenOptions::new()
                                    .append(true)
                                    .read(true)
                                    .create(true)
                                    .open("names.txt")
                                    .unwrap();
    file
}




fn preload_names_to_file(file: &mut File) {
    file.write(b"jerry\n");
    file.write(b"another name\n");
    file.write(b"barry\n");
    file.write(b"Seymore\n");
    //if file is appending
    //this will move file cursor back to start. 
    //so that if it's read after,
    //it will read from start
    file.rewind().unwrap(); 
}



fn vectorize(file: &mut File) -> Vec<String> {

    let mut vector: Vec<String> = Vec::new();

    let file = BufReader::new(file);
    
    for x in file.lines() {
        vector.push(x.unwrap());
    }

    dbg!("{}", &vector);
    vector
}

