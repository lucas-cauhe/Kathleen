
use diesel::{PgConnection, PgArrayExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods};

use crate::models::{Repo};
use std::io;
pub struct List {
    pub by_name: Vec<Repo>,
    pub by_lang: Vec<Repo>,
    pub by_cont: Vec<Repo>,
}

pub struct Query {
    pub repo_name: String,
    pub languages_id: Vec<i32>,
    pub contributors_id: Vec<i32>
}

impl Query {
    pub fn repo_name_eval(&self, conn: &PgConnection) -> Result<Vec<Repo>, io::Error> {
        use crate::schema::repos::dsl::*;
        //let exp = format!("%{}", self.repo_name);
        
        let eval: Vec<Repo> = repos.filter(repo_name.like(&self.repo_name))
            .load::<Repo>(conn)
            .expect("Error loading repo names");
        
        Ok(eval)
    }
    pub fn languages_eval(&self, conn: &PgConnection) -> Result<Vec<Repo>, io::Error> {
        use crate::schema::repos;

        let eval = repos::table.filter(repos::languages_id.contains(&self.languages_id))
            .load::<Repo>(conn)
            .expect("Error loading languages");

        Ok(eval)
    }
    pub fn contributors_eval(&self, conn: &PgConnection) -> Result<Vec<Repo>, io::Error> {
        use crate::schema::repos;
        
        let eval = repos::table.filter(repos::contributors_id.contains(&self.contributors_id))
            .load::<Repo>(conn)
            .expect("Error loading contributors");
        
        Ok(eval)
    }
}