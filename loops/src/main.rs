fn main() {
    println!("Hello, world!");

    // loop label
    let mut i = 0;
    'first_loops: loop{
        let mut j = 0;
        println!(" first loops ");
        'second_loops: loop{
            println!(" second loops ");
            if j>3 {
                break 'second_loops
            }
            if i>3 {
                break 'first_loops // loop label
            }
            j = j+1;
        }
        i = i+1;
    }

    // イテラブルの扱いがpythonみたい。
    let a : [i32; 20] = [5;20];

    for i in a {
        println!("{i}");
    }
}
