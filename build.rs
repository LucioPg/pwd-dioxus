use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Regex-free extraction of CSS-class-like tokens from Rust string literals.
/// Matches sequences like: btn, toggle-primary, text-error/500, focus:ring-2, z-[100]
fn extract_classes_from_rs(source: &str) -> HashSet<String> {
    let mut classes = HashSet::new();
    let mut in_string = false;
    let mut current = String::new();

    for ch in source.chars() {
        match ch {
            '"' if !in_string => {
                in_string = true;
                current.clear();
            }
            '"' if in_string => {
                in_string = false;
                if looks_like_css_class(&current) {
                    classes.insert(current.clone());
                }
                current.clear();
            }
            _ if in_string => {
                current.push(ch);
            }
            _ => {}
        }
    }

    classes
}

/// Heuristic: does this string look like a CSS/Tailwind/DaisyUI class name?
fn looks_like_css_class(s: &str) -> bool {
    if s.is_empty() || s.len() > 80 {
        return false;
    }

    // Must contain at least one letter
    if !s.chars().any(|c| c.is_ascii_alphabetic()) {
        return false;
    }

    // Filter out obvious non-classes
    if s.contains("://") || s.contains('@') || s.contains("mailto:") {
        return false;
    }

    // Filter out file paths
    if s.contains('\\') || (s.contains('/') && s.contains('.')) {
        return false;
    }

    // Filter out Rust identifiers that are clearly not CSS (long camelCase with no hyphens)
    if s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') && s.len() > 30 {
        return false;
    }

    true
}

fn main() {
    println!("cargo:rerun-if-changed=src");

    let src_dir = Path::new("src");
    let output_path = Path::new("assets/tailwind-safelist.txt");

    let mut all_classes = HashSet::new();

    // Scan all .rs files in src/
    if let Ok(entries) = fs::read_dir(src_dir) {
        collect_rs_files(src_dir, &mut all_classes);
    }

    // Also scan assets/components.css for @apply references
    let components_css = Path::new("assets/components.css");
    if components_css.exists() {
        if let Ok(content) = fs::read_to_string(components_css) {
            for token in content.split_whitespace() {
                let clean = token.trim_matches(|c: char| !c.is_ascii_alphanumeric()
                    && !matches!(c, '-' | '_' | '/' | ':' | '[' | ']' | '#' | '.' | '!' | '(' | ')'));
                if looks_like_css_class(clean) {
                    all_classes.insert(clean.to_string());
                }
            }
        }
    }

    let mut classes: Vec<String> = all_classes.into_iter().collect();
    classes.sort();

    let content: String = classes
        .iter()
        .map(|c| format!("{}\n", c))
        .collect();

    let old_content = fs::read_to_string(output_path).unwrap_or_default();

    if content != old_content {
        if let Some(parent) = output_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        fs::write(output_path, &content).expect("Failed to write tailwind-safelist.txt");
        println!("cargo:warning=pwd-dioxus: generated tailwind-safelist.txt with {} classes", classes.len());
    }
}

fn collect_rs_files(dir: &Path, classes: &mut HashSet<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_rs_files(&path, classes);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    classes.extend(extract_classes_from_rs(&content));
                }
            }
        }
    }
}
