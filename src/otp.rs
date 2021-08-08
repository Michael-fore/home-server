
use chrono::Local;
use boringauth::oath::TOTPBuilder;

#[allow(deprecated)]
pub fn get_otp()->String{
    let key = "PuzHmGpcoTn6O5VgE6AM";
        
    let code = TOTPBuilder::new()
    .ascii_key(&key)
    .hash_function(boringauth::oath::HashFunction::Sha1)
    .finalize()
    .unwrap()
    .generate();
    return code;
}


pub fn get_time()->i64{
    return Local::now().timestamp();
}

#[tokio::main]
pub async fn stop_water()-> Result<String, reqwest::Error> {
    let body = reqwest::get("http://10.200.11.162/endWater")
    .await?
    .text()
    .await?;
    println!("body = {:?}", body);
    println!("body = {:?}", get_otp());
    Ok(body)
}
#[tokio::main]
pub async fn start_water()-> Result<String, reqwest::Error> {
    let body = reqwest::get("http://10.200.11.162/startWater")
    .await?
    .text()
    .await?;
    println!("body = {:?}", body);
    println!("body = {:?}", get_otp());
    Ok(body)
}

