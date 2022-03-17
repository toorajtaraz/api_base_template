use crate::actix::Addr;
use crate::actors::database::urls::CreateOrUpdateUrl;
use crate::actors::database::DbActor;

pub async fn add_urls(db: Addr<DbActor>) -> Result<(), ()> {
    let db = db.clone();

    match db
        .send(CreateOrUpdateUrl {
            url_path: format!("/api/service/test"),
            access_level: 1,
            limit_per: 1,
            limit_count: 10,
        })
        .await
    {
        Ok(Ok(res)) => {
            println!("Added: {} to DB", res.url_path);
        }
        _ => {
            eprintln!("Adding to DB failed!");
            return Err(());
        }
    }
    match db
        .send(CreateOrUpdateUrl {
            url_path: format!("/api/service/test_root"),
            access_level: 2,
            limit_per: 1,
            limit_count: 10,
        })
        .await
    {
        Ok(Ok(res)) => {
            println!("Added: {} to DB", res.url_path);
        }
        _ => {
            eprintln!("Adding to DB failed!");
            return Err(());
        }
    }
    match db
        .send(CreateOrUpdateUrl {
            url_path: format!("/api/auth/test"),
            access_level: 0,
            limit_per: 1,
            limit_count: 5,
        })
        .await
    {
        Ok(Ok(res)) => {
            println!("Added: {} to DB", res.url_path);
        }
        _ => {
            eprintln!("Adding to DB failed!");
            return Err(());
        }
    }
    match db
        .send(CreateOrUpdateUrl {
            url_path: format!("/api/auth/login"),
            access_level: 0,
            limit_per: 1,
            limit_count: 5,
        })
        .await
    {
        Ok(Ok(res)) => {
            println!("Added: {} to DB", res.url_path);
        }
        _ => {
            eprintln!("Adding to DB failed!");
            return Err(());
        }
    }
    match db
        .send(CreateOrUpdateUrl {
            url_path: format!("/api/auth/signup"),
            access_level: 0,
            limit_per: 1,
            limit_count: 5,
        })
        .await
    {
        Ok(Ok(res)) => {
            println!("Added: {} to DB", res.url_path);
        }
        _ => {
            eprintln!("Adding to DB failed!");
            return Err(());
        }
    }
    Ok(())
}
