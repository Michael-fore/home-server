
mod otp;
mod iot;

#[macro_use] extern crate rocket;
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/time")]
fn time() -> String {
    
    return otp::get_time().to_string();
}

#[get("/otp")]
fn _otp() -> String {
    
    return otp::get_otp();
}
 
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index,time,_otp])
}

/*
fn main () {
    print!("Running!");
    //iot::start_water();
    let _ = otp::stop_water();

}
*/