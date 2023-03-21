use std::f64::consts::PI;

extern "C" {
    fn abs(input: i32) -> i32;
    fn acos(input: f64) -> f64;
}

fn main() {
    unsafe {
        // Undefined behavior if abs misbehaves.
        println!("Absolute value of -3 according to C: {}", abs(-3));
        println!("PI value from arc cosine: {} {PI}",acos(-1.0));
    }
}