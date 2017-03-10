

pub struct Human {
    name: String,
}

impl Human {
    pub fn new(name: &str) -> Human {
        Human { name: name.to_string() }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn apprivoiser(&mut self) {
        println!("アプリポワゼ！ 颯爽登場！銀河美少年!タウバーン！");
        self.name = "銀河美少年".to_string();
    }

    pub fn input_action(&mut self, args_line: String) {
        println!("{}", args_line);
    }
}