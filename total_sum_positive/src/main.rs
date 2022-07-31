fn main() {
    let slice_numbers: [i32; 4] = [1, -4, 7, 12];
    let sum: i32 = slice_numbers.iter().filter(|x| x.is_positive()).sum();
    println!("The total sum is: {}", sum);
}
