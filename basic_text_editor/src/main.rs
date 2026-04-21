use std::io::{self, Write};
use std::fs::File;
fn main() {
    println!("***Welcome to Text Editor***");
    
    let mut store = vec![];

    let mut file_name = String::new();
    print!("Enter file name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Input failed.");
    let cleaned_file_name = file_name.trim();
    println!("Start typing below.");

    let mut line_counter = 1;

    loop{
        let mut userinput = String::new();

        print!("{}| ", line_counter);
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut userinput)
            .expect("Input failed.");

        let cleaned = userinput.trim_end();

        if cleaned == "exit" {
            break;
        }

        if cleaned.is_empty() {
            continue;
        }
        
        store.push(cleaned.to_string());  
        
        line_counter += 1;

    }

    let mut save_file = File::create(cleaned_file_name).unwrap();
    
    for everyitem in &store{
        writeln!(save_file, "{}", everyitem).unwrap();
    }

    println!("--- Saved to: {} ---", cleaned_file_name);
    for item in &store {
        println!("{}", item);
    }
    println!("--- Lines saved: {} ---", store.len());

}
