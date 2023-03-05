use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("引数の個数が正しくありません");
    }
    
      println!(".intel_syntax noprefix");
      println!(".globl main");
      println!("main:");
      println!("  mov rax, {}", args[1].to_string());
      println!("  ret");
}
