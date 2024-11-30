use crate::db::DataBase;
use crate::{RdbcQueryWrapper, RdbcValue};

mod client;
mod render;

pub fn render_query(query: &mut RdbcQueryWrapper, db_type: DataBase) -> (String, Vec<RdbcValue>) {
    ("".to_string(), vec![])
}
