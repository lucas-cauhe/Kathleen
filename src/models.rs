
use crate::schema::repos;

// REPOS TABLE
#[derive(Queryable)]
pub struct Repo {
    pub id: i32,
    pub repo_name: String,
    pub repo_url: String,
    pub languages_id: Vec<i32>,
    pub contributors_id: Vec<i32>
}

#[derive(Insertable)]
#[table_name="repos"]
pub struct NewRepo {
    pub repo_name: String,
    pub repo_url: String,
    pub languages_id: Vec<i32>,
    pub contributors_id: Vec<i32>
}


// LANGUAGES TABLE

#[derive(Queryable, QueryId)]
pub struct Language {
    pub id: i32,
    pub language_name: String
}



// CONTRIBUTORS TABLE
#[derive(Queryable)]
pub struct Contributor {
    pub id: i32,
    pub contributor: String,
    pub contributor_url: String
}