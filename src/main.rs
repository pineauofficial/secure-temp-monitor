use std::fs;
use serde::Serialize;

#[derive(Serialize)] 
struct TempData {
    sensor_name: String,
    value_celsius: f32,
}

fn main() {
    let path = "/sys/class/thermal/thermal_zone0/temp";
    
    let content = fs::read_to_string(path)
        .expect("Erreur de lecture");
    
    let temp_raw: f32 = content.trim().parse()
        .expect("Pas un nombre");
    
    let data = TempData {
        sensor_name: String::from("CPU_Thermal_Zone_0"),
        value_celsius: temp_raw / 1000.0,
    };

    let json_output = serde_json::to_string_pretty(&data)
        .expect("Erreur de sérialisation");

    println!("{}", json_output);
}
