use std::{fs, path::PathBuf};

use anyhow::Context;
use codes::{CodeContext, CodeDir, CodeModule, CodeSource};
use meguc_main::megu_compile;
use utils::{bind_result, SccMap};

pub fn parse_build(_: Option<String>) -> anyhow::Result<()> {
    // TODO: add Workspace support
    let mut deps_edge = SccMap::new();
    let ctx = CodeContext {
        modules: load_module(&PathBuf::from("."), &mut deps_edge)?,
        deps: deps_edge,
    };

    megu_compile(ctx)?;

    Ok(())
}

pub fn load_module(
    path: &PathBuf,
    deps_edge: &mut SccMap<String>,
) -> anyhow::Result<Vec<CodeModule>> {
    let files = bind_result(fs::read_dir(path)?)?;

    let mut config = None;

    for f in files {
        if f.file_type()?.is_file() && f.file_name() == "Megu.toml" {
            config = Some(fs::read_to_string(f.path())?);
            break;
        }
    }

    let config = config.context("Megu.toml not found")?;
    let config = config.parse::<toml::Table>()?;

    if config.get("module").is_some() {
        let mut data = read_dir_recursive(path)?;
        data.name = "__ROOT__".to_string();
        let name = {
            let module = config.get("module").context("internal error")?;
            module
                .as_table()
                .context("'module' is not table")?
                .get("name")
                .context("'module.name' couldn't find")?
                .as_str()
                .context("'module.name' is not string")?
        };

        let mut deps_vec = Vec::new();

        if config.get("dep").is_some() {
            let deps = config
                .get("dep")
                .context("internal error")?
                .as_array()
                .context("'dep' is not array")?;

            for dep in deps {
                let dep = dep.as_table().context("'dep' is not table")?;

                // let name = dep
                //     .get("name")
                //     .context("'dep.name' couldn't find")?
                //     .as_str()
                //     .context("'dep.name' is not string")?;

                let tpath = dep
                    .get("path")
                    .context("'dep.path' couldn't find")?
                    .as_table()
                    .context("'dep.path' is not table")?;
                {
                    let type_ = tpath
                        .get("type")
                        .context("'dep.path.type' couldn't find")?
                        .as_str()
                        .context("'dep.path.type' is not string")?;

                    // TODO: add remote dep
                    if type_ != "local" {
                        todo!("remote dep")
                    }
                }

                let dpath = tpath
                    .get("path")
                    .context("'dep.path.path' couldn't find")?
                    .as_str()
                    .context("'dep.path.path' is not string")?;

                deps_edge.add_edge((name.to_string(), dpath.to_string()));
                deps_vec.append(&mut load_module(&path.join(dpath), deps_edge)?);
            }
        }

        let cmodule = CodeModule {
            name: name.to_string(),
            dirs: data,
        };

        // add deps

        deps_vec.push(cmodule);
        Ok(deps_vec)
    } else {
        anyhow::bail!("Megu.toml is invalid: path = {:?}", path);
    }
}

pub fn read_dir_recursive(path: &PathBuf) -> anyhow::Result<CodeDir> {
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
            res.dirs.push(read_dir_recursive(&d.path())?);
        }
    }
    Ok(res)
}

pub fn get_extension(name: String) -> Option<String> {
    let idx = name.find('.')?;
    let ext = name[idx..].to_string();
    Some(ext)
}
