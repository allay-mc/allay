//! Localization related utilities.

#[cfg(feature = "config-schema")]
use schemars::JsonSchema;
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    fmt, fs, io,
    path::Path,
};

/// Returns `true` when the key is valid for translations.
///
/// See also:
/// <https://learn.microsoft.com/en-us/minecraft/creator/documents/preparingrawtextforlocalization?view=minecraft-bedrock-stable>
pub fn is_valid_translation_key(key: &str) -> bool {
    !key.contains("=") && !key.contains(char::is_whitespace)
}

/// Common keys used for localization files.
pub mod keys {
    /// The key for the name of the pack.
    pub const fn pack_name() -> &'static str {
        "pack.name"
    }

    /// The key of the description of the pack.
    pub const fn pack_description() -> &'static str {
        "pack.description"
    }

    /// The key of the name for the subpack with the ID `id`.
    ///
    /// `id` must not a valid key. This function does not do any validation.
    pub fn subpack_name(id: &str) -> String {
        format!("subpack.{}.name", id)
    }
}

/// A function that takes in a translation and the language that is expected and returns the value
/// that matches the best.
///
/// For example if a value contains a translation for English (US) and German and English (GB) is
/// expected, the most logical return value would be the value of English (US). The behavior can
/// be controlled with this function for all languages.
///
/// If the localized value has no translation (which should never be the case), the function
/// returns [`None`].
pub type FallbackHandler<T> = dyn Fn(&Localized<T>, Language) -> Option<&T>;

/// A collection of languages that form a group used for fallbacks.
pub type LanguageGroup = HashSet<Language>;

/// Multiple [`LanguageGroup`]s.
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
pub struct LanguageGroups(pub Vec<LanguageGroup>);

impl Default for LanguageGroups {
    fn default() -> Self {
        Self(vec![
            HashSet::from([Language::IndonesiaIndonesian]),
            HashSet::from([Language::DenmarkDanish]),
            HashSet::from([Language::GermanyGerman]),
            HashSet::from([Language::GreatBritainEnglish, Language::NorthAmericaEnglish]),
            HashSet::from([Language::SpainSpanish, Language::MexicoSpanish]),
            HashSet::from([Language::FranceFrench, Language::CanadaFrench]),
            HashSet::from([Language::ItalyItalian]),
            HashSet::from([Language::HungaryHungarian]),
            HashSet::from([Language::NetherlandsDutch]),
            HashSet::from([Language::NorwayBokmål]),
            HashSet::from([Language::PolandPolish]),
            HashSet::from([Language::PortugalProtugese, Language::BrazilPortuguese]),
            HashSet::from([Language::SlovakiaSlovak]),
            HashSet::from([Language::FinlandFinnish]),
            HashSet::from([Language::SwedenSwedish]),
            HashSet::from([Language::TurkeyTurkish]),
            HashSet::from([Language::CzeshRepublicCzech]),
            HashSet::from([Language::GreeceGreek]),
            HashSet::from([Language::BulgariaBulgarian]),
            HashSet::from([Language::RussiaRussian]),
            HashSet::from([Language::UkraineUkrainian]),
            HashSet::from([Language::JapanJapanese]),
            HashSet::from([Language::ChinaChinese, Language::TaiwanChinese]),
            HashSet::from([Language::KoreaKorean]),
        ])
    }
}

impl IntoIterator for LanguageGroups {
    type Item = LanguageGroup;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Generates a [`FallbackHandler`] from [`LanguageGroups`].
pub fn fallback_handler_by_groups<T>(groups: LanguageGroups) -> Box<FallbackHandler<T>> {
    Box::new(
        move |localized: &Localized<T>, language: Language| -> Option<&T> {
            // Attempt 1: try exact match
            let exact_match = localized.get(&language);
            if let Some(value) = exact_match {
                return Some(value);
            }

            // Attempt 2: try an alternative by language group
            for group in &groups.0 {
                let has_group = group.contains(&language);
                if has_group {
                    for candidate in group {
                        if let Some(value) = localized.get(candidate) {
                            return Some(value);
                        }
                    }
                    break;
                }
            }

            // Attempt 3: try any other language
            for candidate in groups.0.iter().flatten() {
                if let Some(value) = localized.get(candidate) {
                    return Some(value);
                }
            }

            // Last resort: nothing was found
            None
        },
    )
}

/// A value mapped to languages.
pub type Localized<T> = HashMap<Language, T>;

/// A value optionally mapped to languages.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[serde(untagged)]
pub enum OptionallyLocalized<T> {
    /// A localized value.
    Localized(Localized<T>),

