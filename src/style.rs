enum Clothing {
    Uniform,
    Traveling,
    Fancy,
    Ragged,
    Utilitarian,
    Traditional,
    Revealing,
    Heavy,
    Strange,
    Flamboyant,
}

enum Personality {
    Secretive,
    Rebellious,
    Violent,
    Idealistic,
    Contemplative,
    Stern,
    Deceptive,
    Friendly,
    Arrogant,
    Nervous,
}

enum HairStyle {
    LongLoose,
    CroppedShort,
    SelfCutShort,
    Braided,
    LongWild,
    Bald,
    UniformlyShort,
    RaggedMessy,
    ComplicatedHairstyle,
    ShavenSides,
}

enum Affectations {
    Trophies,
    RingsJewlery,
    Trinkets,
    Tattoos,
    WarPaint,
    ShadowyCloak,
    BrightBandanas,
    EyePatch,
    Furs,
    InsigniasPlaques,
}

enum ValuedPerson {
    Parent,
    Sibling,
    Lover,
    Friend,
    Yourself,
    Pet,
    Mentor,
    PublicFigure,
    PersonalHero,
    NoOne,
}

enum Value {
    Money,
    Honor,
    YourWord,
    HedonisticPursuits,
    Knowledge,
    Vengeance,
    Power,
    Love,
    Survival,
    Friendship,
}

enum Feelings {
    ToolsToBeUsed,
    PloughTheRest,
    CanNeverBeTrusted,
    HaveToProveThemselves,
    Neutral,
    AreGreat,
    DeservesToDie,
    HedonisticSwine,
    LifeIsValuable,
}

struct PersonalStyle {
    clothing: Clothing,
    personality: Personality,
    hair_style: HairStyle,
    affectations: Affectations,
    valued_person: ValuedPerson,
    value: Value,
    feelings: Feelings,
}










