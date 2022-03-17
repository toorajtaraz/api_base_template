use crate::actix::{Handler, Message};
use crate::actors::database::DbActor;
use crate::diesel::prelude::*;
use crate::models_structs::ips::{Ip, NewIp};
use crate::models_structs::urls::Url;
use crate::schema::ips::dsl::{access_count, first_access, id, ip, ips, last_access, url_path};
use crate::schema::urls::dsl::urls;
use ipnetwork::IpNetwork;

#[derive(Message)]
#[rtype(result = "QueryResult<Ip>")]
pub struct CreateIp {
    pub ip: IpNetwork,
    pub access: chrono::NaiveDateTime,
    pub url_path: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Ip>")]
pub struct NewAccess {
    pub id: i32,
    pub access: chrono::NaiveDateTime,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Ip>")]
pub struct ResetAccess {
    pub id: i32,
    pub access: chrono::NaiveDateTime,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Ip>")]
pub struct DeleteIp {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Ip>>")]
pub struct GetIps;

#[derive(Message)]
#[rtype(result = "QueryResult<Ip>")]
pub struct GetIp {
    pub ip: IpNetwork,
    pub url_path: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<(Ip, Url)>")]
pub struct GetIpUrl {
    pub ip: IpNetwork,
    pub url_path: String,
}

impl Handler<CreateIp> for DbActor {
    type Result = QueryResult<Ip>;

    fn handle(&mut self, msg: CreateIp, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let new_ip = NewIp {
            ip: msg.ip,
            first_access: msg.access,
            last_access: msg.access,
            url_path: msg.url_path,
            access_count: 1 as i32,
        };

        diesel::insert_into(ips)
            .values(new_ip)
            .get_result::<Ip>(&conn)
    }
}

impl Handler<NewAccess> for DbActor {
    type Result = QueryResult<Ip>;

    fn handle(&mut self, msg: NewAccess, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let handle = diesel::update(ips).filter(id.eq(msg.id));
        handle
            .set((
                last_access.eq(msg.access),
                access_count.eq(access_count + 1),
            ))
            .get_result::<Ip>(&conn)
    }
}

impl Handler<ResetAccess> for DbActor {
    type Result = QueryResult<Ip>;

    fn handle(&mut self, msg: ResetAccess, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let handle = diesel::update(ips).filter(id.eq(msg.id));
        handle
            .set((
                first_access.eq(msg.access),
                last_access.eq(msg.access),
                access_count.eq(1),
            ))
            .get_result::<Ip>(&conn)
    }
}

impl Handler<DeleteIp> for DbActor {
    type Result = QueryResult<Ip>;

    fn handle(&mut self, msg: DeleteIp, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::delete(ips)
            .filter(id.eq(msg.id))
            .get_result::<Ip>(&conn)
    }
}

impl Handler<GetIps> for DbActor {
    type Result = QueryResult<Vec<Ip>>;

    fn handle(&mut self, _msg: GetIps, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        ips.get_results::<Ip>(&conn)
    }
}

impl Handler<GetIp> for DbActor {
    type Result = QueryResult<Ip>;

    fn handle(&mut self, msg: GetIp, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        ips.filter(ip.eq(msg.ip).and(url_path.eq(msg.url_path)))
            .get_result::<Ip>(&conn)
    }
}

impl Handler<GetIpUrl> for DbActor {
    type Result = QueryResult<(Ip, Url)>;

    fn handle(&mut self, msg: GetIpUrl, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        ips.inner_join(urls)
            .filter(ip.eq(msg.ip).and(url_path.eq(msg.url_path)))
            .get_result::<(Ip, Url)>(&conn)
    }
}
