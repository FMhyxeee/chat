fn main() {
    let str = "DATABASE_URL=postgres://postgres:postgres@localhost:5432/chat";
    let pos = str.rfind('/').unwrap();
    println!("{}", &str[..pos]);
}
