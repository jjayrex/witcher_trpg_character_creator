mod locale;
mod race;
mod lifepath;
mod style;
mod profession;
mod spells;
mod statistics;

use locale::Locale;

fn main() {
    println!("Witcher Pen & Paper RPG - Character Creator");

    let locale = Locale::new(locale::build_bundle());

    for perk in race::Race::Human.perks() {
        let perk_list = race::read_perks(perk, &locale);

        println!("Name: {}, Description: {}", perk_list.0.unwrap(), perk_list.1.unwrap());
    }
}
