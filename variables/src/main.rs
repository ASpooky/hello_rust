use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}",x);

    x = 6;
    println!("The value of x is: {}",x);

    let y = 6;
    let y = y+1;
    println!("The value of y is: {}",y);

    let y = y*2;
    println!("The value of y is: {}",y);

    let spaces = "    ";
    let spaces = spaces.len(); // mutだと型変換までは許可されてないのでエラー
    println!("spaces is {}",spaces);

    // 配列
    // 固定長
    let a : [i8;5] = [1,1,1,1,1];
    println!("{}",a[1]);

    // 可変長
    let a = [1;8];
    println!("{}",a[0]);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
