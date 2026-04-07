use std::fs;
use serde::Serialize;
use std::time::Duration;
use std::thread;

const SENSOR_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

#[derive(Serialize)] 
struct TempData {
    sensor_name: String,
    value_celsius: f32,
}

fn main() {

    println!("Demarage de la surveillance");

    loop {

        let result = fs::read_to_string(SENSOR_PATH);

        match result {

            Ok(content) => {
                if let Ok(temp_raw) = content.trim().parse::<f32>() {
                    
                    let data = TempData {
                        sensor_name: "CPU".to_string(),
                        value_celsius: temp_raw / 1000.0,
                    };
                    
                    let json = serde_json::to_string(&data).unwrap();
                    println!("{}", json);
                }
            }
            Err(e) => {
                eprintln!("Erreur de lecture du capteur: {}", e);
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}
