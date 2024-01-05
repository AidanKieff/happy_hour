use rand::Rng;
use std::io::{self, prelude::*, BufReader};
use std::fs::{File, OpenOptions};

fn main() {

    let mut file = new_file();

    //preload_names_to_file(&mut file); 
    //uncomment to preload file

    let mut people = preload_names2(&mut file);

    userload_names2(&mut file, &mut people);


    let random_index = rand::thread_rng().gen_range(0..=people.len()-1);
    
    println!("the next host will be: {}", &people[random_index]);

    


}


fn duplicate_checker(vector: &mut Vec<String>, name: &mut String) {
    for x in vector.iter() {
        if x.to_lowercase() == name.to_lowercase() {
            *name = "".to_string()
        }
    }
    match name.as_str() {
        "" => return,
        _ => vector.push(name.to_string()),
    }
}

fn userload_names2(file: &mut File, vector: &mut Vec<String>) {
    'new: loop {
        println!("enter a name to add to the pool. type \"done\" when complete.");
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let mut clean_input = input.trim().to_string();
        if clean_input.to_lowercase().as_str() == "done" {break 'new}

        duplicate_checker(vector, &mut clean_input);
        if clean_input.to_lowercase().as_str() == "" {continue 'new}

        file.write(input.as_bytes());

        dbg!(&vector);
    }
}


fn userload_names(file: &mut File, vector: &mut Vec<String>) {
    'new: loop {
        println!("enter a name to add to the pool. type \"done\" when complete.");
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let clean_input = input.trim().to_string();

        // if vector.contains(&clean_input) {
        //     continue 'new
        // } // same as below
        for x in vector.iter() {
            if x.to_lowercase() == clean_input.to_lowercase() {
                continue 'new
            }
        }

        match clean_input.to_lowercase().as_str() {
            "done" => break 'new,
            "" => continue 'new,
            _ => vector.push(clean_input.clone()),
        }

        file.write(input.as_bytes());

        dbg!(&vector);
    }
}

fn preload_names(file: &mut File) -> Vec<String> {
    let mut string_buffer = String::new();

    file.read_to_string(&mut string_buffer);

    let vector = string_to_vec(&mut string_buffer);
    vector
}

fn preload_names2(file: &mut File) -> Vec<String> {
    let mut string_buffer = String::new();

    file.read_to_string(&mut string_buffer);

    let vector = string_to_vec2(&mut string_buffer);
    vector
}


fn string_to_vec(s: &String) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let bytes = s.as_bytes();
    let mut index = 0;
    'new: for (i, &item) in bytes.iter().enumerate() {
        
        if item == b'\n' {
            let name = &s[index..i];
            index = i +1;
            for x in vec.iter() {
                if x.to_lowercase() == name.to_string().to_lowercase() {
                    continue 'new
                }
            }

            // if vec.contains(&name.to_string())|| vec.contains(&name.to_string().to_lowercase()) {
            //     index = i + 1;
            //     continue
            // }

            vec.push(name.to_string());
        }
    }
    vec
}

fn string_to_vec2(s: &String) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let bytes = s.as_bytes();
    let mut index = 0;
    for (i, &item) in bytes.iter().enumerate() {
        
        if item == b'\n' {
            let name = &s[index..i];
            index = i +1;
            duplicate_checker(&mut vec, &mut name.to_string());
        }
    }
    vec
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



fn second_try() {
    let file = OpenOptions::new()
    .append(true)
    .read(true)
    .create(true)
    .open("names.txt");


let mut file = file.unwrap();

//preload_names(&mut file); 
//uncomment to preload file





let mut string_buffer = String::new();
file.read_to_string(&mut string_buffer);


let mut people = Vec::new();                                                                                                      
let bytes = string_buffer.as_bytes();                                                                                                      
let mut index = 0;                                                              //function                                        
for (i, &item) in bytes.iter().enumerate() {                                                                                                      
if item == b'\n' {                                                                                                      
let name = &string_buffer[index..i];                                                                                                      
people.push(name.to_string());                                                                                                      
index = i+1;                                                                                                      
}                                                                                                      
}
//dbg!("{}", people);

'new: loop {
println!("enter a name to add to the pool. type \"done\" when complete.");
let mut input = String::new();

io::stdin()
.read_line(&mut input)
.expect("Failed to read line");
                                                                //function
let mut clean_input = input.trim().to_string();

for x in &people {
if x == &mut clean_input {                                                          //function
continue 'new
}
}

match clean_input.to_lowercase().as_str() {
"done" => break 'new,                                                                       //function?
"" => continue 'new,
_ => people.push(clean_input.clone()),
}

file.write(input.as_bytes());

dbg!(&people);
}

let random_index = rand::thread_rng().gen_range(0..=people.len()-1);

println!("the next host will be: {}", &people[random_index]);




}