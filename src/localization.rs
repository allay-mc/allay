use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

/// File extension for Minecraft language files.
pub const LANGUAGE_FILE_EXTENSION: &'static str = "lang";

/// A value mapped to languages.
pub type Localized<T> = HashMap<Language, T>;

/// A value optionally mapped to languages.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub enum OptionallyLocalized<T> {
    Localized(Localized<T>),
    Unlocalized(T),
}

/// A group of languages used for fallbacks.
pub type LanguageGroup = Vec<Language>;

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct LanguageGroups(pub Vec<LanguageGroup>); // TODO: maybe use std::collections::HashSet

impl Default for LanguageGroups {
    fn default() -> Self {
        Self(vec![
            vec![Language::IndonesiaIndonesian],
            vec![Language::DenmarkDanish],
            vec![Language::GermanyGerman],
            vec![Language::GreatBritainEnglish, Language::NorthAmericaEnglish],
            vec![Language::SpainSpanish, Language::MexicoSpanish],
            vec![Language::FranceFrench, Language::CanadaFrench],
            vec![Language::ItalyItalian],
            vec![Language::HungaryHungarian],
            vec![Language::NetherlandsDutch],
            vec![Language::NorwayBokmål],
            vec![Language::PolandPolish],
            vec![Language::PortugalProtugese, Language::BrazilPortuguese],
            vec![Language::SlovakiaSlovak],
            vec![Language::FinlandFinnish],
            vec![Language::SwedenSwedish],
            vec![Language::TurkeyTurkish],
            vec![Language::CzeshRepublicCzech],
            vec![Language::GreeceGreek],
            vec![Language::BulgariaBulgarian],
            vec![Language::RussiaRussian],
            vec![Language::UkraineUkrainian],
            vec![Language::JapanJapanese],
            vec![Language::ChinaChinese, Language::TaiwanChinese],
            vec![Language::KoreaKorean],
        ])
    }
}

impl LanguageGroups {
    /// Returns the language group which contains `language`.
    pub fn group_of(&self, language: &Language) -> Option<&LanguageGroup> {
        for group in &self.0 {
            if group.contains(language) {
                return Some(group);
            }
        }
        None
    }

    /// Adds a language to it's own group if it is not present in any group yet.
    pub fn with_language(&mut self, language: Language) -> &mut Self {
        if self.group_of(&language).is_none() {
            self.0.push(vec![language]);
        }
        self
    }

    /// Returns the most fitting language.
    ///
    /// # Parameters
    ///
    /// * `target` - The language you try to get.
    /// * `given` - The languages that are present.
    /// * `fallback`
    ///
    /// # Algorithm
    ///
    /// 1. Does `given` contain `target`? If yes, return it.
    /// 2. Is any language in `given` in the same language group as `target`? If yes, return it.
    /// 3. Returns `fallback` if it's in `given`.
    /// 4. Is any language in `given` in the same language group as `fallback`? If yes, return it.
    /// 5. Returns the first language of `given`.
    /// 6. Return [`None`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use allay::localization::{Language, LanguageGroup, LanguageGroups};
    /// #
    /// assert_eq!(
    ///     LanguageGroups::default().best_language(
    ///         &Language::SpainSpanish,
    ///         &[&Language::SpainSpanish, &Language::MexicoSpanish],
    ///         &Language::NorthAmericaEnglish,
    ///     ),
    ///     Some(&Language::SpainSpanish),
    /// );
    ///
    /// assert_eq!(
    ///     LanguageGroups::default().best_language(
    ///         &Language::NorthAmericaEnglish,
    ///         &[&Language::SpainSpanish, &Language::NorthAmericaEnglish, &Language::MexicoSpanish],
    ///         &Language::NorthAmericaEnglish,
    ///     ),
    ///     Some(&Language::NorthAmericaEnglish),
    /// );
    ///
    /// assert_eq!(
    ///     LanguageGroups::default().best_language(
    ///         &Language::SpainSpanish,
    ///         &[&Language::MexicoSpanish],
    ///         &Language::NorthAmericaEnglish,
    ///     ),
    ///     Some(&Language::MexicoSpanish),
    /// );
    ///
    /// assert_eq!(
    ///     LanguageGroups::default().best_language(
    ///         &Language::SpainSpanish,
    ///         &[],
    ///         &Language::NorthAmericaEnglish,
    ///     ),
    ///     None,
    /// );
    /// ```
    pub fn best_language<'a>(
        &'a self,
        target: &'a Language,
        given: &[&'a Language],
        fallback: &'a Language,
    ) -> Option<&'a Language> {
        // Step 1
        if given.contains(&target) {
            return Some(target);
        }

