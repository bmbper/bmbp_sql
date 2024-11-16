use crate::build::dql::{ConditionBuilder, TableBuilder};
use crate::wrapper::{RdbcDeleteWrapper, RdbcInsertWrapper, RdbcUpdateWrapper};

impl RdbcInsertWrapper {}

impl TableBuilder for RdbcInsertWrapper {}
impl ConditionBuilder for RdbcInsertWrapper {}

impl RdbcUpdateWrapper {}
impl TableBuilder for RdbcUpdateWrapper {}
impl ConditionBuilder for RdbcUpdateWrapper {}

impl RdbcDeleteWrapper {}
impl TableBuilder for RdbcDeleteWrapper {}
impl ConditionBuilder for RdbcDeleteWrapper {}
