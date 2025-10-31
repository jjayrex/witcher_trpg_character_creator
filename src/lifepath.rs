use std::panic::Location;
use crate::locale::Locale;

enum Origins {
    // North
    Redania,
    Kaedwen,
    Temeria,
    Aedirn,
    LyriaRivia,
    KovirPoviss,
    Skellige,
    Cidaris,
    Verden,
    Cintra,
    // Nilfgaard
    HeartOfNilfgaard,
    Vicovaro,
    Angren,
    Nazair,
    Mettina,
    MagTurga,
    Gheso,
    Ebbing,
    Maecht,
    Gemmeria,
    Etolia,
    // Elderlands
    DolBlathanna,
    Mahakam,
}

enum Region {
    North,
    Nilfgaard,
    Elderlands,
    Beyond,
}

enum Personality {
    Shy,
    Aggressive,
    Kind,
    Strange,
    Thoughtful,
    Talkative,
    Romantic,
    Stern,
    Depressive,
    Immature,
}

enum Feelings {
    WantsYouDead,
    CantStandYou,
    JealousOfYou,
    NoFeelingsAboutYou,
    LikesYou,
    LooksUpToYou,
    PossessiveOfYou,
}

enum Age {
    Younger,
    Older,
    Twin,
}

enum Sex {
    Male,
    Female,
}

enum LifeEvents {
    // Fortune
    Jackpot,
    FindTeacher,
    NobleOwesYou,
    FindCombatTeacher,
    WitcherOwesYou,
    FellInWithBandits,
    TamedWildAnimal,
    MageOwesYou,
    BlessedByPriest,
    Knighted,
    // Misfortune
    Debt,
    Imprisonment,
    Addiction,
    AcquaintanceKilled,
    FalseAccusation,
    HuntedByLaw,
    Betrayal,
    Accident,
    Incapacitation,
    Cursed,
}

enum Position {
    // Ally
    BountyHunter,
    Mage,
    MentorTeacher,
    ChildhoodFriend,
    Craftsman,
    OldEnemy,
    DukeDuchess,
    PriestPriestess,
    Soldier,
    Bard,
    // Enemy
    ExFriend,
    ExLover,
    Relative,
    ChildhoodEnemy,
    Cultist,
    Bandit,
}

enum Meeting {
    SavedThem,
    MetInTavern,
    SavedYou,
    HiredYou,
    TrappedTogether,
    ForcedToWorkTogether,
    HiredThem,
    MetWhileDrunk,
    MetWhileTraveling,
    FoughtTogether,
}

enum Cause {
    AssaultedThem,
    CausedLossOfLovedOne,
    CausedHumiliation,
    CausedCurse,
    AccusedOfWitchcraft,
    TurnedDown,
    CausedWound,
    Blackmail,
    FoiledPlans,
    CausedMonsterAttack,
}

enum Closeness {
    Acquaintances,
    Friends,
    CloseFriends,
    Inseperable,
    BoundByBond,
}

enum Escalation {
    MostlyForgotten,
    PlanToBackstab,
    AttackIfEncountered,
    HuntForRevenge,
    OutForBlood,
}

enum PowerType {
    SocialPower,
    Knowledge,
    Physical,
    Minions,
    Magic,
}

enum LoveAffair {
    HappyLoveAffair,
    RomanticTragedy,
    ProblematicLove,
    WhoresAndDebauchery,
}

struct Ally {
    sex: Sex,
    position: Position,
    meeting: Meeting,
    closeness: Closeness,
    location: Region,
}

struct Enemy {
    sex: Sex,
    position: Position,
    cause: Cause,
    were_you_wronged: bool,
    power: u8,
    escalation: Escalation,
    power_type: PowerType,
}

struct Sibling {
    sex: Sex,
    age: Age,
    feelings: String,
    personality: Personality,
}

struct Romance {
    love_affair: LoveAffair,
    tragedy: u8,
    problem: u8,
}

struct Lifepath {
    region: Region,
    homeland: Origins,
    is_family_alive: bool,
    are_parents_alive: bool,
    family: u8,
    parents: u8,
    friend: u8,
    siblings: Vec<Sibling>,
    life_events: Vec<LifeEvents>,
    ally: Vec<Ally>,
    enemy: Vec<Enemy>,
    romance: Vec<Romance>,
}

pub fn read_life_events(le: &LifeEvents, l: &Locale) -> (Option<String>, Option<String>){
    match le {
        &LifeEvents::Jackpot => (l.msg("fortune-jackpot-name"), l.msg("fortune-jackpot-desc")),
        &LifeEvents::FindTeacher => (l.msg("fortune-find-teacher-name"), l.msg("fortune-find-teacher-desc")),
        &LifeEvents::NobleOwesYou => (l.msg("fortune-noble-owes-you-name"), l.msg("fortune-noble-owes-you-desc")),
        &LifeEvents::FindCombatTeacher => (l.msg("fortune-find-combat-teacher-name"), l.msg("fortune-find-combat-teacher-desc")),
        &LifeEvents::WitcherOwesYou => (l.msg("fortune-witcher-owes-you-name"), l.msg("fortune-witcher-owes-you-desc")),
        &LifeEvents::FellInWithBandits => (l.msg("fortune-fell-in-with-bandits-name"), l.msg("fortune-fell-in-with-bandits-desc")),
        &LifeEvents::TamedWildAnimal => (l.msg("fortune-tamed-wild-animal-name"), l.msg("fortune-tamed-wild-animal-desc")),
        &LifeEvents::MageOwesYou => (l.msg("fortune-mage-owes-you-name"), l.msg("fortune-mage-owes-you-desc")),
        &LifeEvents::BlessedByPriest => (l.msg("fortune-blessed-by-priest-name"), l.msg("fortune-blessed-by-priest-desc")),
        &LifeEvents::Knighted => (l.msg("fortune-knighted-name"), l.msg("fortune-knighted-desc")),
        &LifeEvents::Debt => (l.msg("misfortune-debt-name"), l.msg("misfortune-debt-desc")),
        &LifeEvents::Imprisonment => (l.msg("misfortune-imprisonment-name"), l.msg("misfortune-imprisonment-desc")),
        &LifeEvents::Addiction => (l.msg("misfortune-addiction-name"), l.msg("misfortune-addiction-desc")),
        &LifeEvents::AcquaintanceKilled => (l.msg("misfortune-acquaintance-killed-name"), l.msg("misfortune-acquaintance-killed-desc")),
        &LifeEvents::FalseAccusation => (l.msg("misfortune-false-accusation-name"), l.msg("misfortune-false-accusation-desc")),
        &LifeEvents::HuntedByLaw => (l.msg("misfortune-hunted-by-law-name"), l.msg("misfortune-hunted-by-law-desc")),
        &LifeEvents::Betrayal => (l.msg("misfortune-betrayal-name"), l.msg("misfortune-betrayal-desc")),
        &LifeEvents::Accident => (l.msg("misfortune-accident-name"), l.msg("misfortune-accident-desc")),
        &LifeEvents::Incapacitation => (l.msg("misfortune-incapacitation-name"), l.msg("misfortune-incapacitation-desc")),
        &LifeEvents::Cursed => (l.msg("misfortune-cursed-name"), l.msg("misfortune-cursed-desc")),
        _ => (Some("?".to_string()), Some("?".to_string())),
    }
}