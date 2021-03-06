use super::{PartialRecord, RecordStatus};
use crate::{
    error::PointercrateError,
    operation::{Paginate, Paginator},
    schema::records,
    Result,
};
use diesel::{pg::Pg, query_builder::BoxedSelectStatement, PgConnection, QueryDsl, RunQueryDsl};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecordPagination {
    #[serde(rename = "before")]
    before_id: Option<i32>,

    #[serde(rename = "after")]
    after_id: Option<i32>,

    limit: Option<i64>,

    progress: Option<i16>,
    #[serde(rename = "progress__lt")]
    progress_lt: Option<i16>,
    #[serde(rename = "progress__gt")]
    progress_gt: Option<i16>,

    status: Option<RecordStatus>,

    player: Option<i32>,
    submitter: Option<i32>,
    demon: Option<String>,
    video: Option<String>,
}

impl RecordPagination {
    filter_method!(records[
        progress = progress,
        progress < progress_lt,
        progress > progress_gt,
        status_ = status,
        player = player,
        submitter = submitter,
        demon = demon,
        video = video
    ]);
}

impl Paginator for RecordPagination {
    navigation!(records, id, before_id, after_id);
}

impl Paginate<RecordPagination> for PartialRecord {
    fn load(pagination: &RecordPagination, connection: &PgConnection) -> Result<Vec<Self>> {
        let select = records::table.select((
            records::id,
            records::progress,
            records::video,
            records::status_,
            records::player,
            records::submitter,
            records::demon,
        ));

        let mut query = pagination.filter(select.into_boxed());

        filter!(query[
            records::id > pagination.after_id,
            records::id < pagination.before_id
        ]);

        query
            .limit(pagination.limit.unwrap_or(50))
            .load(connection)
            .map_err(PointercrateError::database)
    }
}
