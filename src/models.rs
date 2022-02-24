use crate::schema::sites;

#[derive(Queryable)]
pub struct Site {
    pub id: i32,
    pub site_name: String,
    pub site_url: String,
}

#[table_name="sites"]
#[derive(Insertable)]
pub struct newSite {
    pub site_name: String,
    pub site_url: String,
}