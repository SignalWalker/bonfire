use bonfire_core::meta::CrateVersion;

#[derive(Debug, Clone, serde::Serialize)]
pub struct InstanceConfig {
    pub url: String,
    pub name: String,
    pub description: String,
    pub lang: String,
    pub app_version: CrateVersion<'static>,
    pub core_version: CrateVersion<'static>,
}

impl InstanceConfig {
    pub fn new(
        name: impl Into<String>,
        url: impl Into<String>,
        lang: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        InstanceConfig {
            name: name.into(),
            url: url.into(),
            lang: lang.into(),
            description: description.into(),
            app_version: *crate::VERSION,
            core_version: *bonfire_core::meta::VERSION,
        }
    }
}
