use crate::diesel::RunQueryDsl;
use crate::establish_connection;
use crate::storage::models::{Block, NewBlock};
use crate::storage::schema::block;

pub fn write_block<'a>(new_block: NewBlock) -> Block {
    let connection = establish_connection();

    diesel::insert_into(block::table)
        .values(&new_block)
        .get_result(&connection)
        .expect("Error saving new block")
}
