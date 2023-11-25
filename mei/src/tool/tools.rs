use {
    semver::VersionReq,
    std::collections::HashMap,
    toml::{Item, TableLike},
};

#[derive(Debug)]
pub(crate) struct Tool {
    pub version: VersionReq,
    pub from_crate: Option<Box<str>>,
}

impl Tool {
    pub fn from_toml(item: &Item) -> Option<Self> {
        item.as_table_like()
            .and_then(|t| {
                Some(Self {
                    version: t.get("version")?.as_str()?.parse().ok()?,
                    from_crate: t.get("crate").and_then(Item::as_str).map(Box::from),
                })
            })
            .or_else(|| {
                item.as_str().and_then(|s| {
                    Some(Self {
                        version: s.parse().ok()?,
                        from_crate: None,
                    })
                })
            })
    }
}

#[derive(Default)]
pub(crate) struct Tools(HashMap<Box<str>, Tool>);

impl Tools {
    pub fn from_toml(table: &dyn TableLike) -> Self {
        Self(
            table
                .iter()
                .filter_map(|(name, item)| Some((Box::from(name), Tool::from_toml(item)?)))
                .collect(),
        )
    }

    pub fn get(&self, name: &str) -> Option<&Tool> {
        self.0.get(name)
    }
}
