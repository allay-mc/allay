use maud::{html, DOCTYPE};

pub(crate) fn base(
    title: &str,
    script: &str,
    content: maud::PreEscaped<String>,
) -> maud::PreEscaped<String> {
    html! {
        (DOCTYPE)
        html lang="en" {
            meta charset="UTF-8";
            meta viewport="viewport" content="width=device-width, initial-scale=1";
            title {
                (title)
            }
            link href="style.css" rel="stylesheet";
            script defer src=(script) {}
        }
        body {
            (content)
        }
    }
}

pub(crate) fn download(
    project_name: &str,
    release_file_name: Option<&str>,
    debug_file_name: Option<&str>,
) -> maud::PreEscaped<String> {
    base(
        &format!("Download {}", project_name),
        "download.js",
        html! {
            h1 {
                (project_name)
            }
            @if let Some(file_name) = release_file_name {
                button #download-release-button {
                    (file_name) (" (Release)")
                }
            }
            @if let Some(file_name) = debug_file_name {
                button #download-debug-button {
                    (file_name) (" (Debug)")
                }
            }
        },
    )
}

pub(crate) fn not_found() -> maud::PreEscaped<String> {
    base(
        "Not Found",
        "notfound.js",
        html! {
            h1 {
                "404"
            }
            p {
                "Allay searched everywhere, yet didn't find what you requested."
            }
            button #home-button {
                "Home"
            }
        },
    )
}
