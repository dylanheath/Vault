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

#[derive(Debug , Serialize , Deserialize)]
struct edit_pass {
    password: String,
}

#[derive(Debug , Serialize , Deserialize)]
struct edit_name {
    name: String,
}

#[derive(Debug , Serialize , Deserialize)]
struct edit_passname {
    name: String

}

#[derive(Debug , Serialize , Deserialize)]
struct edit_password {
    password: String
}

#[derive(Debug , Serialize , Deserialize)]
struct edit_username {
    username: String
}

#[derive(Debug , Serialize , Deserialize)]
struct edit_email {
    email: String
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
//


async fn password_editor(find_password: Password) -> mongodb::error::Result<()> {

    let client = Client::with_uri_str("mongodb+srv://Admin:1234@cluster0.h7ieh.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await?;
    let db = client.database("Passwords");
    let coll = db.collection::<Password>("Passwords");

    println!("[*] enter a field to edit"); 

    let mut edit_option = String::new();
    io::stdin().read_line(&mut edit_option).expect("Failed to read line");
    let edit_option = edit_option.trim();

    let mut field = String::new();
    let field = edit_option.trim(); 

    println!("[*] enter new {}", field);

    let mut field_data = String::new();
    io::stdin().read_line(&mut field_data).expect("Failed to read line");
    let field_data = field_data.trim();

    let filter = doc!{"name": find_password.name.to_string()};
    let update = doc!{"$set": {edit_option: field_data.to_string()}};

    let input = coll.update_one(filter , update , None).await.unwrap();

    Ok(())



}

async fn password_find(current_User: User) -> mongodb::error::Result<()> {
    let mut password_name = String::new();
    io::stdin().read_line(&mut password_name).expect("Failed to read line");
    let password_name = password_name.trim();
    
    let handle = SpinnerBuilder::new().spinner(&DOTS).text("  Updating Data").start();
    
    let client = Client::with_uri_str("mongodb+srv://Admin:1234@cluster0.h7ieh.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await?;
    let db = client.database("Password");
    let coll = db.collection::<Password>("Password");

    let mut cursor = coll.find(doc! {"name": password_name.to_string()} , None).await?;

    std::thread::sleep(std::time::Duration::from_secs(3));
    handle.done();

    if let Some(password) = cursor.try_next().await? {
        let find_password = Password {
                name: password.name,
                username: password.username,
                password: password.password,
                email: password.email,
             };
             
            println!("[*] {}", find_password.name);
            println!("  {}", find_password.username);
            println!("  {}", find_password.password);
            println!("  {}", find_password.email);

            password_editor(find_password);
    }

    Ok(())

     
}



async fn change_name(current_User: User) -> mongodb::error::Result<()> {
    
    let mut new_name = String::new();
    io::stdin().read_line(&mut new_name).expect("Failed to read line");
    let new_name = new_name.trim();

    let handle = SpinnerBuilder::new().spinner(&DOTS).text("  Updating Data").start();

    let client = Client::with_uri_str("mongodb+srv://Admin:1234@cluster0.h7ieh.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await?;
    let db = client.database("User");
    let coll = db.collection::<edit_name>("User");
    
    let filter = doc!{"name": current_User.name};
    let update = doc!{"$set": {"name": new_name}};
    
    let input = coll.update_one(filter, update , None).await.unwrap(); 
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    handle.done();

    Ok(())

}

async fn change_password(current_User: User) -> mongodb::error::Result<()>  {

    let mut new_password = String::new();
    io::stdin().read_line(&mut new_password).expect("Failed to read line");
    let new_password = new_password.trim();
    
    let handle = SpinnerBuilder::new().spinner(&DOTS).text("  Updating Data").start();

    let client = Client::with_uri_str("mongodb+srv://Admin:1234@cluster0.h7ieh.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await?;
    let db = client.database("User");
    let coll = db.collection::<edit_pass>("Passwords");

    let filter = doc!{"name": current_User.name.to_string()};
    let update = doc!{"$set": {"password": new_password.to_string()}};

    let input = coll.update_one(filter , update , None).await.unwrap(); 

    std::thread::sleep(std::time::Duration::from_secs(3));
    handle.done();

    Ok(())


}


fn exit(current_User: User) {
    println!("[*] see you soon {}!" ,current_User.name);
    println!("[*] signed out")

}

fn abt_user(current_User: User) {
    println!("[*] credentials");
    println!("  name {}", current_User.name);
    println!("  password {}", current_User.password);

    println!("[*] 1. change name");
    println!("[*] 2. change password");
    println!("[*] 3. back to menu");

    let mut abt_user_option = String::new();       
    io::stdin().read_line(&mut abt_user_option).expect("Failed to read line");
    let abt_user_option = abt_user_option.trim(); 
    // add user option for change user data
    
    if abt_user_option == "1" {
        change_name(current_User);
    } else if abt_user_option == "2" {
        change_password(current_User);
    } else if abt_user_option == "3" {
        menu(current_User);

    } else {
        println!(" invalid option, try again");
        abt_user(current_User);
    }

}

fn abt_userAuth(current_User: User) {
    println!("about user");
    println!("[*] please re-enter your password");

    let mut password_entry = String::new();
    io::stdin().read_line(&mut password_entry).expect("Failed to read line");
    let password_entry = password_entry.trim();

    if password_entry == current_User.password {
            abt_user(current_User);

    } else {
        println!("incorrect password, try again");
        abt_userAuth(current_User);        

    }


}


async fn add(current_User: User) -> mongodb::error::Result<()> {
    
    println!("[*] enter password name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to get input");
    let name = name.trim();

    println!("[*] enter username");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to get input");
    let username =  username.trim();

    println!("[*] enter password");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to get input");
    let password = password.trim();

    println!("[*] enter email");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("Failed to get input");
    let email = email.trim();

    let handle = SpinnerBuilder::new().spinner(&DOTS).text("  Loading Data").start();

    let passwordadd = Password {
        name: name.to_string(),
        username: username.to_string(),
        password: password.to_string(),
        email: email.to_string(),
    };

    let client = Client::with_uri_str("mongodb+srv://Admin:1234@cluster0.h7ieh.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await?;
    let db = client.database("Passwords");
    let coll = db.collection::<Password>("Passwords");

    let insert = coll.insert_one(Password {name: passwordadd.name, password: passwordadd.password , username: passwordadd.username , email: passwordadd.email }, None ).await?;
    println!("[*] password added");

    Ok(())

}


//return result
async fn delete() {

}

async fn view(current_User: User) -> mongodb::error::Result<()> {

    let handle =  SpinnerBuilder::new().spinner(&DOTS).text("  Loading Data").start();

    let client = Client::with_uri_str("mongodb+srv://Admin:1234@cluster0.h7ieh.mongodb.net/myFirstDatabase?retryWrites=true&w=majority").await?; 
    let db = client.database("Passwords");
    let coll = db.collection::<Password>("Passwords");

    let mut cursor = coll.find(doc! {"uid": current_User.uid}, None).await?;

    std::thread::sleep(std::time::Duration::from_secs(3));
    handle.done();

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
    println!(" 3. user");
    println!(" 4. exit");

    let mut menu_option = String::new();
    io::stdin().read_line(&mut menu_option).expect("Failed to read line");
    let menu_option = menu_option.trim();

    if menu_option == "1" {
        add(current_User);
    } else if menu_option == "2"  {
        view(current_User);
    } else if menu_option == "3" {
        abt_userAuth(current_User);
    } else if menu_option == "4" {
        exit(current_User);
    } else {
        println!("invalid option, try again");
        menu(current_User);
    }
    


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
