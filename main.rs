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
    bson::{doc, Bson},
    Client,
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



fn main() {
    //starting sequence
    println!("Vault - by Portal");
    
    let settingsJson = fs::read_to_string("settings.json").expect("Error reading config.json");
    let settingsData: Value = serde_json::from_str(&settingsJson).expect("Error parsing config.json");
    let mut settings = settingsData.clone();

    let db_url = settings["db_url"].as_str().unwrap();


}

//async get user 
async fn get_user(client: &Client, username: String) -> Result<User, mongodb::error::Error> {
    //get user
    let user = client.database("vault").collection("users").find_one(Some(doc!{"username": username}), None);
    //return user
    Ok(user.into_document().unwrap())
}