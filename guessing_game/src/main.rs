use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        let result = io::stdin()
            .read_line(&mut guess);
        //.expect("Failed to read line");
        
        match result{
            Ok(_) => {
                println!("成功!");
                println!("入力された文字:{}",guess);
            }
            Err(error)=>{
                println!("読み込みエラーが発生しました。:{}",error);
            }
        };
        
        let parse_result: Result<u32,_> = guess.trim().parse();
        
        let guess_num: u32 = match parse_result{
            Ok(num)=>num,
            Err(_)=> {
                println!("数字以外が入力されました。再度入力してください。");
                continue;
            }
        };
        
        println!("You guessed: {guess_num}");
        
        match guess_num.cmp(&secret_number){
            Ordering::Less=>println!("Too small.."),
            Ordering::Greater=>println!("Too big.."),
            Ordering::Equal=>{
                println!("You win..");
                break;
            }
        }
    }
}
