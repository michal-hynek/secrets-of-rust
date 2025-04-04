use hello::print;

fn main() {
    print(std::io::stdout()).unwrap();
    println!("{}", hello::world());
}
