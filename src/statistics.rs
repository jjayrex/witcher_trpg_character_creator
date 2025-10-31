use crate::locale::Locale;

enum SkillLevel {
    Inept,
    Everyday,
    Competent,
    Heroic,
    Incredible,
    Legendary,
    Superheroic,
}

pub enum Statistics {
    INT,
    REF,
    DEX,
    BODY,
    SPD,
    EMP,
    CRA,
    WILL,
    LUCK,
}

pub enum Skills {
    // INT
    Awareness,
    Business,
    Deduction,
    Education,
    Language,
    MonsterLore,
    SocialEtiquette,
    Streetwise,
    Tactics,
    Teaching,
    WildernessSurvival,
    // REF
    Brawling,
    DodgeEscape,
    Melee,
    Riding,
    Sailing,
    SmallBlades,
    StaffSpear,
    Swordsmanship,
    // DEX
    Archery,
    Athletics,
    Crossbow,
    SleightOfHand,
    Stealth,
    // BODY
    Physique,
    Endurance,
    // EMP
    Charisma,
    Deceit,
    FineArts,
    Gambling,
    GroomingStyle,
    HumanPerception,
    Leadership,
    Persuasion,
    Performance,
    Seduction,
    // CRA
    Alchemy,
    Crafting,
    Disguise,
    FirstAid,
    Forgery,
    PickLocks,
    TrapCrafting,
    // WILL
    Courage,
    HexWeaving,
    Intimidation,
    SpellCasting,
    ResistMagic,
    ResistCoercion,
    RitualCrafting,
    // Other
    CombatSkill,
}

pub fn read_stats(s: &Statistics, l: &Locale) -> (Option<String>, Option<String>){
    match s {
        &Statistics::INT => (l.msg("stat-int-name"), l.msg("stat-int-desc")),
        &Statistics::REF => (l.msg("stat-ref-name"), l.msg("stat-ref-desc")),
        &Statistics::DEX => (l.msg("stat-dex-name"), l.msg("stat-dex-desc")),
        &Statistics::BODY => (l.msg("stat-body-name"), l.msg("stat-body-desc")),
        &Statistics::SPD => (l.msg("stat-spd-name"), l.msg("stat-spd-desc")),
        &Statistics::EMP => (l.msg("stat-emp-name"), l.msg("stat-emp-desc")),
        &Statistics::CRA => (l.msg("stat-cra-name"), l.msg("stat-cra-desc")),
        &Statistics::WILL => (l.msg("stat-will-name"), l.msg("stat-will-desc")),
        &Statistics::LUCK => (l.msg("stat-luck-name"), l.msg("stat-luck-desc")),
        _ => (Some("?".to_string()), Some("?".to_string())),
    }
}

