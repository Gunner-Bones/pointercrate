use super::User;
use crate::{error::PointercrateError, operation::Delete, schema::members, Result};
use diesel::{delete, ExpressionMethods, PgConnection, RunQueryDsl};

impl Delete for User {
    fn delete(self, connection: &PgConnection) -> Result<()> {
        delete(members::table)
            .filter(members::member_id.eq(self.id))
            .execute(connection)
            .map(|_| ())
            .map_err(PointercrateError::database)
    }
}
