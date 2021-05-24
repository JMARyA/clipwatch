mod config;
use clipboard_macos::*;
use std::sync::{Arc, Mutex};
use std::{process, thread, time};

fn main() {
    let conf = config::parse_config();

    let c = Clipboard::new().unwrap();
    let mut cstr = String::new();
    let ten_millis = time::Duration::from_millis(10);
    let stream: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let cp_stream = stream.clone();

    let summary = conf.summary.clone();
    ctrlc::set_handler(move || {
        if summary {
            let mut fstr = String::new();
            let v = cp_stream.lock().unwrap();
            for i in 0..v.len() {
                fstr.push_str(&*format!("{}\n", v[i]));
            }
            Clipboard::new().unwrap().write(fstr);
        }
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    if conf.ignore_first {
        cstr = c.read().unwrap();
    }

    loop {
        let nstr = c.read();
        if nstr.is_err() {
            continue;
        }
        let nstr = nstr.unwrap();
        if nstr != cstr {
            stream.lock().unwrap().push(nstr.clone());
            println!("{}", nstr);
            cstr = nstr;
        }
        thread::sleep(ten_millis);
    }
}
