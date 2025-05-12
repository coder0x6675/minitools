
use std::path::Path;
use std::collections::HashMap;


type Counts = HashMap<String, i32>;


fn count_extensions(path: &Path, counts: &mut Counts) {

    let entries = std::fs::read_dir(path)
        .expect(&format!("failed to read directory {path:?}"));

    for entry in entries {

        let path = entry.unwrap().path();
        if path.is_dir() {
            count_extensions(&path, counts);
            continue
        }

        if let Some(ext) = path.extension() {
            let ext = ext.to_ascii_lowercase().into_string().unwrap();
            *counts.entry(ext).or_insert(0) += 1;
        }
        else {
            *counts.entry("NONE".to_string()).or_insert(0) += 1;
        }
    }
}


fn main() {

    let paths: Vec<String> = std::env::args()
        .skip(1)
        .collect()
        ;

    let mut counts: Counts = Default::default();
    for path in paths {
        count_extensions(Path::new(&path), &mut counts);
    }

    if let Some(count) = counts.remove("NONE") {
        println!("{count:6} NONE");
    }

    for (extension, count) in counts {
        println!("{count:6} {extension}");
    }
}


