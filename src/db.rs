use mysql::*;
use mysql::prelude::*;

pub fn db_connect()->PooledConn{
    let url = "mysql://mike:Password@123@10.200.11.141/home";

    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    return pool.get_conn().unwrap();
    
}

pub fn db_write_otp(){

    let mut conn = db_connect();
    let a = "123";
    let b = "456";
    let hash =  hash(a,b).to_string().clone();
    let otps = vec![db_models::OTP {
        req_time: Local::now().naive_local(),
        hash:hash,
        device: "Sprinklers".to_string()
    }];

    conn.exec_batch(
        r"INSERT INTO otp_auth (req_time, hash, device)
          VALUES (:req_time, :hash, :device)",
          otps.iter().map(|p| params! {
            "req_time" => p.req_time,
            "hash" => p.hash.to_string().clone(),
            "device" => &p.device,
        })
    ).ok();

}