        // Step 2
        for lang in given {
            match self.group_of(&lang) {
                Some(group) if self.group_of(target).is_some_and(|g| g == group) => {
                    return Some(lang);
                }
                _ => {}
            };
        }

        // Step 3
        if given.contains(&fallback) {
            return Some(fallback);
        }

        // Step 4
        for lang in given {
            match self.group_of(&lang) {
                Some(group) if self.group_of(fallback).is_some_and(|g| g == group) => {
                    return Some(lang);
                }
                _ => {}
            };
        }

        // Step 5 & 6
        if given.is_empty() {
            None
        } else {
            given.first().copied()
        }
    }
}

/// Generates a `language.json` file which is just an array of all language IDs.
pub fn generate_language_json(dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let langs: Vec<String> = dir
        .read_dir()?
        .filter_map(Result::ok)
        .filter(|d| {
            d.path()
                .extension()
                .is_some_and(|ext| ext == LANGUAGE_FILE_EXTENSION)
        })
        .map(|f| match f.path().file_stem() {
            Some(stem) => match stem.to_str() {
                Some(stem) => {
                    if Language::from_file_id(stem).is_some() {
                        Some(stem.to_string())
                    } else {
                        None
                    }
                }
                None => {
                    log::warn!(
                        "{} appears to have an non-unicode file stem",
                        f.path().display()
                    );
                    None
                }
            },
            None => {
                log::warn!("{} appears to have no file stem", f.path().display());
                None
            }
        })
        .filter_map(|x| x)
        .collect();
    let json: serde_json::Value = serde_json::from_value(langs.into())?;
    let path = dir.join("languages.json");
    fs::write(path, json.to_string())?;
    Ok(())
}

/// Updates language files.
///
/// # Parameters
///
/// * `dir` - The root dir of the language files (e.g. `BP/texts/`).
/// * `groups` - The language groups configured for the project.
/// * `fallback` - The language to ultimately fall back to.
/// * `data` - Key mapped to the localized value appended to the language files.
pub fn update_language_files(
    dir: &PathBuf,
    groups: &LanguageGroups,
    fallback: &Language,
    data: HashMap<String, Localized<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("translations: {:#?}", data);
    for (key, target) in &data {
        let mut covered: Vec<&Language> = Vec::new();
        for (lang, translation) in target {
            append_language_file(dir, &lang, &key, &translation)?;
            covered.push(&lang);
        }

        // cover all remaining languages with their fallback if present
        let remaining_languages = groups.0.iter().flatten().filter(|l| !covered.contains(l));
        for remaining in remaining_languages {
            if let Some(lang_with_translation) = groups.best_language(remaining, &covered, fallback)
            {
                let translation = target.get(lang_with_translation).expect(
                    format!(
                        "{} should provide a translation for {}",
                        lang_with_translation, key
                    )
                    .as_str(),
                );
                append_language_file(dir, remaining, &key, &translation)?;
            } else {
                // TODO: this is probably unreachable as `name` and `description` are always at least set
                //       to some language
                log::warn!(
                    "Missing translation for {}; itself and none of it's fallbacks provide '{}'",
                    remaining,
                    key
                );
            }
        }
    }
    Ok(())
}

/// Appends a key-value pair to a language file.
fn append_language_file(
    dir: &PathBuf,
    lang: &Language,
    key: &str,
    translation: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = dir
        .join(lang.file_id())
        .with_extension(LANGUAGE_FILE_EXTENSION);
    log::debug!("Appending {}", path.display());
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    writeln!(file, "{key}={translation}\t## @generated")?;
    Ok(())
}

mod by_id {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(d: D) -> Result<(String, Option<String>), D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;
        let s = s.to_lowercase().replace('_', "-");
        Ok((s, None))
    }
}

