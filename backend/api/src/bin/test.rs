trait PrintStr {
  fn print_str(&self);
}

impl PrintStr for String {
  fn print_str(&self) {
      println!("{:?}", self);
  }
}
fn main() {
  let my_string = "My string".to_string();
  my_string.print_str();
}