//system imports
use tokio;
use std::io;
use std::process;
use std::thread;
use std::fs;
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
use mongodb::{Client, bson::doc };
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
    
    


}

async fn find_user(coll: mongodb::Collection::<User>) -> mongodb::error::Result<()> {

    println!("[*] enter username");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to get input");
    
    let handle = SpinnerBuilder::new().spinner(&DOTS).text("  Loading Data").start();

    let mut cursor = coll.find(doc! {"name": username}, None).await?; 
   // println!("{:?}", cursor);

    if let Some(user) = cursor.try_next().await? {
        let current_User = User {
           uid: user.uid,
           name: user.name,
           password: user.password,
        };

         menu(current_User);
    };

    std::thread::sleep(std::time::Duration::from_secs(3));
    handle.done();
     
    Ok(()) 
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb+srv://Admin:1234@cluster0.h7ieh.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await?;
    let db = client.database("myFirstDatabase");

    let coll = db.collection::<User>("User");

    find_user(coll).await?;

    Ok(())

}
