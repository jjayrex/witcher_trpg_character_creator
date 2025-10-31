use crate::locale::Locale;
use crate::statistics::Skills;

pub enum DefiningSkill {
    Busking,
    PatchJob,
    PracticedParanoia,
    HealingHands,
    MagicTraining,
    ToughAsNails,
    WellTraveled,
    InitiateOfTheGods,
    WitcherTraining,
    Notoriety,
}

pub struct MagicalPerks {
    novice_spells: u8,
    invocations: u8,
    rituals: u8,
    hexes: u8,
    signs: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Profession {
    Bard,
    Craftsman,
    Criminal,
    Doctor,
    Mage,
    ManAtArms,
    Merchant,
    Priest,
    Witcher,
    Noble,
}

impl Profession {
    pub const ALL: [Profession; 10] = [
        Profession::Bard,
        Profession::Craftsman,
        Profession::Criminal,
        Profession::Doctor,
        Profession::Mage,
        Profession::ManAtArms,
        Profession::Merchant,
        Profession::Priest,
        Profession::Witcher,
        Profession::Noble,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            Profession::Bard => "Bard",
            Profession::Craftsman => "Craftsman",
            Profession::Criminal => "Criminal",
            Profession::Doctor => "Doctor",
            Profession::Mage => "Mage",
            Profession::ManAtArms => "ManAtArms",
            Profession::Merchant => "Merchant",
            Profession::Priest => "Priest",
            Profession::Witcher => "Witcher",
            Profession::Noble => "Noble",
        }
    }

    pub const fn defining_skill(self) -> &'static DefiningSkill {
        match self {
            Profession::Bard => &DefiningSkill::Busking,
            Profession::Craftsman => &DefiningSkill::PatchJob,
            Profession::Criminal => &DefiningSkill::PracticedParanoia,
            Profession::Doctor => &DefiningSkill::HealingHands,
            Profession::Mage => &DefiningSkill::MagicTraining,
            Profession::ManAtArms => &DefiningSkill::ToughAsNails,
            Profession::Merchant => &DefiningSkill::WellTraveled,
            Profession::Priest => &DefiningSkill::InitiateOfTheGods,
            Profession::Witcher => &DefiningSkill::WitcherTraining,
            Profession::Noble => &DefiningSkill::Notoriety,
        }
    }

    pub const fn vigor(self) -> u8 {
        match self {
            Profession::Bard
            | Profession::Craftsman
            | Profession::Criminal
            | Profession::Doctor
            | Profession::ManAtArms
            | Profession::Merchant
            | Profession::Noble => 0,
            Profession::Priest | Profession::Witcher => 2,
            Profession::Mage => 5,
        }
    }

    pub const fn magical_perks(self) -> &'static MagicalPerks {
        match self {
            Profession::Bard
            | Profession::Craftsman
            | Profession::Criminal
            | Profession::Doctor
            | Profession::ManAtArms
            | Profession::Merchant
            | Profession::Noble => &MagicalPerks {
                novice_spells: 0,
                invocations: 0,
                rituals: 0,
                hexes: 0,
                signs: false,
            },
            Profession::Mage => &MagicalPerks {
                novice_spells: 5,
                invocations: 0,
                rituals: 1,
                hexes: 1,
                signs: true,
            },
            Profession::Priest => &MagicalPerks {
                novice_spells: 0,
                invocations: 2,
                rituals: 2,
                hexes: 2,
                signs: true,
            },
            Profession::Witcher => &MagicalPerks {
                novice_spells: 0,
                invocations: 0,
                rituals: 0,
                hexes: 0,
                signs: true,
            },
        }
    }

    pub const fn skills(self) -> &'static [Skills; 10] {
        match self {
            Profession::Bard => &[
                Skills::Charisma,
                Skills::Deceit,
                Skills::Performance,
                Skills::Language,
                Skills::HumanPerception,
                Skills::Persuasion,
                Skills::Streetwise,
                Skills::FineArts,
                Skills::Seduction,
                Skills::SocialEtiquette,
            ],
            Profession::Craftsman => &[
                Skills::Crafting,
                Skills::Business,
                Skills::Athletics,
                Skills::Endurance,
                Skills::Physique,
                Skills::Streetwise,
                Skills::FineArts,
                Skills::Alchemy,
                Skills::Education,
                Skills::Persuasion,
            ],
            Profession::Criminal => &[
                Skills::SleightOfHand,
                Skills::PickLocks,
                Skills::Streetwise,
                Skills::Forgery,
                Skills::Deceit,
                Skills::Stealth,
                Skills::Intimidation,
                Skills::SmallBlades,
                Skills::Athletics,
                Skills::Awareness,
            ],
            Profession::Doctor => &[
                Skills::ResistCoercion,
                Skills::Charisma,
                Skills::SocialEtiquette,
                Skills::Courage,
                Skills::HumanPerception,
                Skills::WildernessSurvival,
                Skills::Business,
                Skills::Deduction,
                Skills::SmallBlades,
                Skills::Alchemy,
            ],
            Profession::Mage => &[
                Skills::HumanPerception,
                Skills::SpellCasting,
                Skills::HexWeaving,
                Skills::ResistMagic,
                Skills::StaffSpear,
                Skills::Education,
                Skills::RitualCrafting,
                Skills::SocialEtiquette,
                Skills::Seduction,
                Skills::GroomingStyle,
            ],
            Profession::ManAtArms => &[
                Skills::CombatSkill,
                Skills::CombatSkill,
                Skills::CombatSkill,
                Skills::CombatSkill,
                Skills::CombatSkill,
                Skills::WildernessSurvival,
                Skills::Courage,
                Skills::Physique,
                Skills::Intimidation,
                Skills::DodgeEscape,
            ],
            Profession::Merchant => &[
                Skills::Charisma,
                Skills::SmallBlades,
                Skills::Education,
                Skills::Language,
                Skills::Streetwise,
                Skills::Business,
                Skills::Persuasion,
                Skills::HumanPerception,
                Skills::Gambling,
                Skills::ResistCoercion,
            ],
            Profession::Priest => &[
                Skills::RitualCrafting,
                Skills::Leadership,
                Skills::Courage,
                Skills::HumanPerception,
                Skills::HexWeaving,
                Skills::FirstAid,
                Skills::Charisma,
                Skills::WildernessSurvival,
                Skills::Teaching,
                Skills::SpellCasting,
            ],
            Profession::Witcher => &[
                Skills::Awareness,
                Skills::Deduction,
                Skills::SpellCasting,
                Skills::Alchemy,
                Skills::DodgeEscape,
                Skills::WildernessSurvival,
                Skills::Swordsmanship,
                Skills::Athletics,
                Skills::Stealth,
                Skills::Riding,
            ],
            Profession::Noble => &[
                Skills::Awareness,
                Skills::CombatSkill,
                Skills::Deceit,
                Skills::Education,
                Skills::GroomingStyle,
                Skills::HumanPerception,
                Skills::Leadership,
                Skills::Persuasion,
                Skills::Riding,
                Skills::SocialEtiquette,
            ],
        }
    }
}

