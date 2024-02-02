use once_cell::sync::Lazy;
use snap7_sys::LibSnap7;

pub static S7LIB: Lazy<LibSnap7> = Lazy::new(|| {
    println!("initializing LibSnap7 lib...");
    unsafe { LibSnap7::new("/usr/lib/libsnap7.so") }.unwrap()
});

