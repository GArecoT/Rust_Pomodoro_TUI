use notify_rust::Notification;
use std::io::{self, Write};

mod pomodoro;
fn main() {
    print!("Insira o tempo de estudo em minutos: ");
    io::stdout().flush().unwrap();
    let mut study_time = String::new();
    io::stdin()
        .read_line(&mut study_time)
        .expect("Não foi possível ler o número");

    print!("Insira o tempo de descanso em minutos: ");
    io::stdout().flush().unwrap();
    let mut stop_time = String::new();
    io::stdin()
        .read_line(&mut stop_time)
        .expect("Não foi possível ler o número");

    print!("Insira o número de loops: ");
    io::stdout().flush().unwrap();
    let mut loops = String::new();
    io::stdin()
        .read_line(&mut loops)
        .expect("Não foi possível ler o número");

    let num: u64 = loops.trim().parse().expect("Não é um número");

    for n in 1..num + 1 {
        print!("Trabalho {n}\n");
        let _ = Notification::new()
            .summary("Trabalho!")
            .body("Hora de trabalhar")
            .show();
        io::stdout().flush().unwrap();
        pomodoro::timer(study_time.clone());
        print!("Intervalo {n}\n");

        let _ = Notification::new()
            .summary("Intervalo!")
            .body("Hora de descansar")
            .show();
        io::stdout().flush().unwrap();
        pomodoro::timer(stop_time.clone());
    }
}
