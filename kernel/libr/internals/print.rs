pub trait Print {
    fn fmt(&self) -> &'static str;
    fn print(&self) {
        log!("{}", self.fmt());
    }
    fn println(&self) {
        log!("{}\n", self.fmt());
    }
}