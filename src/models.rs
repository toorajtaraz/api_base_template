use crate::actix::Addr;
use crate::actors::database::DbActor;

pub struct AppState {
    pub db: Addr<DbActor>,
    pub salt: String,
    pub secret: String,
}
