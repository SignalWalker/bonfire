use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, serde::Serialize)]
pub struct CrateVersion<'v>(pub &'v str, pub &'v str, pub &'v str);

impl<'v> Display for CrateVersion<'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

impl<'v> CrateVersion<'v> {
    pub fn new(version: &'v str) -> Option<Self> {
        let mut split = version.split('.');
        Some(CrateVersion(split.next()?, split.next()?, split.next()?))
    }
}

#[macro_export]
macro_rules! crate_version {
    () => {{
        pub(crate) mod _version {
            pub(crate) const VERSION_STR: &str = clap::crate_version!();
        }
        CrateVersion::<'static>::new(_version::VERSION_STR).unwrap()
    }};
}

lazy_static::lazy_static! {
    pub static ref VERSION: CrateVersion<'static> = crate_version!();
}
