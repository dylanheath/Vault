//system imports
use tokio;
use std::io;
use std::process;
use std::thread;
use std::fs;
use std::str;
use std::fmt;
use std::env;
use bson::from_document;
use bson::oid::Error;
use futures::stream::StreamExt;
use futures::TryStreamExt;
use async_std::io::prelude::*;
use async_std::net;
//function imports

use serde::{Deserialize , Serialize};
use serde_json::{json ,Result, Value};

//animations
use terminal_spinners::{SpinnerBuilder, DOTS};

//socket/networking


//mongodb
use mongodb::{Client, bson::doc , options::FindOptions };
use mongodb::bson::{self, Bson};
//authenticator
use google_authenticator::GoogleAuthenticator;

//encryption
use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;


#[derive(Debug , Serialize, Deserialize)]
struct User {
    uid: i32,
    name: String,
    password: String,

}

#[derive(Debug , Serialize , Deserialize)]
struct Password {
    name: String,
    username: String,
    password: String,
    email: String,
  
}

impl fmt::Display for User{
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(
         f,
         "{},{}, {}",
         self.uid,
         self.name,
         self.password,
      )
   }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "{}, {} ,{}, {}",
            self.name,
            self.username,
            self.password,
            self.email,
        )
       }
}

//static server connections
//
fn exit(current_User: User) {


}


async fn add(current_User: User) -> mongodb::error::Result<()> {
    
    println!("[*] enter password name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to get input");

    println!("[*] enter username");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to get input");

    println!("[*] enter password");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to get input");

    println!("[*] enter email");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Failed to get input");

    let handle = SpinnerBuilder::new().spinner(&DOTS).text("  Loading Data").start();

    let passwordadd = Password {
        name: name,
        username: username,
        password: password,
        email: email,
    };

    let client = Client::with_uri_str("mongodb+srv://Admin:1234@cluster0.h7ieh.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await?;
    let db = client.database("myFirstDatabase");
    let coll = db.collection::<Password>("Passwords");

    let insert = coll.insert_one(Password {name: passwordadd.name.to_string(), password: passwordadd.password.to_string() , username: passwordadd.username.to_string() , email: passwordadd.email.to_string() }, None ).await?;

    Ok(())

}


//return result
async fn delete() {

}

async fn view(current_User: User) -> mongodb::error::Result<()> {

    let client = Client::with_uri_str("mongodb+srv://Admin:1234@cluster0.h7ieh.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await?; 
    let db = client.database("myFirstDatabase");
    let coll = db.collection::<Password>("Passwords");

    let mut cursor = coll.find(doc! {"UID": current_User.uid}, None).await?;

    while let Some(password) = cursor.try_next().await? {

            let passwordview = Password {
                name: password.name,
                username: password.username,
                password: password.password,
                email: password.email,
             };
             
            println!("[*] {}", passwordview.name);
            println!("  {}", passwordview.username);
            println!("  {}", passwordview.password);
            println!("  {}", passwordview.email);
        
    } 

    Ok(())
}


fn menu(current_User: User) {
    println!("[*] menu");
    println!(" 1. add");
    println!(" 2. view");
    println!(" 3. exit");

    let mut menu_option = String::new();
    io::stdin().read_line(&mut menu_option).expect("Failed to read line");
    let menu_option = menu_option.trim();

    if menu_option == "1" {
        add(current_User);
    } else if menu_option == "2"  {
        view(current_User);
    } else if menu_option == "3" {
        exit(current_User);
    } else {
        println!("option not valid");
        menu(current_User);
    };
    


//menu options and display    
    // println!("{}" , current_User.uid);    
}

async fn find_user(coll: mongodb::Collection::<User>) -> mongodb::error::Result<()> {

    // need to change how to string is formatted compared to mongodb query 
    println!("[*] enter username");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to get input");

    let username = username.trim();
 
    let filter = doc! {"name": username};
    
    let handle = SpinnerBuilder::new().spinner(&DOTS).text("  Loading Data").start();

    let mut cursor = coll.find(filter, None).await?; 
   // println!("{:?}", cursor);
   //
    std::thread::sleep(std::time::Duration::from_secs(3));
    handle.done();

    if let Some(user) = cursor.try_next().await? {
        let current_User = User {
           uid: user.uid,
           name: user.name,
           password:user.password,
        };
        

        
         menu(current_User);
    };


    Ok(()) 
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb+srv://Admin:1234@cluster0.h7ieh.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await?;
    let db = client.database("User"); //< needs to change for every collection such as Users and passwords

    let coll = db.collection::<User>("User");

    find_user(coll).await?;
    
    Ok(())

}
