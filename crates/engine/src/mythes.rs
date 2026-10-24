//! Thesaurus files in the MyThes format (`th_it_IT_v2.dat`), read from the dictionary folders:
//! headword lines `word|N`, each followed by `N` sense lines `(part of speech)|synonym|synonym…`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

/// Lower-case headword → the synonyms of each of its senses.
pub type MyThes = HashMap<String, Vec<Vec<String>>>;

/// The most headwords read from one file, so a damaged or huge file can't use all the memory.
const MAX_HEADWORDS: usize = 600_000;

pub fn parse(bytes: &[u8]) -> MyThes {
    let head_end = bytes.iter().position(|b| *b == b'\n').unwrap_or(bytes.len());
    let latin1 = String::from_utf8_lossy(bytes.get(..head_end).unwrap_or(&[])).to_ascii_uppercase().contains("8859-1");
    let text = if latin1 { bytes.iter().map(|b| char::from(*b)).collect::<String>() } else { String::from_utf8_lossy(bytes).into_owned() };
    let mut map = MyThes::new();
    let mut lines = text.lines().skip(1);
    while let Some(line) = lines.next() {
        let Some((head, n)) = line.rsplit_once('|') else { continue };
        let Ok(n) = n.trim().parse::<usize>() else { continue };
        let head = head.trim().to_lowercase();
        let mut senses = Vec::new();
        for _ in 0..n.min(64) {
            let Some(sense) = lines.next() else { break };
            let synonyms: Vec<String> = sense.split('|').skip(1).map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect();
            if !synonyms.is_empty() {
                senses.push(synonyms);
            }
        }
        if !senses.is_empty() && !head.is_empty() && map.len() < MAX_HEADWORDS {
            map.entry(head).or_default().extend(senses);
        }
    }
    map
}

/// The thesaurus file for `lang` in `dirs`: `th_it_IT_v2.dat`, `th_it_IT.dat`, then any of the language.
pub fn find(dirs: &[PathBuf], lang: &str) -> Option<PathBuf> {
    let full = lang.replace('-', "_");
    if full.is_empty() || !full.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') {
        return None;
    }
    let base = full.split('_').next().unwrap_or("").to_string();
    for d in dirs {
        for stem in [&full, &base] {
            for name in [format!("th_{stem}_v2.dat"), format!("th_{stem}.dat")] {
                let p = d.join(name);
                if p.is_file() {
                    return Some(p);
                }
            }
        }
        if let Ok(rd) = std::fs::read_dir(d) {
            let mut names: Vec<String> = rd
                .filter_map(|e| e.ok()?.file_name().into_string().ok())
                .filter(|n| n.starts_with(&format!("th_{base}_")) && n.ends_with(".dat"))
                .collect();
            names.sort();
            if let Some(n) = names.first() {
                return Some(Path::new(d).join(n));
            }
        }
    }
    None
}

/// The thesaurus installed for the default language, read once.
fn installed() -> Option<Arc<MyThes>> {
    static CACHE: OnceLock<Mutex<HashMap<String, Option<Arc<MyThes>>>>> = OnceLock::new();
    let lang = crate::hunspell::default_language();
    let cache = CACHE.get_or_init(Default::default);
    if let Ok(c) = cache.lock()
        && let Some(t) = c.get(&lang)
    {
        return t.clone();
    }
    let loaded = find(&crate::hunspell::search_dirs(), &lang).and_then(|p| std::fs::read(p).ok()).map(|b| Arc::new(parse(&b)));
    if let Ok(mut c) = cache.lock() {
        c.insert(lang, loaded.clone());
    }
    loaded
}

/// The synonyms of `word` in the installed thesaurus (every sense, without repeats, at most `max`),
/// or `None` when no thesaurus is installed.
pub fn lookup(word: &str, max: usize) -> Option<Vec<String>> {
    let th = installed()?;
    let w = word.trim().to_lowercase();
    let mut out: Vec<String> = Vec::new();
    for sense in th.get(&w).into_iter().flatten() {
        for s in sense {
            if s.to_lowercase() != w && !out.contains(s) && out.len() < max {
                out.push(s.clone());
            }
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_mythes_file_is_read() {
        let th =
            parse("UTF-8\ncasa|2\n(nome)|abitazione|dimora|focolare\n(nome)|ditta|azienda\nstrada|1\n(nome)|via|percorso\nrotto|zz\n".as_bytes());
        assert_eq!(th.get("casa").map(Vec::len), Some(2));
        assert_eq!(th["casa"][0], ["abitazione", "dimora", "focolare"]);
        assert_eq!(th["strada"][0][0], "via");
        assert!(!th.contains_key("rotto"));
        assert!(parse(b"").is_empty());
        let latin = parse(b"ISO8859-1\nperch\xe9|1\n(cong.)|poich\xe9\n");
        assert_eq!(latin["perché"][0][0], "poiché");
    }

    #[test]
    fn a_thesaurus_is_found_by_language_in_the_folders() {
        let dir = std::env::temp_dir().join(format!("sheetcalc-th-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        assert_eq!(find(std::slice::from_ref(&dir), "it-IT"), None);
        std::fs::write(dir.join("th_it_IT_v2.dat"), b"UTF-8\n").unwrap();
        assert_eq!(find(std::slice::from_ref(&dir), "it-IT"), Some(dir.join("th_it_IT_v2.dat")));
        assert_eq!(find(std::slice::from_ref(&dir), "it"), Some(dir.join("th_it_IT_v2.dat")));
        assert_eq!(find(std::slice::from_ref(&dir), "../etc"), None);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
