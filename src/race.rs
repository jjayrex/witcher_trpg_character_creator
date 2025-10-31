use bitflags::bitflags;
use crate::locale::Locale;

pub enum Standings {
    Equal,
    Tolerated,
    ToleratedFeared,
    Feared,
    Hated,
    HatedFeared,
}

pub struct SocialStanding {
    north: Standings,
    nilfgaard: Standings,
    skellige: Standings,
    dol_blathanna: Standings,
    mahakam: Standings,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Perks: u32 {
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
        const PLEASANT_DEMEANOR = 1 << 13;
        const EYE_FOR_DETAIL = 1 << 14;
        const SCENT_TRACKING = 1 << 15;
        const SMALL_STATURE = 1 << 16;
        const CALM_HEARTED = 1 << 17;
        const CLAWS_FANGS = 1 << 18;
        const SCALED_HIDE = 1 << 19;
        const REPTILIAN_PHYSIOLOGY = 1 << 20;
        const LIONHEARTED = 1 << 21;
        const STRANGE_PHYSIOLOGY = 1 << 22;
        const RAZOR_TEETH = 1 << 23;
        const POOR_EYESIGHT = 1 << 24;
        const NIMBLE = 1 << 25;
        const FARMHAND = 1 << 26;
        const MAGIC_RESISTANT = 1 << 27;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Race {
    Human,
    Elf,
    Dwarf,
    Witcher,
    Halfling,
    Gnome,
    Vran,
    Werebbub,
}

pub const HUMAN_TRAITS: &[Perks] = &[Perks::TRUSTWORTHY, Perks::INGENUITY, Perks::BLINDLY_STUBBORN];
pub const ELF_TRAITS: &[Perks] = &[Perks::ARTISTIC, Perks::MARKSMAN, Perks::NATURAL_ATTUNEMENT];
pub const DWARF_TRAITS: &[Perks] = &[Perks::TOUGH, Perks::STRONG, Perks::CRAFTERS_EYE];
pub const WITCHER_TRAITS: &[Perks] = &[Perks::ENHANCED_SENSES, Perks::RESILIENT_MUTATION, Perks::DULLED_EMOTIONS, Perks::LIGHTNING_REFLEXES];
pub const HALFLING_TRAITS: &[Perks] = &[Perks::NIMBLE, Perks::FARMHAND, Perks::MAGIC_RESISTANT];
pub const GNOME_TRAITS: &[Perks] = &[Perks::PLEASANT_DEMEANOR, Perks::EYE_FOR_DETAIL, Perks::SCENT_TRACKING, Perks::SMALL_STATURE];
pub const VRAN_TRAITS: &[Perks] = &[Perks::CALM_HEARTED, Perks::CLAWS_FANGS, Perks::SCALED_HIDE, Perks::REPTILIAN_PHYSIOLOGY];
pub const WEREBBUB_TRAITS: &[Perks] = &[Perks::LIONHEARTED, Perks::STRANGE_PHYSIOLOGY, Perks::RAZOR_TEETH, Perks::POOR_EYESIGHT];

impl Race {
    pub const ALL: [Race; 8] = [
        Race::Human,
        Race::Elf,
        Race::Dwarf,
        Race::Witcher,
        Race::Halfling,
        Race::Gnome,
        Race::Vran,
        Race::Werebbub,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Race::Human => "Human",
            Race::Elf => "Elf",
            Race::Dwarf => "Dwarf",
            Race::Witcher => "Witcher",
            Race::Halfling => "Halfling",
            Race::Gnome => "Gnome",
            Race::Vran => "Vran",
            Race::Werebbub => "Werebbub",
        }
    }

    pub const fn social_standing(self) -> &'static SocialStanding {
        match self {
            Race::Human => &SocialStanding {
                north: Standings::Equal,
                nilfgaard: Standings::Equal,
                skellige: Standings::Equal,
                dol_blathanna: Standings::Hated,
                mahakam: Standings::Tolerated,
            },
            Race::Elf => &SocialStanding {
                north: Standings::Hated,
                nilfgaard: Standings::Equal,
                skellige: Standings::Equal,
                dol_blathanna: Standings::Equal,
                mahakam: Standings::Equal,
            },
            Race::Dwarf => &SocialStanding {
                north: Standings::Tolerated,
                nilfgaard: Standings::Equal,
                skellige: Standings::Equal,
                dol_blathanna: Standings::Equal,
                mahakam: Standings::Equal,
            },
            Race::Witcher => &SocialStanding {
                north: Standings::HatedFeared,
                nilfgaard: Standings::HatedFeared,
                skellige: Standings::Tolerated,
                dol_blathanna: Standings::Tolerated,
                mahakam: Standings::Tolerated,
            },
            Race::Halfling => &SocialStanding {
                north: Standings::Tolerated,
                nilfgaard: Standings::Equal,
                skellige: Standings::Equal,
                dol_blathanna: Standings::Equal,
                mahakam: Standings::Equal,
            },
            Race::Gnome => &SocialStanding {
                north: Standings::Tolerated,
                nilfgaard: Standings::Equal,
                skellige: Standings::Equal,
                dol_blathanna: Standings::Equal,
                mahakam: Standings::Equal,
            },
            Race::Vran => &SocialStanding {
                north: Standings::HatedFeared,
                nilfgaard: Standings::ToleratedFeared,
                skellige: Standings::HatedFeared,
                dol_blathanna: Standings::Hated,
                mahakam: Standings::Tolerated,
            },
            Race::Werebbub => &SocialStanding {
                north: Standings::Tolerated,
                nilfgaard: Standings::Tolerated,
                skellige: Standings::Tolerated,
                dol_blathanna: Standings::Tolerated,
                mahakam: Standings::Equal,
            },
        }
    }

