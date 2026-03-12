/*
 -key concepts
-declared using const.
-always immutable (cant use 'mut')
-type annotation is required
-value must be known at compile time.
-naming convestions: UPPERCASE_WITH_UNDERSCORES.
-live for entire program duration.
-inlined where used(replaced with the value at compile time).
 */
const MAX_P:i32=100_000;

fn main() {
println!("a constant, maxmum marks, has a value : {}",MAX_P);
}