pub fn read_defining_skill(
    ds: &DefiningSkill,
    l: &Locale,
) -> (Option<String>, String, Option<String>) {
    match ds {
        &DefiningSkill::Busking => (
            l.msg("defining-skill-bard-name"),
            "EMP".to_string(),
            l.msg("defining-skill-bard-desc"),
        ),
        &DefiningSkill::PatchJob => (
            l.msg("defining-skill-craftsman-name"),
            "CRA".to_string(),
            l.msg("defining-skill-craftsman-desc"),
        ),
        &DefiningSkill::PracticedParanoia => (
            l.msg("defining-skill-criminal-name"),
            "INT".to_string(),
            l.msg("defining-skill-criminal-desc"),
        ),
        &DefiningSkill::HealingHands => (
            l.msg("defining-skill-doctor-name"),
            "CRA".to_string(),
            l.msg("defining-skill-doctor-desc"),
        ),
        &DefiningSkill::MagicTraining => (
            l.msg("defining-skill-mage-name"),
            "INT".to_string(),
            l.msg("defining-skill-mage-desc"),
        ),
        &DefiningSkill::ToughAsNails => (
            l.msg("defining-skill-man-at-arms-name"),
            "BODY".to_string(),
            l.msg("defining-skill-man-at-arms-desc"),
        ),
        &DefiningSkill::WellTraveled => (
            l.msg("defining-skill-merchant-name"),
            "INT".to_string(),
            l.msg("defining-skill-merchant-desc"),
        ),
        &DefiningSkill::InitiateOfTheGods => (
            l.msg("defining-skill-priest-name"),
            "EMP".to_string(),
            l.msg("defining-skill-priest-desc"),
        ),
        &DefiningSkill::WitcherTraining => (
            l.msg("defining-skill-witcher-name"),
            "INT".to_string(),
            l.msg("defining-skill-witcher-desc"),
        ),
        &DefiningSkill::Notoriety => (
            l.msg("defining-skill-notoriety-name"),
            "EMP".to_string(),
            l.msg("defining-skill-notoriety-desc"),
        ),
        _ => (
            Some("?".to_string()),
            "?".to_string(),
            Some("?".to_string()),
        ),
    }
}
