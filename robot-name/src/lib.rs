use rand::Rng;

#[derive(Default)]
pub struct Robot {
    name: String,
}

fn create_name() -> String {
    let mut rng = rand::thread_rng();

    format!(
        "{}{}{:03}", 
        rng.gen_range(b'A', b'Z' + 1) as char, 
        rng.gen_range(b'A', b'Z' + 1) as char, 
        rng.gen_range(0, 1000)
    )
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: create_name()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = create_name()
    }
}
