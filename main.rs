    email: String,
    token: i32,
    status: String,
    role: String,
    
}

fn user_auth(currentUser: User) {

    println!("[*] enter password")
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read line");
    
    //decode currentUser password from base64 then from rsa
    //
    //encode password input to rsa

    if password == currentUser.password {
        println!("[*] logged in")
        menu(currentUser)

    } else {
        println!("[*] incorrect password")
        user_auth(currentUser)
    }

}


//get user from colleciton

fn get_user(currentUser: User)  {

    let client = Client::with_uri_str("mongodb+srv://Admin:1234@cluster0.h7ieh.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").expect("Failed to connect to server");
    let db = client.database("Portal");
    let UserCollection = db.collection("Users");
    let mut cursor = UserCollection.find(Some(doc! {"uid" => currentUser.uid}), None).unwrap();


    while let Some(doc) = cursor.next() {
        let doc = doc.unwrap();
        currentUser.uid = doc.get_i32("UID").unwrap();
        currentUser.username = doc.get_str("Username").unwrap().to_string();
        currentUser.email = doc.get_str("Email").unwrap().to_string();
        currentUser.password = doc.get_vec("Password").unwrap();
        currentUser.token = doc.get_i32("Token").unwrap();
        currentUser.status = doc.get_str("Status").unwrap().to_string();
        currentUser.role = doc.get_str("Role").unwrap().to_string();

    userAuth(currentUser)

    }
}


fn main() {
    //starting sequence
    

    println!("Vault - by Portal");
    
    let settingsJson = fs::read_to_string("settings.json").expect("Error reading config.json");
    let settingsData: Value = serde_json::from_str(&settingsJson).expect("Error parsing config.json");
    let mut settings = settingsData.clone();

    let status = settings["Status"].as_str().unwrap();
    let uid  = settings["UID"].as_str().unwrap();
    let clientToken = settings["Token"].as_i32().unwrap(); 


    println!("[*] enter username");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");


    let currentUser = User {
        uid: 0,
        username: username.trim().to_string(),
        email: "".to_string(),
        token: 0,
        role: String,
        Status: String,
    };

    get_user(currentUser);

    }
    



