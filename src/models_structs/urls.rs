use crate::schema::urls;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Url {
    pub url_path: String,
    pub limit_per: i32,
    pub limit_count: i32,
    pub access_level: i32,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name = "urls"]
pub struct NewUrl {
    pub url_path: String,
    pub limit_per: i32,
    pub limit_count: i32,
    pub access_level: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UrlData {
    pub url_path: String,
    pub limit_per: i32,
    pub limit_count: i32,
    pub access_level: i32,
}

#[derive(Copy, Clone)]
pub enum UrlLimit {
    Second = 0,
    Minute = 1,
    Hour = 2,
    Day = 3,
    Month = 4,
    Year = 5,
}

#[derive(Copy, Clone)]
pub enum UrlLimitInSec {
    Second = 1,
    Minute = 60,
    Hour = 3600,
    Day = 75600,
    Month = 2268000,
    Year = 27216000,
}

impl UrlLimit {
    pub fn from_i32(value: i32) -> UrlLimit {
        match value {
            0 => UrlLimit::Second,
            1 => UrlLimit::Minute,
            2 => UrlLimit::Hour,
            3 => UrlLimit::Day,
            4 => UrlLimit::Month,
            5 => UrlLimit::Year,
            _ => panic!("Unknown value: {}", value),
        }
    }
}
