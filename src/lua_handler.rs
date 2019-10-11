use std::sync::{Arc, Mutex};

use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

use crate::drawer::Drawer;
use crate::editor::Editor;

pub trait LuaHandler {
    fn exec_cmd(&mut self, cmd: String, args: Vec<String>) -> Result<()>;
}

impl LuaHandler for Editor {
    fn exec_cmd(&mut self, cmd: String, args: Vec<String>) -> Result<()> {
        let lua = Lua::new();

        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();

            /*let editor_table = lua_ctx.create_table()?;
            editor_table.set("width", self.width)?;
            editor_table.set("height", self.height)?;
            editor_table.set("running", self.running)?;
            globals.set("editor", editor_table)?;*/

            lua_ctx.scope(|scope| {
                let editor = scope.create_function_mut(|_, ()| {
                    Ok(self)
                })?;

                lua_ctx.load("write(\"test\")").exec()?;

                Ok(())
            })?;

            lua_ctx.load("editor:write(\"test\")").exec().expect("Error"); // TODO replace expect with ?

            Ok(())
        })
    }
}

impl UserData for &mut Editor {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("write", |_, this, s: String| {
            this.buffer.push(s);
            Ok(())
        });
    }
}
