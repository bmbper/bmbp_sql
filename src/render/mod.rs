use crate::db::DataBase;
use crate::render::client::{MysqlSQLRender, SqliteSQLRender};
use crate::render::render::RdbcSQLRender;
use crate::{RdbcDeleteWrapper, RdbcInsertWrapper, RdbcQueryWrapper, RdbcUpdateWrapper, RdbcValue};

mod client;
mod render;

pub fn render_query(query: &RdbcQueryWrapper, db_type: DataBase) -> (String, Vec<RdbcValue>) {
    match db_type {
        DataBase::MySql => MysqlSQLRender::render_query(query),
        DataBase::Sqlite => SqliteSQLRender::render_query(query),
        DataBase::Postgres => client::PgSQLRender::render_query(query),
        DataBase::Oracle => client::OracleSQLRender::render_query(query),
    }
}
pub fn render_insert(insert: &RdbcInsertWrapper, db_type: DataBase) -> (String, Vec<RdbcValue>) {
    match db_type {
        DataBase::MySql => MysqlSQLRender::render_insert(insert),
        DataBase::Sqlite => SqliteSQLRender::render_insert(insert),
        DataBase::Postgres => client::PgSQLRender::render_insert(insert),
        DataBase::Oracle => client::OracleSQLRender::render_insert(insert),
    }
}
pub fn render_update(update: &RdbcUpdateWrapper, db_type: DataBase) -> (String, Vec<RdbcValue>) {
    match db_type {
        DataBase::MySql => MysqlSQLRender::render_update(update),
        DataBase::Sqlite => SqliteSQLRender::render_update(update),
        DataBase::Postgres => client::PgSQLRender::render_update(update),
        DataBase::Oracle => client::OracleSQLRender::render_update(update),
    }
}

pub fn render_delete(delete: &RdbcDeleteWrapper, db_type: DataBase) -> (String, Vec<RdbcValue>) {
    match db_type {
        DataBase::MySql => MysqlSQLRender::render_delete(delete),
        DataBase::Sqlite => SqliteSQLRender::render_delete(delete),
        DataBase::Postgres => client::PgSQLRender::render_delete(delete),
        DataBase::Oracle => client::OracleSQLRender::render_delete(delete),
    }
}
