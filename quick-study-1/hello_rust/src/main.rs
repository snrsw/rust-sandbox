fn main() {
    let spaces = "    ";
    println!("spaces: {}", spaces);
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    let (x, y): (i32, i64) = (5, 6);
    println!("x: {}, y: {}", x, y);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr: {:?}", arr);
    println!("arr[0]: {:?}", arr[0]);

    let slice: &[i32] = &arr;
    println!("slice: {:?}", slice);
}
