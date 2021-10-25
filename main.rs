//system imports

use std::io;
use std::process;
use std::thread;
use std::fs;
use std::env;
use futures::stream::StreamExt;


//json
use serde::Deserialize;
use serde_json::{Result, Value};

//animations
use terminal_spinners::{SpinnerBuilder, DOTS};

//socket/networking


//mongodb
use mongodb::{
    bson::{doc, Bson },
    sync::Client,
};




//authenticator
use google_authenticator::GoogleAuthenticator;

//encryption
use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;


struct User {

    uid: i32,
    username: String,
    email: String,
    
}

struct Collection {
    User: String,
    Password: String,
    Logs: String,
    Alerts: String,
}

fn userAuth() {


}

//get user from colleciton

fn get_user(currentUser: User)  {

    let client = Client::with_uri_str("mongodb+srv://Admin:1234@cluster0.h7ieh.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").expect("Failed to initialize client");
    let db = client.database("Portal");
    let UserCollection = db.collection("Users");
    let mut cursor = UserCollection.find(Some(doc! {"uid" => currentUser.uid}), None).unwrap();


    while let Some(doc) = cursor.next() {
        let doc = doc.unwrap();
        currentUser.uid = doc.get_i32("UID").unwrap();
        currentUser.username = doc.get_str("Username").unwrap().to_string();
        currentUser.email = doc.get_str("Email").unwrap().to_string();
        currentUser.password = doc.get_vec("Password", "binary").unwrap();



    userAuth(currentUser: User)
    }
}


fn main() {
    //starting sequence
    

    println!("Vault - by Portal");
    
    let settingsJson = fs::read_to_string("settings.json").expect("Error reading config.json");
    let settingsData: Value = serde_json::from_str(&settingsJson).expect("Error parsing config.json");
    let mut settings = settingsData.clone();

    let db_url = settings["db_url"].as_str().unwrap();
    

    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");


    let currentUser = User {
        uid: 0,
        username: username.trim().to_string(),
        email: "".to_string(),
    };

    get_user(currentUser);

    }
    



