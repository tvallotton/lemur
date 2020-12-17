use rand;

struct Scope {
    names: Vec<String>,
}

impl Scope {
    pub fn new() {
        Scope { names: vec![] }
    }

    pub fn available(&self, to_check: &String) -> bool {
        for name in self.names.iter() {
            if to_check == name {
                return false;
            }
        }
        return true;
    }

    pub fn get_name(&self, priority: String) -> String {
        if self.names.contains(&priority) {
            self.names.push(priority);
            return priority;
        }
        let integer = 0;
        loop {
            let out = format!("{}{}", priority, String::from(integer));
            if !self.names.contains(&out) {
                self.names.push(out);
                return out;
            }
            integer += 1;
        }
    }
}
