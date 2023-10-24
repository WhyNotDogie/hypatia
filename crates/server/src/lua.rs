use std::{sync::Arc, fmt::{Display, Debug}};

use common::fs::Node;
use log::error;
use mlua::{Lua, Value};

pub fn new_lua(id: u128) -> Lua {
    let l = Lua::new();
    let globs = l.globals();
    globs.set("require", l.create_async_function(async move |l, path: String| -> Result<Value, mlua::Error> {
        let v: Vec<&str> = path.split("/").filter(|v| -> bool {v != &""}).collect();
        let mut p = &crate::get_info_from_game_id(id).await.map_err(|e| {
            error!("database connection error (in lua require definition)");
            struct E;
            impl std::error::Error for E {}
            impl Debug for E {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    <Self as Display>::fmt(self, f)
                }
            }
            impl Display for E {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "Database connection error")
                }
            }
            mlua::Error::ExternalError(Arc::new(E))
        })?.lua;
        let mut indx = 0;
        for part in &v {
            let c = &p.nodes[*part];
            match c {
                Node::Folder(f) => {
                    p = f;
                }
                Node::File(f) => {
                    if indx == v.len()-1 {
                        let val: Value = l.load(f)
                            .set_name(&path)
                            .call_async(()).await?;
                        return Ok(val)

                    } else {
                        struct E(String);
                        impl std::error::Error for E {}
                        impl Debug for E {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                <Self as Display>::fmt(self, f)
                            }
                        }
                        impl Display for E {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                write!(f, "File not found: {}", self.0)
                            }
                        }
                        return Err(mlua::Error::ExternalError(Arc::new(E(path))))
                    }
                }
            }
            indx += 1;
        }
        struct E;
        impl std::error::Error for E {}
        impl Debug for E {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                <Self as Display>::fmt(self, f)
            }
        }
        impl Display for E {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Expected a valid path")
            }
        }
        Err(mlua::Error::ExternalError(Arc::new(E)))
    }).unwrap()).unwrap();

    drop(globs);

    l
}