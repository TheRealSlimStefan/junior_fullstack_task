#[macro_use] extern crate rocket;
use rand::prelude::*;
use chrono::Datelike;

#[get("/calculateDisselUsageForDistance?<distance>&<yearOfProduction>&<fuelUsagePer100KM>")]
fn calculateDisselUsageForDistance(distance: u16, yearOfProduction: u16, fuelUsagePer100KM: u8) -> String {
    if yearOfProduction < 1885 || yearOfProduction >  chrono::Utc::now().year() as u16 {
        return ("The year of production can't be less than the year of production of the first car and can't be greater than the current year ").to_string();
    } 
    else{
        let fuelUsage: f64 = (f64::from(distance) / 100.0) * f64::from(fuelUsagePer100KM);

        fuelUsage.to_string()
    } 
}

#[get("/probabilityOfUnitInjectorFail/<vin>")]
fn probabilityOfUnitInjectorFail(vin: String) -> String {
    if vin.chars().count() != 21 {
        return ("Vin number should has 17 characters").to_string();
    } 
    else{
        let mut rng = rand::thread_rng();
        let randomNumber: f64 = rng.gen();
        let failProbability: f64 = (randomNumber * 100.0).round() / 100.0;

        failProbability.to_string()  
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![calculateDisselUsageForDistance, probabilityOfUnitInjectorFail])
}
