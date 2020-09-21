pub mod interact {
    use std::io;

    pub fn read_input() -> String {
        let mut value = String::new();

        match io::stdin().read_line(&mut value) {
            Ok(_) => return value,
            Err(_e) => String::from("ERROR"),
        }
    }
}
