use rand::Rng;
use std::io::{self, prelude::*, BufReader};
use std::fs::{File, OpenOptions};

fn main() {

    let mut file = new_file();

    //preload_names_to_file(&mut file); 
    //uncomment to preload file

    let mut people = preload_names(&mut file);

    userload_names(&mut file, &mut people);


    let random_index = rand::thread_rng().gen_range(0..=people.len()-1);
    
    println!("the next host will be: {}", &people[random_index]);

}

fn duplicate_checker <'a>(vector: &mut Vec<String>, name: &'a mut String) -> Result<(), ()> {
    if name.as_str() == "" {
        return Err(())
    }
    for x in vector.iter() {
        if x.to_lowercase() == name.to_lowercase() {
            return Err(()) 
        }
    }
    vector.push(name.to_string());
    Ok(())
}

fn preload_names(file: &mut File) -> Vec<String> {
    let mut string_buffer = String::new();

    file.read_to_string(&mut string_buffer);

    let vector = string_to_vec(&mut string_buffer);
    vector
}

fn string_to_vec(s: &String) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let bytes = s.as_bytes();
    let mut index = 0;
    for (i, &item) in bytes.iter().enumerate() {
        
        if item == b'\n' {
            let name = &s[index..i];
            index = i +1;
            if let Err(()) = duplicate_checker(&mut vec, &mut name.to_string()) {
                continue
            }
        }
    }
    vec
}


fn userload_names(file: &mut File, vector: &mut Vec<String>) {
    loop {
        println!("enter a name to add to the pool. type \"done\" when complete.");
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let mut clean_input = input.trim().to_string();
        if clean_input.to_lowercase().as_str() == "done" {break}

        
        if let Err(()) = duplicate_checker(vector, &mut clean_input) {
            continue
        }

        file.write(input.as_bytes());

        dbg!(&vector);
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
