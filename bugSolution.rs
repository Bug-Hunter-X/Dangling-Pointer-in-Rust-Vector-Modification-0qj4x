fn main() {
    let mut v = vec![1, 2, 3];
    // Avoid using raw pointers directly.
    // Instead, use iterators for safe access and modification.
    for i in 0..v.len() {
        v[i] = v[i] * 2; // Safe modification using indexing
    }
    println!("Modified vector: {:?}", v);
} 