pub fn read_skills(p: &Skills, l: &Locale) -> (Option<String>, Option<String>){
    match p {
        // INT
        &Skills::Awareness => (l.msg("skill-awareness-name"), l.msg("skill-awareness-desc")),
        &Skills::Business => (l.msg("skill-business-name"), l.msg("skill-business-desc")),
        &Skills::Deduction => (l.msg("skill-deduction-name"), l.msg("skill-deduction-desc")),
        &Skills::Education => (l.msg("skill-education-name"), l.msg("skill-education-desc")),
        &Skills::Language => (l.msg("skill-language-name"), l.msg("skill-language-desc")),
        &Skills::MonsterLore => (l.msg("skill-monster-lore-name"), l.msg("skill-monster-lore-desc")),
        &Skills::SocialEtiquette => (l.msg("skill-social-etiquette-name"), l.msg("skill-social-etiquette-desc")),
        &Skills::Streetwise => (l.msg("skill-streetwise-name"), l.msg("skill-streetwise-desc")),
        &Skills::Tactics => (l.msg("skill-tactics-name"), l.msg("skill-tactics-desc")),
        &Skills::Teaching => (l.msg("skill-teaching-name"), l.msg("skill-teaching-desc")),
        &Skills::WildernessSurvival => (l.msg("skill-wilderness-survival-name"), l.msg("skill-wilderness-survival-desc")),
        // REF
        &Skills::Brawling => (l.msg("skill-brawling-name"), l.msg("skill-brawling-desc")),
        &Skills::DodgeEscape => (l.msg("skill-dodge-escape-name"), l.msg("skill-dodge-escape-desc")),
        &Skills::Melee => (l.msg("skill-melee-name"), l.msg("skill-melee-desc")),
        &Skills::Riding => (l.msg("skill-riding-name"), l.msg("skill-riding-desc")),
        &Skills::Sailing => (l.msg("skill-sailing-name"), l.msg("skill-sailing-desc")),
        &Skills::SmallBlades => (l.msg("skill-small-blades-name"), l.msg("skill-small-blades-desc")),
        &Skills::StaffSpear => (l.msg("skill-staff-spear-name"), l.msg("skill-staff-spear-desc")),
        &Skills::Swordsmanship => (l.msg("skill-swordsmanship-name"), l.msg("skill-swordsmanship-desc")),
        // DEX
        &Skills::Archery => (l.msg("skill-archery-name"), l.msg("skill-archery-desc")),
        &Skills::Athletics => (l.msg("skill-athletics-name"), l.msg("skill-athletics-desc")),
        &Skills::Crossbow => (l.msg("skill-crossbow-name"), l.msg("skill-crossbow-desc")),
        &Skills::SleightOfHand => (l.msg("skill-sleight-of-hand-name"), l.msg("skill-sleight-of-hand-desc")),
        &Skills::Stealth => (l.msg("skill-stealth-name"), l.msg("skill-stealth-desc")),
        // BODY
        &Skills::Physique => (l.msg("skill-physique-name"), l.msg("skill-physique-desc")),
        &Skills::Endurance => (l.msg("skill-endurance-name"), l.msg("skill-endurance-desc")),
        // EMP
        &Skills::Charisma => (l.msg("skill-charisma-name"), l.msg("skill-charisma-desc")),
        &Skills::Deceit => (l.msg("skill-deceit-name"), l.msg("skill-deceit-desc")),
        &Skills::FineArts => (l.msg("skill-fine-arts-name"), l.msg("skill-fine-arts-desc")),
        &Skills::Gambling => (l.msg("skill-gambling-name"), l.msg("skill-gambling-desc")),
        &Skills::GroomingStyle => (l.msg("skill-grooming-style-name"), l.msg("skill-grooming-style-desc")),
        &Skills::HumanPerception => (l.msg("skill-human-perception-name"), l.msg("skill-human-perception-desc")),
        &Skills::Leadership => (l.msg("skill-leadership-name"), l.msg("skill-leadership-desc")),
        &Skills::Persuasion => (l.msg("skill-persuasion-name"), l.msg("skill-persuasion-desc")),
        &Skills::Performance => (l.msg("skill-performance-name"), l.msg("skill-performance-desc")),
        &Skills::Seduction => (l.msg("skill-seduction-name"), l.msg("skill-seduction-desc")),
        // CRA
        &Skills::Alchemy => (l.msg("skill-alchemy-name"), l.msg("skill-alchemy-desc")),
        &Skills::Crafting => (l.msg("skill-crafting-name"), l.msg("skill-crafting-desc")),
        &Skills::Disguise => (l.msg("skill-disguise-name"), l.msg("skill-disguise-desc")),
        &Skills::FirstAid => (l.msg("skill-first-aid-name"), l.msg("skill-first-aid-desc")),
        &Skills::Forgery => (l.msg("skill-forgery-name"), l.msg("skill-forgery-desc")),
        &Skills::PickLocks => (l.msg("skill-pick-locks-name"), l.msg("skill-pick-locks-desc")),
        &Skills::TrapCrafting => (l.msg("skill-trap-crafting-name"), l.msg("skill-trap-crafting-desc")),
        // WILL
        &Skills::Courage => (l.msg("skill-courage-name"), l.msg("skill-courage-desc")),
        &Skills::HexWeaving => (l.msg("skill-hex-weaving-name"), l.msg("skill-hex-weaving-desc")),
        &Skills::Intimidation => (l.msg("skill-intimidation-name"), l.msg("skill-intimidation-desc")),
        &Skills::SpellCasting => (l.msg("skill-spell-casting-name"), l.msg("skill-spell-casting-desc")),
        &Skills::ResistMagic => (l.msg("skill-resist-magic-name"), l.msg("skill-resist-magic-desc")),
        &Skills::ResistCoercion => (l.msg("skill-resist-coercion-name"), l.msg("skill-resist-coercion-desc")),
        &Skills::RitualCrafting => (l.msg("skill-ritual-crafting-name"), l.msg("skill-ritual-crafting-desc")),
        _ => (Some("?".to_string()), Some("?".to_string())),
    }
}
