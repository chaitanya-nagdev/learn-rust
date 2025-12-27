fn main() {

    let mut name = String::from("Chaitanya"); 
    append_last_name(&mut name);
        
    println!("{}",name);
    
    let f1 = &mut name; 
    just_playing(f1); 

    let _f2 = &mut name; 
    //println!("Issue fr fr {},{}",f1,f2);

}

fn append_last_name(name:&mut String) {
    name.push_str(" Nagdev"); 
}

/*
fn just_playing(f1: &mut String) {
    println!("Just Playing with {}",f1); 
}
*/
