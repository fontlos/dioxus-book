use std::env::current_dir;

fn main() {
    let book_dir = current_dir().unwrap().join("./examples/book");
    let out_dir = current_dir().unwrap().join("./examples/book");
    let out = dioxus_book::build(book_dir);
    std::fs::write(out_dir.join("book.rs"), out).unwrap();
}