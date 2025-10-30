use fluent_bundle::{FluentBundle, FluentResource};
use std::fs;
use std::sync::{Arc, RwLock};
use unic_langid::langid;

#[derive(Clone)]
pub struct Locale(Arc<RwLock<FluentBundle<FluentResource>>>);

impl Locale {
    pub fn new(bundle: FluentBundle<FluentResource>) -> Self {
        Self(Arc::new(RwLock::new(bundle)))
    }

    pub fn msg(&self, key: &str) -> Option<String> {
        let b = self.0.read().ok()?;
        let msg = b.get_message(key)?;
        let pat = msg.value()?;
        let mut errs = Vec::new();
        Some(b.format_pattern(pat, None, &mut errs).into_owned())
    }
}

pub fn build_bundle() -> FluentBundle<FluentResource> {
    let langid = langid!("en-US");

    let locale: String = if langid == langid!("pl-PL") {
        fs::read_to_string("locale/pl.ftl").expect("missing pl.ftl")
    } else {
        fs::read_to_string("locale/en.ftl").expect("missing en.ftl")
    };

    let mut bundle = FluentBundle::new(vec![langid]);

    let res = FluentResource::try_new(locale).expect("invalid FTL");
    bundle.add_resource(res).expect("failed to add resource");
    bundle
}
