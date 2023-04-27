/* Get current date-time as a String */

pub fn get_now() -> String {

    let now = chrono::Local::now();
    let date: String = now.date_naive().to_string();
    date
}