use glob::glob;
use std::error::Error;
use std::fs::{canonicalize, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use super::aiml;
use super::parser;

fn load_aiml(filename: PathBuf, root: &mut aiml::AIML) -> Result<(), Box<dyn Error>> {
    debug!("About to load file: {:?}", filename);
    let mut f = File::open(filename)?;
    //info!("{:?}", f);
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    parser::parse(&content, root);
    return Ok(());
}

/// This will load a whole directory recursively
pub fn load_aiml_set(aimlset: &str) -> aiml::AIML {
    let mut aiml = aiml::AIML::new();
    let path = match canonicalize(Path::new(aimlset)) {
        Ok(path) => {
            debug!("aimlset path exists: {:?}", path);
            path
        }
        Err(e) => {
            error!("probably aimlset path doesn't exists: {}", e);
            return aiml;
        }
    };

    for file in
        glob(&format!("{}/**/*.aiml", path.to_str().unwrap())).expect("Failed to read glob pattern")
    {
        match file {
            Ok(filepath) => match load_aiml(filepath, &mut aiml) {
                Ok(_) => {}
                Err(e) => {
                    error!("Something went wrong: {}", e);
                    return aiml;
                }
            },
            Err(e) => {
                error!("Error in reading glob: {}", e);
                return aiml;
            }
        }
    }
    return aiml;
}

/// This will load a file
pub fn load_aiml_file(aimlfile: &str) -> aiml::AIML {
    let mut aiml = aiml::AIML::new();
    let path = match canonicalize(Path::new(aimlfile)) {
        Ok(path) => {
            debug!("aimlset path exists: {:?}", path);
            path
        }
        Err(e) => {
            error!("probably aimlset path doesn't exists: {}", e);
            return aiml;
        }
    };

    match load_aiml(path, &mut aiml) {
        Ok(_) => {}
        Err(e) => {
            error!("Something went wrong: {}", e);
        }
    }

    return aiml;
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
