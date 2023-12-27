use std::fs::{self};

use anyhow::Context;
use codes::{CodeContext, CodeDir, CodeModule, CodeSource};

pub fn parse_build(_: Option<String>) -> anyhow::Result<()> {
    let files = bind_result(fs::read_dir(".")?)?;

    let mut config = None;

    for f in files {
        if f.file_type()?.is_file() && f.file_name() == "Megu.toml" {
            config = Some(fs::read_to_string(f.path())?);
            break;
        }
    }

    let config = config.context("Megu.toml not found")?;
    let config_toml = config.parse::<toml::Table>()?;

    // TODO: add Workspace support
    // TODO; add depandencies
    if config_toml.get("module").is_some() {
        let mut data = read_dir_recursive(".")?;
        data.name = "__ROOT__".to_string();
        let root_name = {
            let module = config_toml.get("module").context("internal error")?;
            module
                .as_table()
                .context("'module' is not table")?
                .get("name")
                .context("'module.name' couldn't find")?
                .as_str()
                .context("'module.name' is not string")?
        };

        let root = CodeModule {
            name: root_name.to_string(),
            dirs: data,
        };

        // add deps

        meguc_main::megu_compile(CodeContext {
            modules: vec![root],
        })?;
    } else {
        anyhow::bail!("Megu.toml is invalid");
    }

    Ok(())
}

pub fn bind_result<T, E>(iter: impl Iterator<Item = Result<T, E>>) -> Result<Vec<T>, E> {
    let mut vec = Vec::new();
    for item in iter {
        vec.push(item?);
    }
    Ok(vec)
}

pub fn read_dir_recursive(path: &str) -> anyhow::Result<CodeDir> {
    let mut res = CodeDir::default();
    let data = bind_result(fs::read_dir(path)?)?;
    for d in data {
        if d.file_type()?.is_file()
            && get_extension(d.file_name().to_str().unwrap().to_string())
                == Some(".meg".to_string())
        {
            res.source.push(CodeSource {
                name: d.file_name().to_str().unwrap().to_string(),
                code: fs::read_to_string(d.path())?,
            })
        } else if d.file_type()?.is_dir() {
            res.dirs
                .push(read_dir_recursive(d.path().to_str().unwrap())?);
        }
    }
    Ok(res)
}

pub fn get_extension(name: String) -> Option<String> {
    let idx = name.find('.')?;
    let ext = name[idx..].to_string();
    Some(ext)
}
