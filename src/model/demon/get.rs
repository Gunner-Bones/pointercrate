use super::Demon;
use crate::{error::PointercrateError, operation::Get, Result};
use diesel::{result::Error, PgConnection, RunQueryDsl};

impl Get<String> for Demon {
    fn get(name: String, connection: &PgConnection) -> Result<Self> {
        match Demon::by_name(&name).first(connection) {
            Ok(demon) => Ok(demon),
            Err(Error::NotFound) =>
                Err(PointercrateError::ModelNotFound {
                    model: "Demon",
                    identified_by: name,
                }),
            Err(err) => Err(PointercrateError::database(err)),
        }
    }
}

impl Get<i16> for Demon {
    fn get(position: i16, connection: &PgConnection) -> Result<Self> {
        match Demon::by_position(position).first(connection) {
            Ok(demon) => Ok(demon),
            Err(Error::NotFound) =>
                Err(PointercrateError::ModelNotFound {
                    model: "Demon",
                    identified_by: position.to_string(),
                }),
            Err(err) => Err(PointercrateError::database(err)),
        }
    }
}

impl Get<i16, i16>(pos: i16, ver: i16) for Demon {
    fn get(pos: pos, connection: &PgConnection) -> Result<Self> {
        match Demon::by_position(pos).filter(demons::version.eq(ver).first(connection)) {
            Ok(demon) => Ok(demon),
            Err(Error::NotFound) =>
                Err(PointercrateError::ModelNotFound {
                    model: "Demon",
                    identified_by: format!("{},{}",pos.to_string(),ver.to_string()),
                }),
            Err(err) => Err(PointercrateError::database(err)),
        }
    }
}
