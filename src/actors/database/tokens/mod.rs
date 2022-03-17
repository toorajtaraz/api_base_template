use crate::actix::{Handler, Message};
use crate::actors::database::DbActor;
use crate::diesel::prelude::*;
use crate::models_structs::tokens::{NewToken, Token};
use crate::schema::tokens::dsl::{id, tokens};

#[derive(Message)]
#[rtype(result = "QueryResult<Token>")]
pub struct CreateToken {
    pub user_id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Token>")]
pub struct DeleteToken {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Token>")]
pub struct GetToken {
    pub id: i32,
}

impl Handler<CreateToken> for DbActor {
    type Result = QueryResult<Token>;

    fn handle(&mut self, msg: CreateToken, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        let new_token = NewToken {
            user_id: msg.user_id,
        };

        diesel::insert_into(tokens)
            .values(new_token)
            .get_result::<Token>(&conn)
    }
}

impl Handler<DeleteToken> for DbActor {
    type Result = QueryResult<Token>;

    fn handle(&mut self, msg: DeleteToken, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::delete(tokens)
            .filter(id.eq(msg.id))
            .get_result::<Token>(&conn)
    }
}

impl Handler<GetToken> for DbActor {
    type Result = QueryResult<Token>;

    fn handle(&mut self, msg: GetToken, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        tokens.filter(id.eq(msg.id)).get_result::<Token>(&conn)
    }
}
