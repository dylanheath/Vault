//system imports

use std::io;
use std::process;
use std::thread;
use std::fs;
use std::env;
use bson::oid::Error;
use futures::stream::StreamExt;
use async_std::io::prelude::*;
use async_std::net;
//function imports

//json
use serde::Deserialize;
use serde_json::{json ,Result, Value};

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
    password: String,  
    email: String,
    token: i32,
    status: String,
    role: String,
    
}

fn menu(currentUser: User) {
    println!("menu");


}

fn user_auth(currentUser: User) {

    println!("[*] enter password");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read line");
    
    //decode currentUser password from base64 then from rsa
    //
    //encode password input to rsa

    if password == currentUser.password {
        println!("[*] logged in");

    } else {
        println!("[*] incorrect password");
    }

}


//get user from colleciton

async fn get_user(currentUser: User) -> <i32> {

    let client = Client::with_uri_str("mongodb+srv://Admin:1234@cluster0.h7ieh.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").expect("Failed to connect to server");
    let db = client.database("Portal");
    let UserCollection = db.collection("Users");
    
    let user_data = UserCollection.find_one(Some(doc! {"UID": currentUser.uid}), None).unwrap();
    
   // let query = doc! {"Username": currentUser.username};

    currentUser.uid =  user_data.get_i32("UID");

}









    let user_data: Value = json!(user_data);

    currentUser.uid = user_data["UID"].to_i32();

    
    
   // while let Some(doc) = cursor.next() {
     //   let doc = doc.unwrap();
       // currentUser.uid = doc.get_i64("UID").unwrap();
       // currentUser.username = doc.get_str("Username").unwrap().to_string();
       // currentUser.email = doc.get_str("Email").unwrap().to_string();
       // currentUser.password = doc.get_vec("Password").unwrap();
       // currentUser.token = doc.get_i32("Token").unwrap();
       // currentUser.status = doc.get_str("Status").unwrap().to_string();
       // currentUser.role = doc.get_str("Role").unwrap().to_string();

    user_auth(currentUser)

    }


fn main() {
    //starting sequence
    

    println!("Vault - by Portal");
    
    let settingsJson = fs::read_to_string("settings.json").expect("Error reading config.json");
    let settingsData: Value = serde_json::from_str(&settingsJson).expect("Error parsing config.json");
    let mut settings = settingsData.clone();

    let status = settings["Status"].as_str().unwrap();
    let uid  = settings["UID"].as_str().unwrap();
    let client_Token = settings["Token"].as_i32().unwrap(); 


    println!("[*] enter username");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");


    let currentUser = User {
        uid: 0,
        username: username.trim().to_string(),
        email: "".to_string(),
        password: "".to_string(),
        token: 0,
        role: "".to_string(),
        status: "".to_string(),
    };

    get_user(currentUser);

    }
    



