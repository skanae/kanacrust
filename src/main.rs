use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("引数の個数が正しくありません");
    }
    
    let p: &str = &args[1];
    let mut split = p.split(' ');
    let mut p1 = vec![];
    for i in split {
        p1.push(i);
    }
    
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    

    for i in 0..p1.len() {
      let operator = p1[i];
      if i==0 {
        println!("  mov rax, {}", p1[0]);
        continue;
      };
      if operator == "+"{
        println!("  add rax, {}",p1[i+1]);
        continue;
      }
      else if operator == "-"{
        println!("  sub rax, {}",p1[i+1]);
        continue;
      }
    }

    println!("  ret");

}

// fn split_digit(s: &str) -> (&str, &str) {
//   let first_non_num = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
//   s.split_at(first_non_num)
// }
