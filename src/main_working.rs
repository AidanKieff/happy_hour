use rand::Rng;
use std::io;

//another way to randomly select:
/* 
//use rand::seq::SliceRandom;
    //in main, add:
   
    let another_way = people.choose(& mut rand::thread_rng()).unwrap();
    dbg!(another_way);
*/
fn main() {

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
