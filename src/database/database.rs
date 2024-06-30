use postgres::Error as PostgresError;
use postgres::{ Client, NoTls };
use crate::constants::constants::DB_URL;

//set_database function
pub fn set_database() -> Result<(), PostgresError> {
    //Connect to database
    let mut client = Client::connect(DB_URL, NoTls)?;

    //Create table
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL UNIQUE,
            password VARCHAR NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )?;
    Ok(())
}