use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("引数の個数が正しくありません");
    }
    
    let p: &str = &args[1];
    let mut p1: Vec<char> = vec![];

    for i in p.chars() {
        p1.push(i);
    }
    

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", p1[0]);

    for i in 0..p1.len() {
      let operator = p1[i];
      if i==0 {continue};
      if operator == '+'{
        println!("  add rax, {}",p1[i+1]);
        continue;
      }
      else if operator == '-'{
        println!("  sub rax, {}",p1[i+1]);
        continue;
      }
    }
    // for i in p2 {
    //   if i=="+"{
    //     print!("  add rax");
    //     continue;
    //   }
    //   else if i=="-"{
    //     print!("  sub rax");
    //     continue;
    //   }
    //   else{
    //     println!(", {}",i);
    //   }
    // }
    println!("  ret");

}

// fn split_digit(s: &str) -> (&str, &str) {
//   let first_non_num = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
//   s.split_at(first_non_num)
// }
