
/* 
 * Passing Ownership back & forth to same variable & thus making variable also mutable as we are 
 * changing the value binded to it. 
fn main() {
    let mut player_name = String::from("Virat");
    player_name = find_lastname(player_name); // getting ownersip back via return (function moves
                                              // ownership back) 
     println!("Player Name : {}",player_name);
}
*/  

/*
 * Passing data to mutable reference as want to modify the data 
 * in function & returning ownership back of player_name to full_name 
*/
fn main() {
    let player_name = String::from("Virat");
    let full_name = find_lastname(player_name);  


    println!("Player Name : {}",full_name);
}

fn find_lastname(mut name: String) -> String {
    name.push_str(" Kohli"); 
    name 
}

