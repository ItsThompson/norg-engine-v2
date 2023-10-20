use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Project {
    _id : i32,
    _name : String,
}

const DATABASE:&str = "/Users/thompsontong/Documents/projects/2023/norg/sample-data/sample.db";

pub fn get_projects() -> Result<()> {
    let conn = Connection::open(DATABASE)?;

    // SELECT projects.id, projects.name, properties.property_key, properties.property_value FROM projects 
    // INNER JOIN properties ON projects.id = properties.project_id;
    let mut statement = conn.prepare("SELECT * FROM \"main\".\"projects\"")?;
    let project_iter  = statement.query_map( [], |row| {
        Ok(Project{
            _id: row.get(0)?,
            _name: row.get(1)?,
        })
    })?;

    for project in project_iter {
        println!("Found project: {:?}", project.unwrap());
    }

    Ok(())
}
