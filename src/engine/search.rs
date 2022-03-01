
// Search Engine For Version 1
// Runs queries to find the most repeated values in the DB to the one given
// repo_name (optional) => '%repo_name'
// languages_id => lang_idx OR lang_idy
// contributors_id => cont_idx OR cont_idy

// From the frontend you receive a JSON object with the query parameters

// []


use std::io;
use diesel::PgConnection;

use super::types::*;

pub fn browse(query: &Query, conn:&PgConnection) -> Result<List, io::Error> {
    // query would come from the frontend

    // retrieve the repos in db that are most alike to query repo
    let by_name = query.repo_name_eval(&conn)?;
    let by_lang = query.languages_eval(&conn)?;
    let by_cont = query.contributors_eval(&conn)?;

    // filter results

    // build List with filtered repos

    let repo = List { by_name, by_lang, by_cont };

    // return list
    Ok(repo)
    

    
}




