use std::collections::HashMap;

pub(crate) fn groups() -> Vec<Vec<&'static str>> {
    vec![
        vec!["id-id"],
        vec!["da-dk"],
        vec!["de-de"],
        vec!["en-gb", "en-us"],
        vec!["es-es", "es-mx"],
        vec!["fr-ca", "fr-fr"],
        vec!["it-it"],
        vec!["hu-hu"],
        vec!["nl-nl"],
        vec!["nb-no"],
        vec!["pl-pl"],
        vec!["pt-br", "pt-pt"],
        vec!["sk-sk"],
        vec!["fi-fi"],
        vec!["sv-se"],
        vec!["tr-tr"],
        vec!["cs-cz"],
        vec!["el-gr"],
        vec!["bg-bg"],
        vec!["ru-ru"],
        vec!["uk-ua"],
        vec!["jp-jp"],
        vec!["zh-cn", "zh-tw"],
        vec!["ko-kr"],
    ]
}

pub(crate) fn group_of(language: &str) -> Vec<&'static str> {
    for g in groups() {
        for langs in &g {
            if langs.contains(language) {
                return g;
            }
        }
    }
    panic!("language {} does not exist", language);
}

pub(crate) fn languages() -> Vec<&'static str> {
    groups().into_iter().flatten().collect()
}

/// Returns the translation by
///
/// 1. trying to get the translation mapped to `preferred`
/// 2. trying to get the first translation mapped to `preffered`s language group
/// 3. trying to get the translation mapped to `primary`
/// 4. trying to get the fisrt translation mapped to `primary`s language group
/// 5. trying to get the first translation
/// 6. panicing
pub(crate) fn best_translation<'a>(
    source: &'a HashMap<String, String>,
    preferred: &str,
    primary: &str,
) -> &'a String {
    if let Some(val) = source.get(preferred) {
        return val;
    }
    for similar in group_of(preferred) {
        if let Some(val) = source.get(similar) {
            return val;
        }
    }
    if let Some(val) = source.get(primary) {
        return val;
    }
    for similar in group_of(primary) {
        if let Some(val) = source.get(primary) {
            return val;
        }
    }
    source.values().next().expect("no language provided")
}

/// Converts the Allay localization notation into the Minecraft localization
/// notation.
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///     "en_US",
///     allay::configuration::localization::allay_to_minecraft("en-us")
/// );
/// ```
pub(crate) fn allay_to_minecraft(language: &str) -> String {
    let lang = &language[0..2];
    let country = &language[3..5];
    let mut res = String::new();
    res.push_str(lang);
    res.push('_');
    res.push_str(&country.to_uppercase());
    res
}

/// Converts the Minecraft localization notation into the Minecraft localization
/// notation.
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///     "en-us",
///     allay::configuration::localization::allay_to_minecraft("en_US")
/// );
/// ```
pub(crate) fn minecraft_to_allay(language: &str) -> String {
    let lang = &language[0..2];
    let country = &language[3..5];
    let mut res = String::new();
    res.push_str(lang);
    res.push('-');
    res.push_str(&country.to_lowercase());
    res
}
