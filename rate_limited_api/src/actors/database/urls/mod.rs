use crate::actix::{Handler, Message};
use crate::actors::database::DbActor;
use crate::diesel::prelude::*;
use crate::models_structs::urls::{NewUrl, Url};
use crate::schema::urls::dsl::{access_level, limit_count, limit_per, url_path, urls};

#[derive(Message)]
#[rtype(result = "QueryResult<Url>")]
pub struct CreateOrUpdateUrl {
    pub url_path: String,
    pub access_level: i32,
    pub limit_per: i32,
    pub limit_count: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Url>")]
pub struct DeleteUrl {
    pub url_path: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Url>")]
pub struct GetUrl {
    pub url_path: String,
}

impl Handler<CreateOrUpdateUrl> for DbActor {
    type Result = QueryResult<Url>;

    fn handle(&mut self, msg: CreateOrUpdateUrl, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let new_url = NewUrl {
            url_path: msg.url_path,
            access_level: msg.access_level,
            limit_per: msg.limit_per,
            limit_count: msg.limit_count,
        };

        diesel::insert_into(urls)
            .values(new_url)
            .on_conflict(url_path)
            .do_update()
            .set((
                access_level.eq(msg.access_level),
                limit_per.eq(msg.limit_per),
                limit_count.eq(msg.limit_count),
            ))
            .get_result::<Url>(&conn)
    }
}

impl Handler<DeleteUrl> for DbActor {
    type Result = QueryResult<Url>;

    fn handle(&mut self, msg: DeleteUrl, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::delete(urls)
            .filter(url_path.eq(msg.url_path))
            .get_result::<Url>(&conn)
    }
}

impl Handler<GetUrl> for DbActor {
    type Result = QueryResult<Url>;

    fn handle(&mut self, msg: GetUrl, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        urls.filter(url_path.eq(msg.url_path))
            .get_result::<Url>(&conn)
    }
}
