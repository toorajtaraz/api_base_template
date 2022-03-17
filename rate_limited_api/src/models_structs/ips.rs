use crate::schema::ips;
use diesel::{Insertable, Queryable};
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Ip {
    pub id: i32,
    pub ip: IpNetwork,
    pub url_path: String,
    pub first_access: chrono::NaiveDateTime,
    pub last_access: chrono::NaiveDateTime,
    pub access_count: i32,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "ips"]
pub struct NewIp {
    pub ip: IpNetwork,
    pub first_access: chrono::NaiveDateTime,
    pub last_access: chrono::NaiveDateTime,
    pub url_path: String,
    pub access_count: i32,
}

#[derive(Serialize, Deserialize)]
pub struct IpData {
    pub ip: IpNetwork,
    pub first_access: chrono::NaiveDateTime,
    pub last_access: chrono::NaiveDateTime,
    pub url_path: String,
    pub access_count: i32,
}
