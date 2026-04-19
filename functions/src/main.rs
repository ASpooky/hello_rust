fn main() {

    let y = {
        let x = five();
        x + 1 // セミコロンを付けたら式が文になる。セミコロンがついているものは文。
    };
    println!("The value of y is: {y}");
    another_function(32,'h');
}

fn another_function(x:i32,unit_label:char){
    println!("The value of x is: {x}{unit_label}");
}

fn five() -> i32 {
    5 // ここもセミコロンはいらない。
}


