#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use dotenv::dotenv;
use std::{env, i32};
use models::*;

fn connect() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("something happened");
    PgConnection::establish(&db_url)
        .expect("Error while connecting to DB")
}


fn retrieve_sites(conn: &PgConnection){
    use schema::sites::dsl::*;

    
    let results = sites
        .load::<Site>(conn)
        .expect("Error loading sites");
    
    for site in results{
        println!("URL -> {}", site.site_url);
        println!("Name -> {}", site.site_name);
    }
}


fn insert_sites(conn: &PgConnection, name: &str, url: &str) -> Result<i32, Error>{
    let site = newSite {
        site_name: name.to_string(),
        site_url: url.to_string(),
    };
    let id: i32 = diesel::insert_into(schema::sites::table)
        .values(&site)
        .returning(schema::sites::id)
        .get_result(conn)?;
    
    Ok(id)
}


fn main(){
    
    println!("Connecting to DB");
    let connection = connect();

    let googleId = insert_sites(&connection, "Google", "google.com");
    let facebookId = insert_sites(&connection, "Facebook", "facebook.com");
    let instagramId = insert_sites(&connection, "Instagram", "instagram.com");
    
    println!("Retrieving Sites");
    retrieve_sites(&connection);

}