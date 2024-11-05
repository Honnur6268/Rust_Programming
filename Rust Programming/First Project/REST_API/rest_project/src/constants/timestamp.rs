use chrono::Local;

pub fn current_timestamp() -> String {
    let now = Local::now();
    now.format("%d-%b-%Y %H:%M:%S %A").to_string()
}