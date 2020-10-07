pub fn trim_str(string: String) -> String {
    string.trim().to_string()
}

pub fn str2int(string: String) -> i32 {
    string.parse::<i32>().unwrap()
}

pub fn str2float(string: String) -> f32 {
    string.parse::<f32>().unwrap()
}
