mod locale;
mod race;

use locale::Locale;
use race::Perks;

fn main() {
    println!("Witcher Pen & Paper RPG - Character Creator");

    let locale = Locale::new(locale::build_bundle());
}

fn read_perks(p: Perks, l: &Locale) -> (String, String) {
    match p {
        Perks::ENHANCED_SENSES => (l.msg("perk-enhanced-senses-name"), l.msg("perk-enhanced-senses-desc")),
        Perks::RESILIENT_MUTATION => (l.msg("perk-resilient-mutation-name"), l.msg("perk-resilient-mutation-desc")),
        Perks::DULLED_EMOTIONS  => (l.msg("perk-dulled-emotions-name"), l.msg("perk-dulled-emotions-desc")),
        Perks::LIGHTNING_REFLEXES  => (l.msg("perk-lightning-reflexes-name"), l.msg("perk-lightning-reflexes-desc")),
        Perks::ARTISTIC  => (l.msg("perk-artistic-name"), l.msg("perk-artistic-desc")),
        Perks::MARKSMAN  => (l.msg("perk-marksman-name"), l.msg("perk-marksman-desc")),
        Perks::NATURAL_ATTUNEMENT  => (l.msg("perk-natural-attunement-name"), l.msg("perk-natural-attunement-desc")),
        Perks::TOUGH  => (l.msg("perk-tough-name"), l.msg("perk-tough-desc")),
        Perks::STRONG  => (l.msg("perk-strong-name"), l.msg("perk-strong-desc")),
        Perks::CRAFTERS_EYE  => (l.msg("perk-crafters-eye-name"), l.msg("perk-crafters-eye-desc")),
        Perks::TRUSTWORTHY  => (l.msg("perk-trustworthy-name"), l.msg("perk-trustworthy-desc")),
        Perks::INGENUITY  => (l.msg("perk-ingenuity-name"), l.msg("perk-ingenuity-desc")),
        Perks::BLINDLY_STUBBORN  => (l.msg("perk-blindly-stubborn-name"), l.msg("perk-blindly-stubborn-desc")),
        _ => ("?".to_string(), "?".to_string()),
    }
}