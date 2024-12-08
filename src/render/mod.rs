use crate::db::DataBase;
use crate::render::client::{MysqlSQLRender, SqliteSQLRender};
use crate::render::render::RdbcSQLRender;
use crate::RdbcQueryWrapper;
use bmbp_rdbc_type::RdbcValue;

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
