extern crate diesel;
use self::diesel::prelude::*;

use crate::establish_connection;
use crate::storage::models::Block;

pub fn all_blocks() -> Result<Vec<Block>, diesel::result::Error> {
    use crate::storage::schema::block::dsl::*;
    let connection = establish_connection();
    let results = block
        .load::<Block>(&connection)
        .expect("Error loading blocks");

    Ok(results)
}
