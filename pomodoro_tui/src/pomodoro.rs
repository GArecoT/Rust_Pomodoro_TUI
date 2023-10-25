use std::thread::sleep;
use std::io;
use std::time::Duration;
use std::time::Instant;
pub fn timer(){
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Não foi possível ler o número");
    let num:u64 = input_line.trim().parse().expect("não é um número"); 
    let now = Instant::now();
    let mut temp = now.elapsed().as_secs();
    while now.elapsed().as_secs() <=num {
        if temp != now.elapsed().as_secs(){
            let output_min = (temp / 60).to_string();
            let output_sec  = (temp % 60).to_string();

            if temp % 60 < 10{
                println!("{output_min}:0{output_sec}"); 
            }else{
                println!("{output_min}:{output_sec}"); 
            }
            temp = now.elapsed().as_secs();
            sleep(Duration::new(0, 900))
        }
    }
}
