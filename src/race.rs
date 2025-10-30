use bitflags::bitflags;

enum Races {
    Human,
    Dwarf,
    Elf,
    Witcher,
}

enum Standings {
    Equal,
    Tolerated,
    Feared,
    Hated,
}

struct SocialStanding {
    north: Standings,
    nilfgaard: Standings,
    skellige: Standings,
    dol_blathanna: Standings,
    mahakam: Standings,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Perks: u16 {
        const ENHANCED_SENSES = 1 << 0;
        const RESILIENT_MUTATION = 1 << 1;
        const DULLED_EMOTIONS = 1 << 2;
        const LIGHTNING_REFLEXES = 1 << 3;
        const ARTISTIC = 1 << 4;
        const MARKSMAN = 1 << 5;
        const NATURAL_ATTUNEMENT = 1 << 6;
        const TOUGH = 1 << 7;
        const STRONG = 1 << 8;
        const CRAFTERS_EYE = 1 << 9;
        const TRUSTWORTHY = 1 << 10;
        const INGENUITY = 1 << 11;
        const BLINDLY_STUBBORN = 1 << 12;
    }
}

struct Race {
    name: Races,
    social: SocialStanding,
    perks: Perks,
}
