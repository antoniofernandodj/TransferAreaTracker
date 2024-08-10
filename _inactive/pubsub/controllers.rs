use super::models::Event;

pub fn numero_grande_callback(event: &Event) {
    println!("Numero muito grande...");
    println!("Evento: {event:?}")
}

pub fn numero_pequeno_callback(event: &Event) {
    println!("Numero muito pequeno...");
    println!("Evento: {event:?}")
}
