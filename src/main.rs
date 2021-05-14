use clipboard_macos::*;
use std::{thread, time};

fn main() {
    let c = Clipboard::new().unwrap();
    let mut cstr = String::new();
    let ten_millis = time::Duration::from_millis(10);

    loop {
        let nstr = c.read();
        if nstr.is_err() {
            continue;
        }
        let nstr = nstr.unwrap();
        if nstr != cstr {
            println!("{}", nstr);
            cstr = nstr;
        }
        thread::sleep(ten_millis);
    }
}
