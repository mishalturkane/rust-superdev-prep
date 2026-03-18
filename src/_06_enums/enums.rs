#[derive(Debug)]
#[allow(dead_code)]
pub enum Weather {
    Sunny,
    Rainy,
    Cloudy,
    Snowy,
}

pub fn enums(){
    let current_weather = Weather::Sunny;
    println!("{:?}", current_weather);
}