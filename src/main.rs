use std::io;
fn main() {
    println!("Welcome to pig latin converter!");
    println!("input a sentence to receive the pig latin output of it!");
    let mut user_input = String::new();
 
    io::stdin()
                   .read_line(&mut user_input)
                   .expect("could not process your input");
    let mut vector: Vec<String> = Vec::new();
    for word in user_input.split_whitespace(){
        let first_letter = word.chars().next().unwrap();
        match first_letter {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                let final_string = String::from("");
                let output = final_string + word + "-hay";
                vector.push(output);
            },
            _ => {
                let final_string = String::from("");
                let edited_word = remove_first_letter(word);
                let edited_first_letter = char_to_str(first_letter); 
                let output = final_string + edited_word + "-" + &edited_first_letter + "ay";
                vector.push(output);
            },
        }
     
    }
    println!("{:?}", vector.join(" "));
}

fn remove_first_letter(value: &str) -> &str{
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}
fn char_to_str (value: char) -> String{
    value.to_string()
}