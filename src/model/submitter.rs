use crate::schema::submitters;
use diesel::{
    expression::bound::Bound,
    insert_into,
    pg::PgConnection,
    query_dsl::{QueryDsl, RunQueryDsl},
    result::QueryResult,
    sql_types, ExpressionMethods,
};
use ipnetwork::IpNetwork;

mod get;

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "submitters"]
#[primary_key("submitter_id")]
pub struct Submitter {
    pub id: i32,
    pub ip: IpNetwork,
    pub banned: bool,
}

#[derive(Insertable, Debug)]
#[table_name = "submitters"]
struct NewSubmitter<'a> {
    #[column_name = "ip_address"]
    ip: &'a IpNetwork,
}

type AllColumns = (
    submitters::submitter_id,
    submitters::ip_address,
    submitters::banned,
);

type All = diesel::dsl::Select<submitters::table, AllColumns>;
type WithIp<'a> = diesel::dsl::Eq<submitters::ip_address, Bound<sql_types::Inet, &'a IpNetwork>>;
type ByIp<'a> = diesel::dsl::Filter<All, WithIp<'a>>;

impl Submitter {
    fn all() -> All {
        submitters::table.select((
            submitters::submitter_id,
            submitters::ip_address,
            submitters::banned,
        ))
    }

    pub fn by_ip(ip: &IpNetwork) -> ByIp {
        Submitter::all().filter(submitters::ip_address.eq(ip))
    }

    pub fn insert(conn: &PgConnection, ip: &IpNetwork) -> QueryResult<Submitter> {
        let new = NewSubmitter { ip };

        insert_into(submitters::table).values(&new).get_result(conn)
    }
}
