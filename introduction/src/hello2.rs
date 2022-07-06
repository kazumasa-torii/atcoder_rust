pub fn main() {
    println!("hello world");
    let southen_germany = "Grii Gott!";
    let japan = "ハロー";
    let regions = [southen_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
