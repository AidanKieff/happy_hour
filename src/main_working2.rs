use rand::Rng;
use std::io::{self, prelude::*, BufReader};
use std::fs::{File, OpenOptions};

fn main() {
    //first_try();

    let file = OpenOptions::new()
                            .append(true)
                            .read(true)
                            .create(true)
                            .open("names.txt");

    let mut file = file.unwrap();

    file.write(b"first name\n");

    file.write(b"second name\n");

    file.write(b"third name\n");

    file.rewind().unwrap(); //becaue file is appending and may want to be read, this will move file cursor back to start. 

    let mut string_buffer = String::new();

    file.read_to_string(&mut string_buffer);

    let mut names_vector = string_to_vec(&string_buffer);

    dbg!("{}", names_vector);
    

    


}


fn string_to_vec(s: &String) -> Vec<&str> {
    let mut vec = Vec::new();
    let bytes = s.as_bytes();
    let mut index = 0;
    for (i, &item) in bytes.iter().enumerate() {
        
        if item == b'\n' {
            vec.push(&s[index..i]);
            index = i+1;
        }
    }

    vec
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


fn first_try() {

    //another way to randomly select:
/* 
    //use rand::seq::SliceRandom;
    //in main, add:
   
    let another_way = people.choose(& mut rand::thread_rng()).unwrap();
    dbg!(another_way);
*/
    let mut people: Vec<String> = Vec::new();


    'outer_loop: loop {
        println!("enter a name to add to the pool. type \"done\" when complete.");

       'inner_loop: loop {
            let mut input = String::new();

            io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

            let clean_input = input.trim().to_string();

            match clean_input.to_lowercase().as_str() {
                "done" => break 'outer_loop,
                "" => break 'inner_loop,
                _ => people.push(clean_input),
            }

            dbg!(&people);
        }
        
    }

    let random_index = rand::thread_rng().gen_range(0..=people.len()-1);
    
    println!("the next host will be: {}", &people[random_index]);

}