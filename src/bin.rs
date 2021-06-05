use clap::clap_app;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (about: "Convert B58 address to animal name")
        (@arg ADDR: +required "The address to convert")
    )
    .get_matches();

    if let Some(addr) = matches.value_of("ADDR") {
        let animal_name = addr
            .parse::<angry_purple_tiger::AnimalName>()
            .expect("animal name");
        println!("{}", animal_name);
    }
}
