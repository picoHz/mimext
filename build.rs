use std::{
    collections::{btree_map::Entry, BTreeMap},
    env, fs,
    path::Path,
};

include!("src/list.rs");

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("mime_to_ext.rs");

    let mut patterns = BTreeMap::<&str, Vec<String>>::new();
    for (ext, mime) in LIST {
        for m in *mime {
            let entry = patterns.entry(m);
            if matches!(entry, Entry::Vacant(_)) {
                entry.or_insert_with(|| ext.iter().map(|e| format!("\"{e}\"")).collect());
            }
        }
    }
    let patterns = patterns
        .into_iter()
        .map(|(mime, ext)| format!("\"{mime}\" => &[{}],", ext.join(",")))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(
        &dest_path,
        &format!(
            "match mime {{
            {patterns}
            _ => &[],
            }}"
        ),
    )
    .unwrap();

    let dest_path = Path::new(&out_dir).join("ext_to_mime.rs");

    let mut patterns = BTreeMap::new();
    for (ext, mime) in LIST {
        for e in *ext {
            patterns
                .entry(e)
                .or_insert_with(std::vec::Vec::new)
                .append(&mut mime.iter().map(|m| format!("\"{m}\"")).collect());
        }
    }
    let patterns = patterns
        .into_iter()
        .map(|(ext, mime)| format!("\"{ext}\" => &[{}],", mime.join(",")))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(
        dest_path,
        format!(
            "match ext {{
            {patterns}
            _ => &[],
            }}"
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=src/list.rs");
}
