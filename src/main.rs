use rand::seq::SliceRandom;
use rand::thread_rng;
use std::process::Command;


use std::fs;

const PEOPLE: [&str; 6] = [...redacted or smth...]; 



fn main() { 
    println!("Secret Santa Selection!");
    
    // clean out/
    let _ = fs::remove_dir_all("out/");
    fs::create_dir("out/").expect("unable to create directory");

    // the index is the gifter and the value is the giftee
    let arr = gen_arr();
        
    for (i, element) in arr.iter().enumerate() {
        let file_path = format!("out/{}.txt", PEOPLE[i]);
        let file_content = format!("To: {}\nFrom: ~\n\nYour target: {}", PEOPLE[i], PEOPLE[*element]);
        fs::write(&file_path, file_content).expect("Unable to write to file");
        Command::new("sh")
            .arg("-c")
            .arg(format!("zip {}.zip {}", &file_path, &file_path))
            .output()
            .expect("failed to execute zip process");
        let _ = fs::remove_file(&file_path);
    }
}

// shuffle til valid then return
fn gen_arr() -> [usize; 6] {
    let mut arr: [usize; 6] = [0, 1, 2, 3, 4, 5]; 
    while !is_valid(&arr) { arr.shuffle(&mut thread_rng()); }
    arr
}

fn is_valid(arr: &[usize; 6]) -> bool {
    for (i, element) in arr.iter().enumerate() {
        if i == *element { return false; } // people can't gift to themselves
    }
    return true
}