// FIXME: always serialized to Other (which is fine but makes all other options redundant and may be an issue
//        in the future as only the vanilla languages have a name which is nicer for printing).
/// Languages that can be used for translation.
#[derive(Clone, Debug, Default, Eq, Hash, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
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

    /// A language not natively supported by Minecraft with its (file) ID and name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use allay::localization::Language;
    ///
    /// Language::Other("at-at".to_string(), Some("Austrian German".to_string()));
    /// ```
    // FIXME: JSON schema, see also: https://graham.cool/schemars/deriving/attributes/#with
    #[serde(with = "by_id")]
    #[cfg_attr(feature = "config-schema", schemars(with = "String"))]
    Other(String, Option<String>),
}

impl Language {
    /// Ascii-compatible alias for [`Language::NorwayBokmål`].
    #[allow(non_upper_case_globals)]
    pub const NorwayBokmal: Self = Self::NorwayBokmål;

    /// Returns a slice of each language natively supported by Minecraft.
    pub fn vanilla() -> &'static [Self] {
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

    /// Returns the language that matches the pair.
    pub fn from_pair(pair: (&str, &str)) -> Self {
        match pair {
            ("id", "id") => Self::IndonesiaIndonesian,
            ("da", "dk") => Self::DenmarkDanish,
            ("de", "de") => Self::GermanyGerman,
            ("en", "gb") => Self::GreatBritainEnglish,
            ("en", "us") => Self::NorthAmericaEnglish,
            ("es", "es") => Self::SpainSpanish,
            ("es", "mx") => Self::MexicoSpanish,
            ("fr", "ca") => Self::CanadaFrench,
            ("fr", "fr") => Self::FranceFrench,
            ("it", "it") => Self::ItalyItalian,
            ("hu", "hu") => Self::HungaryHungarian,
            ("nl", "nl") => Self::NetherlandsDutch,
            ("nb", "no") => Self::NorwayBokmål,
            ("pl", "pl") => Self::PolandPolish,
            ("pt", "br") => Self::BrazilPortuguese,
            ("pt", "pt") => Self::PortugalProtugese,
            ("sk", "sk") => Self::SlovakiaSlovak,
            ("fi", "fi") => Self::FinlandFinnish,
            ("sv", "se") => Self::SwedenSwedish,
            ("tr", "tr") => Self::TurkeyTurkish,
            ("cs", "cz") => Self::CzeshRepublicCzech,
            ("el", "gr") => Self::GreeceGreek,
            ("bg", "bg") => Self::BulgariaBulgarian,
            ("ru", "ru") => Self::RussiaRussian,
            ("uk", "ua") => Self::UkraineUkrainian,
            ("ja", "jp") => Self::JapanJapanese,
            ("zh", "cn") => Self::ChinaChinese,
            ("zh", "tw") => Self::TaiwanChinese,
            ("ko", "kr") => Self::KoreaKorean,
            (country, lang) => Self::Other(format!("{}-{}", country, lang), None),
        }
    }

    /// Returns the language that matches the ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use allay::localization::Language;
    /// #
    /// assert_eq!(Some(Language::GermanyGerman), Language::from_id("de-de"));
    /// assert_eq!(Some(Language::Other("at-at".to_string(), None)), Language::from_id("at-at"));
    /// assert_eq!(None, Language::from_id("de_DE"));
    /// ```
    pub fn from_id(id: &str) -> Option<Self> {
        Some(Self::from_pair(id.split_once('-')?))
    }

    /// Returns the language that matches the file ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use allay::localization::Language;
    /// #
    /// assert_eq!(Some(Language::GermanyGerman), Language::from_file_id("de_DE"));
    /// assert_eq!(Some(Language::Other("at-at".to_string(), None)), Language::from_file_id("at_AT"));
    /// assert_eq!(None, Language::from_file_id("de-DE"));
    /// ```
    pub fn from_file_id(id: &str) -> Option<Self> {
        let pair = id.split_once('_')?;
        let lang = pair.1.to_lowercase();
        let pair = (pair.0, lang.as_str());
        Some(Self::from_pair(pair))
    }

