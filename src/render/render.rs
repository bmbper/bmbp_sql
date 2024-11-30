use crate::{RdbcDeleteWrapper, RdbcInsertWrapper, RdbcQueryWrapper, RdbcUpdateWrapper, RdbcValue};

pub trait RdbcSQLRender {
    fn render_query(query: &mut RdbcQueryWrapper) -> (String, Vec<RdbcValue>);
    fn render_update(query: &mut RdbcUpdateWrapper) -> (String, Vec<RdbcValue>);
    fn render_insert(query: &mut RdbcInsertWrapper) -> (String, Vec<RdbcValue>);
    fn render_delete(query: &mut RdbcDeleteWrapper) -> (String, Vec<RdbcValue>);
}
