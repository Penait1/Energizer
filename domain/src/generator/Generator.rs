struct Generator {
    name: String,
    output: u64,
    running: bool
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