    pub const fn perks(self) -> &'static [Perks] {
        match self {
            Race::Human => HUMAN_TRAITS,
            Race::Elf => ELF_TRAITS,
            Race::Dwarf => DWARF_TRAITS,
            Race::Witcher => WITCHER_TRAITS,
            Race::Halfling => HALFLING_TRAITS,
            Race::Gnome => GNOME_TRAITS,
            Race::Vran => VRAN_TRAITS,
            Race::Werebbub => WEREBBUB_TRAITS,
        }
    }
}

pub fn read_perks(p: &Perks, l: &Locale) -> (Option<String>, Option<String>){
    match p {
        &Perks::ENHANCED_SENSES => (l.msg("perk-enhanced-senses-name"), l.msg("perk-enhanced-senses-desc")),
        &Perks::RESILIENT_MUTATION => (l.msg("perk-resilient-mutation-name"), l.msg("perk-resilient-mutation-desc")),
        &Perks::DULLED_EMOTIONS  => (l.msg("perk-dulled-emotions-name"), l.msg("perk-dulled-emotions-desc")),
        &Perks::LIGHTNING_REFLEXES  => (l.msg("perk-lightning-reflexes-name"), l.msg("perk-lightning-reflexes-desc")),
        &Perks::ARTISTIC  => (l.msg("perk-artistic-name"), l.msg("perk-artistic-desc")),
        &Perks::MARKSMAN  => (l.msg("perk-marksman-name"), l.msg("perk-marksman-desc")),
        &Perks::NATURAL_ATTUNEMENT  => (l.msg("perk-natural-attunement-name"), l.msg("perk-natural-attunement-desc")),
        &Perks::TOUGH  => (l.msg("perk-tough-name"), l.msg("perk-tough-desc")),
        &Perks::STRONG  => (l.msg("perk-strong-name"), l.msg("perk-strong-desc")),
        &Perks::CRAFTERS_EYE  => (l.msg("perk-crafters-eye-name"), l.msg("perk-crafters-eye-desc")),
        &Perks::TRUSTWORTHY  => (l.msg("perk-trustworthy-name"), l.msg("perk-trustworthy-desc")),
        &Perks::INGENUITY  => (l.msg("perk-ingenuity-name"), l.msg("perk-ingenuity-desc")),
        &Perks::BLINDLY_STUBBORN  => (l.msg("perk-blindly-stubborn-name"), l.msg("perk-blindly-stubborn-desc")),
        &Perks::PLEASANT_DEMEANOR  => (l.msg("perk-pleasant-demeanor-name"), l.msg("perk-pleasant-demeanor-desc")),
        &Perks::EYE_FOR_DETAIL  => (l.msg("perk-eye-for-detail-name"), l.msg("perk-eye-for-detail-desc")),
        &Perks::SCENT_TRACKING  => (l.msg("perk-scent-tracking-name"), l.msg("perk-scent-tracking-desc")),
        &Perks::SMALL_STATURE  => (l.msg("perk-small-stature-name"), l.msg("perk-small-stature-desc")),
        &Perks::CALM_HEARTED  => (l.msg("perk-calm-hearted-name"), l.msg("perk-calm-hearted-desc")),
        &Perks::CLAWS_FANGS  => (l.msg("perk-claws-fangs-name"), l.msg("perk-claws-fangs-desc")),
        &Perks::SCALED_HIDE  => (l.msg("perk-scaled-hide-name"), l.msg("perk-scaled-hide-desc")),
        &Perks::REPTILIAN_PHYSIOLOGY  => (l.msg("perk-reptilian-physiology-name"), l.msg("perk-reptilian-physiology-desc")),
        &Perks::LIONHEARTED  => (l.msg("perk-lionhearted-name"), l.msg("perk-lionhearted-desc")),
        &Perks::STRANGE_PHYSIOLOGY  => (l.msg("perk-strange-physiology-name"), l.msg("perk-strange-physiology-desc")),
        &Perks::RAZOR_TEETH  => (l.msg("perk-razor-teeth-name"), l.msg("perk-razor-teeth-desc")),
        &Perks::POOR_EYESIGHT  => (l.msg("perk-poor-eyesight-name"), l.msg("perk-poor-eyesight-desc")),
        &Perks::NIMBLE  => (l.msg("perk-nimble-name"), l.msg("perk-nimble-desc")),
        &Perks::FARMHAND  => (l.msg("perk-farmhand-name"), l.msg("perk-farmhand-desc")),
        &Perks::MAGIC_RESISTANT  => (l.msg("perk-magic-resistant-name"), l.msg("perk-magic-resistant-desc")),
        _ => (Some("?".to_string()), Some("?".to_string())),
    }
}