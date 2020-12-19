use glob::glob;
use std::fs::{File, canonicalize};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::io::Read;


use super::aimlparser;
use super::aiml;
// use std::io::ErrorKind;


fn load_aiml(filename: PathBuf, root : &mut aiml::AIML) -> Result<(), Box<dyn Error>> {
    debug!("About to load file: {:?}", filename);
    let mut f = File::open(filename)?;
    // let f = match f {
    //     Ok(file) => { file },
    //     Err(e)  => {
    //         match e.kind() {
    //             ErrorKind::NotFound => {error!("File not found: {}", e); return Err(e)},
    //             other_error => {error!("Error opening file: {}", e); return Err(e)},
    //         };
    //     }
    // }

    info!("{:?}", f);

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    aimlparser::parse(&content, root);

    debug!("{}", content);

    return Ok(());
}

pub fn load_aiml_set(aimlset: &str) -> aiml::AIML{
    // debug!("current directory: {:?}", env::current_dir().unwrap());
    // debug!("aimlset path: {:?}", env::current_dir().unwrap().join(Path::new(aimlset)));
    // debug!("abs {:?}", fs::canonicalize(Path::new(aimlset)));

    // if Path::new(aimlset).exists() {
    //     debug!("aimlset path exists: {:?}", Path::new(aimlset).to_str());
    // } else {
    //     error!("aimlset path doesn't exist");
    //     return false;
    // }
    let mut aiml = aiml::AIML::new();
    let path = match canonicalize(Path::new(aimlset)) {
        Ok(path) => {
            debug!("aimlset path exists: {:?}", path);
            path
        },
        Err(e) => {
            error!("probably aimlset path doesn't exists: {}", e);
            return aiml;
        }
    };


    for file in glob(&format!("{}/**/*.aiml", path.to_str().unwrap())).expect("Failed to read glob pattern") {
        match file {
            Ok(filepath) => {
                match load_aiml(filepath, &mut aiml) {
                    Ok(_) => {},
                    Err(e) => {
                        error!("Something went wrong: {}", e);
                        return aiml;
                    }
                }
            },
            Err(e) => {
                error!("Error in reading glob: {}", e);
                return aiml;
            },
        }
    }
    return aiml;
}

#[test]
fn it_works() {
    assert_eq!(2+2, 4);
}

#[test]
fn test_loading_aiml_sets() {
    assert_eq!(load_aiml_set("data"), true);
}