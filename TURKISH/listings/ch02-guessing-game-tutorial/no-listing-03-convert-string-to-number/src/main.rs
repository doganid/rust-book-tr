use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Tuttuğum sayıyı tahmin edin!");

    let gizli_sayı = rand::thread_rng().gen_range(1..101);

    println!("Gizli sayı: {}", gizli_sayı);

    println!("Tahmininizi girin.");

    // ANCHOR: here
    // --snip--

    let mut tahmin = String::new();

    io::stdin()
        .read_line(&mut tahmin)
        .expect("Veri okuma hatası!");

    let tahmin: u32 = tahmin.trim().parse().expect("Lütfen bir sayı türü girin!");

    println!("Tahmininiz: {}", tahmin);

    match tahmin.cmp(&gizli_sayı) {
        Ordering::Less => println!("Sayınız küçük!"),
        Ordering::Greater => println!("Sayınız büyük!"),
        Ordering::Equal => println!("Kazandınız"),
    }
    // ANCHOR_END: here
}