    /// A value that is not localized.
    Unlocalized(T),
}

/// File extension for Minecraft language files.
pub const LANGUAGE_FILE_EXTENSION: &str = "lang";

/// Adds a single translation to the appropiate language file.
pub fn add_translation_to_file<D>(
    dir: &Path,
    language: Language,
    key: &str,
    value: &D,
) -> io::Result<()>
where
    D: fmt::Display,
{
    let file_path = dir
        .join(language.file_id())
        .with_extension(LANGUAGE_FILE_EXTENSION);
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;
    append_translation_data(&mut file, key, &value)
}

/// Adds translations to translation files without respecting fallbacks.
///
/// `dir` is the directory containing the `.lang` files. This function only covers languages specified in `translation`.
pub fn add_translation_without_fallbacks_to_files<D>(
    dir: &Path,
    key: &str,
    translation: &Localized<D>,
) -> io::Result<()>
where
    D: fmt::Display,
{
    for (language, text) in translation {
        add_translation_to_file(dir, *language, key, text)?;
    }
    Ok(())
}

/// Adds translations to translation files by respecting fallbacks.
///
/// `dir` is the directory containing the `.lang` files. This function covers **all** languages.
///
/// # Panics
///
/// This function panics when translation is [`OptionallyLocalized::Localized`] and an empty
/// [`HashMap`].
pub fn add_translation_with_fallbacks_to_files<D>(
    dir: &Path,
    key: &str,
    translation: &OptionallyLocalized<D>,
    fallback_handler: &FallbackHandler<D>,
) -> io::Result<()>
where
    D: fmt::Display,
{
    for language in Language::vanilla() {
        let value = match translation {
            OptionallyLocalized::Localized(localized) =>
            {
                #[allow(clippy::expect_used)]
                fallback_handler(localized, *language).expect("empty translation")
            }
            OptionallyLocalized::Unlocalized(value) => value,
        };
        add_translation_to_file(dir, *language, key, value)?;
    }
    Ok(())
}

/// Appends a translation to translation data (usually a `.lang` file).
///
/// # Panics
///
/// Panics when the translation key is invalid.
pub fn append_translation_data<W, D>(data: &mut W, key: &str, value: &D) -> io::Result<()>
where
    W: io::Write,
    D: fmt::Display,
{
    if !is_valid_translation_key(key) {
        panic!("Translation key is invalid");
    }
    writeln!(data, "{}={}\t## @generated by Allay", key, value)
}

/// Generates `languages.json` by finding each `.lang` file in `dir`.
pub fn generate_languages_data<P>(dir: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let mut languages = Vec::new();
    for entry in dir.as_ref().read_dir()? {
        let path = entry?.path();
        if !path.is_file() {
            continue;
        }
        let Some(file_stem) = path.file_stem().and_then(|p| p.to_str()) else {
            continue;
        };
        if path
            .extension()
            .is_some_and(|ext| ext.to_string_lossy() == LANGUAGE_FILE_EXTENSION)
        {
            languages.push(file_stem.to_string());
        };
    }

    Ok(languages)
}

/// Languages that are natively supported by Minecraft.
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "config-schema", derive(JsonSchema))]
#[allow(missing_docs)]
pub enum Language {
    #[serde(rename = "id-id")]
    IndonesiaIndonesian,

    #[serde(rename = "da-dk")]
    DenmarkDanish,

    #[serde(rename = "de-de")]
    GermanyGerman,

    #[serde(rename = "en-gb")]
    GreatBritainEnglish,

    #[default]
    #[serde(rename = "en-us")]
    NorthAmericaEnglish,

    #[serde(rename = "es-es")]
    SpainSpanish,

    #[serde(rename = "es-mx")]
    MexicoSpanish,

    #[serde(rename = "fr-ca")]
    CanadaFrench,

    #[serde(rename = "fr-fr")]
    FranceFrench,

    #[serde(rename = "it-it")]
    ItalyItalian,

    #[serde(rename = "hu-hu")]
    HungaryHungarian,

    #[serde(rename = "nl-nl")]
    NetherlandsDutch,

    #[serde(rename = "nb-no")]
    NorwayBokmål,

    #[serde(rename = "pl-pl")]
    PolandPolish,

    #[serde(rename = "pt-br")]
    BrazilPortuguese,

    #[serde(rename = "pt-pt")]
    PortugalProtugese,

    #[serde(rename = "sk-sk")]
    SlovakiaSlovak,

    #[serde(rename = "fi-fi")]
    FinlandFinnish,

    #[serde(rename = "sv-se")]
    SwedenSwedish,

    #[serde(rename = "tr-tr")]
    TurkeyTurkish,

    #[serde(rename = "cs-cz")]
    CzeshRepublicCzech,

