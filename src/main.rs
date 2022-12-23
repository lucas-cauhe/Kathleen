#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate hnsw;

pub mod models;
pub mod schema;
pub mod types;
mod engine;

use types::Query;
use engine::{search::main::browse};
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


fn retrieve_repos(conn: &PgConnection){
    use schema::repos::dsl::*;

    
    let results = repos
        .load::<Repo>(conn)
        .expect("Error loading repos");
    
    for repo in results{
        println!("URL -> {}", repo.repo_url);
        println!("Name -> {}", repo.repo_name);
    }
}


fn insert_repos(conn: &PgConnection, repo: &NewRepo) -> Result<i32, Error>{
    
    let id: i32 = diesel::insert_into(schema::repos::table)
        .values(repo)
        .returning(schema::repos::id)
        .get_result(conn)?;
    
    Ok(id)
}

fn main(){
    
    println!("Connecting to DB");
    let connection = connect();

    /* let lids = vec![1, 2, 3];
    let cids  = vec![1, 2, 3];
    let repo = NewRepo {
        repo_name: "Pepper Sprout".to_string(),
        repo_url: "https://github.com/lucas-cauhe/Pepper_Sprout".to_string(),
        languages_id: lids.to_vec(),
        contributors_id: cids.to_vec(),
    };
    let _repo_id = insert_repos(&connection, &repo); */
    /* let facebookId = insert_repos(&connection, "Facebook", "facebook.com", );
    let instagramId = insert_repos(&connection, "Instagram", "instagram.com"); */
    
    //println!("Searching Repos");
    //retrieve_repos(&connection);

    let query = Query {
        repo_name: Some("".to_string()),
        contributors_id: [1, 2].to_vec(),
        languages_id: [].to_vec()
    };

    let result = browse(&query, &connection).unwrap();
    
    println!("Results for searching names: ");
    for repo in result.by_name {
        println!("Repo with name {n} and id {id}", n=repo.repo_name, id=repo.id);
    }

    println!("Results for searching langs: ");
    for repo in result.by_lang {
        println!("Repo with name {n} and id {id}", n=repo.repo_name, id=repo.id);
    }

    println!("Results for searching conts: ");
    for repo in result.by_cont {
        println!("Repo with name {n} and id {id}", n=repo.repo_name, id=repo.id);
    }

}