use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};
use std::process::Command;

fn limpiar_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("fallo al limpiar la terminal");
    } else {
        Command::new("clear")
            .status()
            .expect("fallo al limpiar la terminal");
    }
}

fn inicio() {
    let mut iniciar_juego = String::new();

    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("[================================================]");
    println!("                                                  ");
    println!("               ADIVINA EL NÚMERO                  ");
    println!("                                                  ");
    println!("                     [jugar]                      ");
    println!("                                                  ");
    println!("[================================================]");
    

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut iniciar_juego).expect("Error");
}

fn nivel_grafico() {
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("[================================================]");
    println!("                                                  ");
    println!("                      NIVEL                       ");
    println!("                                                  ");
    println!("                     1 [FACIL]                    ");
    println!("                     2 [INTERMEDIO]               ");
    println!("                     3 [DIFICIL]                  ");
    println!("                                                  ");
    println!("                                                  ");
    println!("[================================================]");
    print!("                          ");
    io::stdout().flush().unwrap();
}

fn grafico_num_mayor() {
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("[================================================]");
    println!("                                                  ");
    println!("               ADIVINA EL NÚMERO                  ");
    println!("                                                  ");
    println!("      [INGRESAR UN NÚMERO MAS GRANDE [++]         ");
    println!("                                                  ");
    println!("[================================================]");
}

fn grafico_num_menor() {
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("[================================================]");
    println!("                                                  ");
    println!("               ADIVINA EL NÚMERO                  ");
    println!("                                                  ");
    println!("     [INGRESAR UN NÚMERO MAS PEQUEÑO [--]         ");
    println!("                                                  ");
    println!("[================================================]");
}

fn grafico_ganaste() {
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("[================================================]");
    println!("    *      *    *       *      *       *    *     ");
    println!("   *             *   *    *        *      *   *   ");
    println!("     *    *    *      GANASTE     *     *    *    ");
    println!("  *    *      *    *   *      *   *      *    *   ");
    println!("  *  *     *   *    *       *        *      *     ");
    println!("[================================================]");
}

fn mecanica_nivel(nivel: i32) -> i32 {
    let mut vidas = 5;
    if nivel == 1 {
        vidas = 10;
    } else if nivel == 2 {
        vidas = 6;
    } else if nivel == 3 {
        vidas = 4;
    } else {
        println!("Error de numero ");
    }

    vidas
}

fn numero_aleatorio(numero_secreto: u32) {
    let mut adivina = String::new();

    io::stdin()
        .read_line(&mut adivina)
        .expect("Failed to read line");

    limpiar_terminal();

    let adivina: u32 = adivina.trim().parse().expect("Please type a number!");

    match adivina.cmp(&numero_secreto) {
        Ordering::Less => grafico_num_mayor(),
        Ordering::Greater => grafico_num_menor(),
        Ordering::Equal => grafico_ganaste(),
    }
}

fn main() {
    inicio();
    limpiar_terminal();
    nivel_grafico();

    let mut nivel = String::new();

    io::stdin().read_line(&mut nivel).expect("Error");

    let nivel: i32 = nivel
        .trim()
        .parse()
        .expect("Por favor, ingresa un número válido");

    let mecanica = mecanica_nivel(nivel);

    println!("{}", mecanica);

    limpiar_terminal();
    let numero_secreto = rand::thread_rng().gen_range(1..=100);
    loop {
        numero_aleatorio(numero_secreto);
    }
}