    #[serde(rename = "el-gr")]
    GreeceGreek,

    #[serde(rename = "bg-bg")]
    BulgariaBulgarian,

    #[serde(rename = "ru-ru")]
    RussiaRussian,

    #[serde(rename = "uk-ua")]
    UkraineUkrainian,

    #[serde(rename = "ja-jp")]
    JapanJapanese,

    #[serde(rename = "zh-cn")]
    ChinaChinese,

    #[serde(rename = "zh-tw")]
    TaiwanChinese,

    #[serde(rename = "ko-kr")]
    KoreaKorean,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Language {
    /// Ascii-compatible alias for [`Language::NorwayBokmål`].
    #[allow(non_upper_case_globals)]
    pub const NorwayBokmal: Self = Self::NorwayBokmål;

    /// Returns a human-readable representation of the language.
    pub const fn as_str(&self) -> &'static str {
        use Language as L;
        match self {
            L::IndonesiaIndonesian => "Indonesian",
            L::DenmarkDanish => "Danish",
            L::GermanyGerman => "German",
            L::GreatBritainEnglish => "English (UK)",
            L::NorthAmericaEnglish => "English (US)",
            L::SpainSpanish => "Spanish",
            L::MexicoSpanish => "Mexican Spanish",
            L::CanadaFrench => "Canadian",
            L::FranceFrench => "French",
            L::ItalyItalian => "Italian",
            L::HungaryHungarian => "Hungarian",
            L::NetherlandsDutch => "Dutch",
            L::NorwayBokmål => "Bokmål",
            L::PolandPolish => "Polish",
            L::BrazilPortuguese => "Portuguese (Brazil)",
            L::PortugalProtugese => "Portuguese (Portugal)",
            L::SlovakiaSlovak => "Slovak",
            L::FinlandFinnish => "Finnish",
            L::SwedenSwedish => "Swedish",
            L::TurkeyTurkish => "Turkish",
            L::CzeshRepublicCzech => "Czech",
            L::GreeceGreek => "Greek",
            L::BulgariaBulgarian => "Bulgarian",
            L::RussiaRussian => "Russian",
            L::UkraineUkrainian => "Ukrainian",
            L::JapanJapanese => "Japanese",
            L::ChinaChinese => "Chinese (Simplified)",
            L::TaiwanChinese => "Chinese (Traditional)",
            L::KoreaKorean => "Korean",
        }
    }

