pub struct Generator {
    pub name: String,
    pub output: u64,
    pub running: bool,
    pub price: u64
}

impl Generator {
    fn get_output(&self) {
        if self.running {
            self.output;
        } else {
            0;
        }
    }
}
