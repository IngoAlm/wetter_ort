// Programm zum Anzeigen von Wetterdaten eines Ortes.

mod environment;
use chrono::{Datelike, Local, Timelike};
use reqwest::blocking::get;
use serde_json::{Value, from_str};
use std::env;
use std::io;

fn read_city() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let city = &args[1];
        return city.to_string();
    } else {
        let mut city = String::from("");
        println!("Bitte gib eine Stadt ein: ");
        // Lese die Eingabe vom Benutzer
        io::stdin()
            .read_line(&mut city)
            .expect("Fehler beim Lesen der Eingabe");
        // Entferne das Zeilenumbruchzeichen am Ende.
        let city_ret = city.trim();
        city_ret.to_string()
    }
}

fn date_time() {
    // Datum und Uhrzeit mit chrono ausgeben.
    let now = Local::now();
    println!(
        "Heute ist {}, der {}.{}.{}",
        now.weekday(),
        now.day(),
        now.month(),
        now.year()
    );
    println!(
        "Es ist {}.{}.{} Uhr.\n",
        now.hour(),
        now.minute(),
        now.second()
    );
}

fn main() {
    let city = read_city();

    let api_key = environment::API_KEY;

    // Wetterdaten von Openweathermap.org holen.
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&lang=de&appid={}",
        city, api_key
    );
    let response = get(url).unwrap();
    // Wetterdaten in JSON Datei wandeln.
    let response_text: String = response.text().unwrap();
    let json: Value = from_str(&response_text).expect("Json ist nicht richtig formatiert!");

    // Daten aus der JSON Datei holen.
    let ortsname = json["name"].as_str().unwrap();
    let temp_kelvin = json["main"]["temp"].as_f64().unwrap();
    // Temperatur in Celsius umrechnen.
    let temp_celsius = temp_kelvin - 273.15;
    let luftdruck = json["main"]["pressure"].as_f64().unwrap();
    let wind = json["wind"]["speed"].as_f64().unwrap();
    // Wind von m/s in km/h umrechnen.
    let wind_kmh = wind * 3.6;
    let windrichtung = json["wind"]["deg"].as_u64().unwrap();
    let luftfeuchte = json["main"]["humidity"].as_u64().unwrap();
    let wolken = json["clouds"]["all"].as_u64().unwrap();

    // Datum und Uhrzeit ausgeben.
    date_time();

    println!("Die Wetterdaten von {:?}:\n", ortsname);
    println!("Temperatur  : {}° Celsius", temp_celsius.round());
    println!("Luftdruck   : {} hpa", luftdruck);
    println!("Wind        : {} km/h", wind_kmh.round());
    println!("Windrichtung: {}°", windrichtung);
    println!("Luftfeuchte : {:?}%", luftfeuchte);
    println!("Bewölkung   : {}%", wolken);
    println!("");

    if temp_celsius > 20.0 {
        println!("Ächz, eine mörderische Hitze!");
    } else if temp_celsius >= 15.0 {
        println!("Puh, ganz schön Warm!");
    } else if temp_celsius >= 5.0 {
        println!("Naja, langsam wird es besser")
    } else {
        println!("Wow, echt tolles Wetter");
    }
    println!("\n");
}
