mod add 
mod edit 
mod view 
mod exit 
mod search

pub fn menu(currentUser: User) {

    println!("menu")
    println!("[*] 1. add")
    println!("[*] 2. view")
    println!("[*] 3. edit") 
    println!("[*] 4. search") 
    println!("[*] 5. exit")

    println!("[*] :")
    let mut menuOption =  String::new();
    io::stdin().read_line(&mut menuOption).expect("Failed to read line");
    
    if menuOption == 1 {
        
        add(currentUser)

    }

    if menuOption == 2 {


    }





