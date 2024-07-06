// 简单的Hangman程序
// 用户有五次错误猜测的机会
// 从words.txt中随机选择单词
// 灵感来源于：https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// 该作业将介绍一些Rust的基本语法：
// - 变量声明
// - 字符串操作
// - 条件语句
// - 循环
// - 向量（Vectors）
// - 文件操作
// - 用户输入
// 我们尝试隐藏Rust的一些特性，因为我们将在接下来的讲座中更深入地讨论这些细节。

extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("无法读取文件.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}



// 读取用户输入
fn input() -> char {
    loop {
        print!("请输入一个字母: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取输入失败.");
        let guess_char = match guess.chars().next() {
            Some(c) => c,
            None => {
                println!("请输入一个有效的字母！");
                continue;
            }
        };
        return guess_char;
    }
}




fn main() {
    let secret_word = pick_a_random_word();
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    let word_len = secret_word_chars.len();
    let mut chars_left: Vec<bool> = vec![false; word_len];
    
    println!("欢迎来到Hangman游戏！你有{}次错误猜测的机会。", NUM_INCORRECT_GUESSES);

    let mut incorrect_guesses = 0;
    loop {
        // 显示当前的猜测状态
        for i in 0..word_len {
            if chars_left[i] {
                print!("{}", secret_word_chars[i]);
            } else {
                print!("_");
            }
        }
        println!();

        let guess_char: char = input();

        // 检查猜测
        let mut correct_guess = false;
        for i in 0..word_len {
            if guess_char == secret_word_chars[i] && !chars_left[i] {
                chars_left[i] = true;
                correct_guess = true;
            }
        }

        if !correct_guess {
            incorrect_guesses += 1;
            println!("错误！你还有{}次机会。", NUM_INCORRECT_GUESSES - incorrect_guesses);
        }

        // 检查是否游戏结束
        if incorrect_guesses >= NUM_INCORRECT_GUESSES {
            println!("你输了！正确的单词是：{}", secret_word);
            break;
        }

        if chars_left.iter().all(|&c| c) {
            println!("恭喜你！你猜对了单词：{}", secret_word);
            break;
        }
    }
}