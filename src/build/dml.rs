use crate::build::dql::ConditionBuilder;
use crate::{RdbcCondition, RdbcDeleteWrapper, RdbcInsertWrapper, RdbcUpdateWrapper};

impl RdbcInsertWrapper {}

impl RdbcUpdateWrapper {}
impl ConditionBuilder for RdbcUpdateWrapper {
    fn condition_mut(&mut self) -> &mut RdbcCondition {
        if self.where_condition.is_none() {
            self.where_condition = Some(RdbcCondition::new());
        }
        self.where_condition.as_mut().unwrap()
    }
}
impl RdbcDeleteWrapper {}

impl ConditionBuilder for RdbcDeleteWrapper {
    fn condition_mut(&mut self) -> &mut RdbcCondition {
        if self.where_condition.is_none() {
            self.where_condition = Some(RdbcCondition::new());
        }
        self.where_condition.as_mut().unwrap()
    }
}
