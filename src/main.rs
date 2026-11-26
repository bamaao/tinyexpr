use std::io::{self, BufRead, Write};

fn main() {
    println!("tinyexpr REPL (输入表达式计算，输入 exit 退出)");
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                eprintln!("读取输入失败：{err}");
                break;
            }
        };

        let input = line.trim();
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }
        if input.is_empty() {
            continue;
        }

        match tinyexpr::eval(input) {
            Ok(value) => println!("{value}"),
            Err(err) => eprintln!("{err}"),
        }
        let _ = stdout.flush();
    }
}
