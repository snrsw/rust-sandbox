fn foo() -> String {
    let s = String::from("Hello");
    s
}

fn append(src: &Vec<i32>, dst: &mut Vec<i32>) {
    for &x in src {
        dst.push(x);
    }
}

fn main() {
    let x: String;
    x = String::from("Hello, world!");
    println!("{}", x);

    let y = x;
    println!("{}", y);

    let z = y.clone();
    println!("{}", z);
    println!("{}", y);

    let hoge = foo();
    println!("{}", hoge);

    let mut v = vec![1, 2, 3];
    append(&v.clone(), &mut v);
    println!("{:?}", v);

    let mut x = 5;
    println!("{}", x);
    let y = &mut x;
    *y += 1;
    println!("{}", y);
    println!("{}", x);
}
