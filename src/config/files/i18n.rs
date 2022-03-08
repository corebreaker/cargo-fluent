use serde::Deserialize;

#[derive(Deserialize)]
pub(in super::super) struct I18nGettext {
    pub(in super::super) output_dir: Option<String>,
    pub(in super::super) po_dir: Option<String>
}

impl I18nGettext {
    pub(super) fn is_empty(&self) -> bool {
        self.output_dir.as_ref().map_or(true, |d| d.is_empty()) && self.po_dir.as_ref().map_or(true, |d| d.is_empty())
    }
}

#[derive(Deserialize)]
pub(in super::super) struct I18nFluent {
    pub(in super::super) assets_dir: Option<String>
}

impl I18nFluent {
    pub(super) fn is_empty(&self) -> bool {
        self.assets_dir.as_ref().map_or(true, |d| d.is_empty())
    }
}

#[derive(Deserialize)]
pub(in super::super) struct I18nFile {
    pub(in super::super) fallback_language: Option<String>,
    pub(in super::super) fluent: Option<I18nFluent>,
    pub(in super::super) gettext: Option<I18nGettext>
}

impl I18nFile {
    pub(super) fn is_empty(&self) -> bool {
        self.fallback_language.as_ref().map_or(true, |l| l.is_empty())
            && self.fluent.as_ref().map_or(true, |c| c.is_empty())
            && self.gettext.as_ref().map_or(true, |c| c.is_empty())
    }
}