    /// Returns the file ID of the lanuage (e.g. `id_ID` for Indonesian).
    pub const fn file_id(&self) -> &'static str {
        use Language as L;
        match self {
            L::IndonesiaIndonesian => "id_ID",
            L::DenmarkDanish => "da_DK",
            L::GermanyGerman => "de_DE",
            L::GreatBritainEnglish => "en_GB",
            L::NorthAmericaEnglish => "en_US",
            L::SpainSpanish => "es_ES",
            L::MexicoSpanish => "es_MX",
            L::CanadaFrench => "fr_CA",
            L::FranceFrench => "fr_FR",
            L::ItalyItalian => "it_IT",
            L::HungaryHungarian => "hu_HU",
            L::NetherlandsDutch => "nl_NL",
            L::NorwayBokmål => "nb_NO",
            L::PolandPolish => "pl_PL",
            L::BrazilPortuguese => "pt_BR",
            L::PortugalProtugese => "pt_PT",
            L::SlovakiaSlovak => "sk_SK",
            L::FinlandFinnish => "fi_FI",
            L::SwedenSwedish => "sv_SE",
            L::TurkeyTurkish => "tr_TR",
            L::CzeshRepublicCzech => "cs_CZ",
            L::GreeceGreek => "el_GR",
            L::BulgariaBulgarian => "bg_BG",
            L::RussiaRussian => "ru_RU",
            L::UkraineUkrainian => "uk_UA",
            L::JapanJapanese => "ja_JP",
            L::ChinaChinese => "zh_CN",
            L::TaiwanChinese => "zh_TW",
            L::KoreaKorean => "ko_KR",
        }
    }

    /// Returns the ID of the language used by Allay (for example `allay.toml` file).
    pub const fn id(&self) -> &'static str {
        use Language as L;
        match self {
            L::IndonesiaIndonesian => "id-id",
            L::DenmarkDanish => "da-dk",
            L::GermanyGerman => "de-de",
            L::GreatBritainEnglish => "en-gb",
            L::NorthAmericaEnglish => "en-us",
            L::SpainSpanish => "es-es",
            L::MexicoSpanish => "es-mx",
            L::CanadaFrench => "fr-ca",
            L::FranceFrench => "fr-fr",
            L::ItalyItalian => "it-it",
            L::HungaryHungarian => "hu-hu",
            L::NetherlandsDutch => "nl-nl",
            L::NorwayBokmål => "nb-no",
            L::PolandPolish => "pl-pl",
            L::BrazilPortuguese => "pt-br",
            L::PortugalProtugese => "pt-pt",
            L::SlovakiaSlovak => "sk-sk",
            L::FinlandFinnish => "fi-fi",
            L::SwedenSwedish => "sv-se",
            L::TurkeyTurkish => "tr-tr",
            L::CzeshRepublicCzech => "cs-cz",
            L::GreeceGreek => "el-gr",
            L::BulgariaBulgarian => "bg-bg",
            L::RussiaRussian => "ru-ru",
            L::UkraineUkrainian => "uk-ua",
            L::JapanJapanese => "ja-jp",
            L::ChinaChinese => "zh-cn",
            L::TaiwanChinese => "zh-tw",
            L::KoreaKorean => "ko-kr",
        }
    }

    /// Returns a flag emoji representing the language.
    pub const fn flag(&self) -> &'static str {
        use Language as L;
        match self {
            L::IndonesiaIndonesian => "\u{1F1EE}\u{1F1E9}",
            L::DenmarkDanish => "\u{1F1E9}\u{1F1F0}",
            L::GermanyGerman => "\u{1F1E9}\u{1F1EA}",
            L::GreatBritainEnglish => "\u{1F1EC}\u{1F1E7}",
            L::NorthAmericaEnglish => "\u{1F1FA}\u{1F1F8}",
            L::SpainSpanish => "\u{1F1EA}\u{1F1F8}",
            L::MexicoSpanish => "\u{1F1F2}\u{1F1FD}",
            L::CanadaFrench => "\u{1F1E8}\u{1F1E6}",
            L::FranceFrench => "\u{1F1EB}\u{1F1F7}",
            L::ItalyItalian => "\u{1F1EE}\u{1F1F9}",
            L::HungaryHungarian => "\u{1F1ED}\u{1F1FA}",
            L::NetherlandsDutch => "\u{1F1F3}\u{1F1F1}",
            L::NorwayBokmål => "\u{1F1F3}\u{1F1F4}",
            L::PolandPolish => "\u{1F1F5}\u{1F1F1}",
            L::BrazilPortuguese => "\u{1F1E7}\u{1F1F7}",
            L::PortugalProtugese => "\u{1F1F5}\u{1F1F9}",
            L::SlovakiaSlovak => "\u{1F1F8}\u{1F1F0}",
            L::FinlandFinnish => "\u{1F1EB}\u{1F1EE}",
            L::SwedenSwedish => "\u{1F1F8}\u{1F1EA}",
            L::TurkeyTurkish => "\u{1F1F9}\u{1F1F7}",
            L::CzeshRepublicCzech => "\u{1F1E8}\u{1F1FF}",
            L::GreeceGreek => "\u{1F1EC}\u{1F1F7}",
            L::BulgariaBulgarian => "\u{1F1E7}\u{1F1EC}",
            L::RussiaRussian => "\u{1F1F7}\u{1F1FA}",
            L::UkraineUkrainian => "\u{1F1FA}\u{1F1E6}",
            L::JapanJapanese => "\u{1F1EF}\u{1F1F5}",
            L::ChinaChinese => "\u{1F1E8}\u{1F1F3}",
            L::TaiwanChinese => "\u{1F1F9}\u{1F1FC}",
            L::KoreaKorean => "\u{1F1F0}\u{1F1F7}",
        }
    }

    /// Returns a slice of each language natively supported by Minecraft.
    pub const fn vanilla() -> &'static [Self] {
        &[
            Self::IndonesiaIndonesian,
            Self::DenmarkDanish,
            Self::GermanyGerman,
            Self::GreatBritainEnglish,
            Self::NorthAmericaEnglish,
            Self::SpainSpanish,
            Self::MexicoSpanish,
            Self::CanadaFrench,
            Self::FranceFrench,
            Self::ItalyItalian,
            Self::HungaryHungarian,
            Self::NetherlandsDutch,
            Self::NorwayBokmål,
            Self::PolandPolish,
            Self::BrazilPortuguese,
            Self::PortugalProtugese,
            Self::SlovakiaSlovak,
            Self::FinlandFinnish,
            Self::SwedenSwedish,
            Self::TurkeyTurkish,
            Self::CzeshRepublicCzech,
            Self::GreeceGreek,
            Self::BulgariaBulgarian,
            Self::RussiaRussian,
            Self::UkraineUkrainian,
            Self::JapanJapanese,
            Self::ChinaChinese,
            Self::TaiwanChinese,
            Self::KoreaKorean,
        ]
    }
}
