use mysql::chrono::*;

pub struct OTP {
    pub req_time: NaiveDateTime,
    pub hash: String,
    pub device: String
}