use std::env;

use once_cell::sync::Lazy;
use snap7_sys::LibSnap7;

pub static S7LIB: Lazy<LibSnap7> = Lazy::new(|| {
    println!("initializing LibSnap7 lib...");
    let paths = [
        format!("{}/libsnap7.so", env::current_dir().unwrap().display()),
        format!("/usr/lib/libsnap7.so"),
    ];
    for path in paths {
        println!("check '{}'...", path);
        match unsafe { LibSnap7::new(&path) } {
            Ok(lib) => {
                println!("check '{}' - ok", path);
                println!("initializing LibSnap7 lib - ok");
                return lib;
            },
            Err(_) => {
                println!("check '{}' - not found", path);
            },
        }
    }
    panic!("libsnap7.so - not found")
});
