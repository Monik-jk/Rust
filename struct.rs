struct Rectangle{
    width: i32,
    height: i32,
}
impl Rectangle{
  fn new(width:i32,height:i32) -> Self{
      Self{width,height}
  }
    fn area(&self) ->i32{
        self.width * self.height
    }
}
fn main() {
    let rect =Rectangle::new(2,5);
    println!("{}", rect.area());
}
