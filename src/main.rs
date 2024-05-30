use std::io;

fn main() {
    println!("Entrez le premier nombre : ");
    let mut nombre1 = String::new();
    io::stdin().read_line(&mut nombre1).expect("Erreur de lecture de ligne");
    let nombre1: u32 = nombre1.trim().parse().expect("Ceci n'est pas un nombre");

    println!("Entrez le deuxième nombre : ");
    let mut nombre2 = String::new();
    io::stdin().read_line(&mut nombre2).expect("Erreur de lecture de ligne");
    let nombre2: u32 = nombre2.trim().parse().expect("Ceci n'est pas un nombre");

    let somme = nombre1 + nombre2;
    println!("La somme est : {}", somme);
}