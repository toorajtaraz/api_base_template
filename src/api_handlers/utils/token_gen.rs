use crate::actix::Addr;
use crate::actors::database::tokens::{CreateToken, GetToken};
use crate::actors::database::DbActor;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    token_id: i32,
    user_id: i32,
    user_access_level: i32,
    exp: usize,
}

pub async fn gen_token(
    user_id: i32,
    user_access_level: i32,
    db: Addr<DbActor>,
    secret: String,
) -> Result<String, ()> {
    match db.clone().send(CreateToken { user_id: user_id }).await {
        Ok(Ok(val)) => {
            let claims = Claims {
                token_id: val.id,
                user_id: user_id,
                user_access_level: user_access_level,
                exp: chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::hours(5))
                    .unwrap()
                    .timestamp() as usize,
            };
            match encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_ref()),
            ) {
                Ok(val) => Ok(val),
                _ => Err(()),
            }
        }
        _ => {
            return Err(());
        }
    }
}

pub async fn verify_token(
    token: String,
    secret: String,
    db: Addr<DbActor>,
) -> Result<(i32, i32), ()> {
    let token = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(val) => val,
        _ => {
            return Err(());
        }
    };
    let elapsed = chrono::Utc::now()
        .naive_utc()
        .signed_duration_since(chrono::NaiveDateTime::from_timestamp(
            token.claims.exp as i64,
            0,
        ))
        .num_seconds();
    if elapsed >= 5 * 60 * 60 {
        return Err(());
    }
    match db
        .send(GetToken {
            id: token.claims.token_id,
        })
        .await
    {
        Ok(Ok(_)) => Ok((token.claims.user_id, token.claims.user_access_level)),
        _ => Err(()),
    }
}