    /// Returns the file ID of the lanuage (e.g. `id_ID` for Indonesian).
    pub fn file_id(&self) -> String {
        use Language as L;
        match self {
            L::IndonesiaIndonesian => String::from("id_ID"),
            L::DenmarkDanish => String::from("da_DK"),
            L::GermanyGerman => String::from("de_DE"),
            L::GreatBritainEnglish => String::from("en_GB"),
            L::NorthAmericaEnglish => String::from("en_US"),
            L::SpainSpanish => String::from("es_ES"),
            L::MexicoSpanish => String::from("es_MX"),
            L::CanadaFrench => String::from("fr_CA"),
            L::FranceFrench => String::from("fr_FR"),
            L::ItalyItalian => String::from("it_IT"),
            L::HungaryHungarian => String::from("hu_HU"),
            L::NetherlandsDutch => String::from("nl_NL"),
            L::NorwayBokmål => String::from("nb_NO"),
            L::PolandPolish => String::from("pl_PL"),
            L::BrazilPortuguese => String::from("pt_BR"),
            L::PortugalProtugese => String::from("pt_PT"),
            L::SlovakiaSlovak => String::from("sk_SK"),
            L::FinlandFinnish => String::from("fi_FI"),
            L::SwedenSwedish => String::from("sv_SE"),
            L::TurkeyTurkish => String::from("tr_TR"),
            L::CzeshRepublicCzech => String::from("cs_CZ"),
            L::GreeceGreek => String::from("el_GR"),
            L::BulgariaBulgarian => String::from("bg_BG"),
            L::RussiaRussian => String::from("ru_RU"),
            L::UkraineUkrainian => String::from("uk_UA"),
            L::JapanJapanese => String::from("ja_JP"),
            L::ChinaChinese => String::from("zh_CN"),
            L::TaiwanChinese => String::from("zh_TW"),
            L::KoreaKorean => String::from("ko_KR"),
            L::Other(id, _name) => {
                let pair = id
                    .split_once('-')
                    .expect(format!("language has invalid id: {}", id).as_str());
                vec![pair.0, "_", pair.1.to_uppercase().as_str()].join("")
            }
        }
    }
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.file_id() == other.file_id()
    }
}

impl PartialEq<&str> for Language {
    fn eq(&self, other: &&str) -> bool {
        &self.file_id() == other
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Language as L;
        write!(
            f,
            "{}",
            match self {
                L::IndonesiaIndonesian => "Indonesian".to_string(),
                L::DenmarkDanish => "Danish".to_string(),
                L::GermanyGerman => "German".to_string(),
                L::GreatBritainEnglish => "UK English".to_string(),
                L::NorthAmericaEnglish => "US English".to_string(),
                L::SpainSpanish => "Spanish".to_string(),
                L::MexicoSpanish => "Mexican Spanish".to_string(),
                L::CanadaFrench => "Canadian".to_string(),
                L::FranceFrench => "French".to_string(),
                L::ItalyItalian => "Italian".to_string(),
                L::HungaryHungarian => "Hungarian".to_string(),
                L::NetherlandsDutch => "Dutch".to_string(),
                L::NorwayBokmål => "Bokmål".to_string(),
                L::PolandPolish => "Polish".to_string(),
                L::BrazilPortuguese => "Brazilian Portuguese".to_string(),
                L::PortugalProtugese => "Portuguese".to_string(),
                L::SlovakiaSlovak => "Slovak".to_string(),
                L::FinlandFinnish => "Finnish".to_string(),
                L::SwedenSwedish => "Swedish".to_string(),
                L::TurkeyTurkish => "Turkish".to_string(),
                L::CzeshRepublicCzech => "Czech".to_string(),
                L::GreeceGreek => "Greek".to_string(),
                L::BulgariaBulgarian => "Bulgarian".to_string(),
                L::RussiaRussian => "Russian".to_string(),
                L::UkraineUkrainian => "Ukrainian".to_string(),
                L::JapanJapanese => "Japanese".to_string(),
                L::ChinaChinese => "Chinese (Simplified)".to_string(),
                L::TaiwanChinese => "Chinese (Traditional)".to_string(),
                L::KoreaKorean => "Korean".to_string(),
                L::Other(id, name) => match name {
                    Some(name) => format!("{name} ({id})"),
                    None => format!("Language with id {id}"),
                },
            }
        )
    }
}
