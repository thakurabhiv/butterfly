use chrono::{ Utc, Datelike, DateTime };

pub fn get_current_fy() -> String {
    get_fy_string(Utc::now())
}

fn get_fy_string(date: DateTime<Utc>) -> String {
    let year = date.year();
    let month = date.month();

    if month <= 3 {
        format!("{}-{:02}", year-1, year % 100)
    } else {
        format!("{}-{:02}", year, (year+1) % 100)
    }
}