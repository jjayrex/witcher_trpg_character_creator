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

    pub fn msg(&self, key: &str) -> String {
        let b = self.0.read().ok().unwrap();
        let msg = b.get_message(key).unwrap();
        let pat = msg.value().unwrap();
        let mut errs = Vec::new();
        b.format_pattern(pat, None, &mut errs).into_owned()
    }
}

pub fn build_bundle() -> FluentBundle<FluentResource> {
    let langid = langid!("en-US");
    let locale = "".to_string();

    if langid == langid!("pl-PL") {
        let locale = fs::read_to_string("locale/pl.ftl").unwrap();
    } else {
        let locale = fs::read_to_string("locale/en.ftl").unwrap();
    }

    let mut bundle = FluentBundle::new(vec![langid]);

    let res = FluentResource::try_new(locale).unwrap();

    bundle.add_resource(res);
    bundle
}
