//system imports

use std::io;
use std::process;
use std::thread;
use std::fs;
use std::env;
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
//authenticator
use google_authenticator::GoogleAuthenticator;

//encryption
use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;


#[derive(Debug , Serialize, Deserialize)]
struct User {
    name: String,
    password: String,
}

fn user_data() {

}


async fn find_user(coll: mongodb::Collection::<User>) -> mongodb::error::Result<()> {
    
    println!("[*] enter username");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to get input");
    
    let handle = SpinnerBuilder::new().spinner(&DOTS).text("  Loading Data").start();

    let cursor = coll.find(doc! {"name": username}, None).await?;
    println!("{:?}", cursor);

   //while let Some(user) = cursor.try_next().await? {
       //println!("{:?}", user);
   // }


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
