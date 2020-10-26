pub fn trim_str(string: String) -> String {
    string.trim().to_string()
}

pub fn str2int(string: String) -> i32 {
    string.parse::<i32>().unwrap()
}

pub fn str2float(string: String) -> f32 {
    string.parse::<f32>().unwrap()
}

pub fn str2big_float(string: String) -> f64 {
    string.parse::<f64>().unwrap()
}

#[cfg(test)]
mod test {

    #[test]
    fn trim_str() {
        assert_eq!(super::trim_str(String::from("   Mateus ")), "Mateus");
    }

    #[test]
    fn str2int() {
        let number = String::from("42");
        assert_eq!(super::str2int(number), 42)
    }

    #[test]
    fn str2float() {
        let number = String::from("42");
        assert_eq!(super::str2float(number), 42.0);
    }

    #[test]
    fn str2big_float() {
        let number = String::from("42.4163546324238435174135434723657135457343654163156");
        assert_eq!(
            super::str2big_float(number),
            42.4163546324238435174135434723657135457343654163156
        );
    }
}
