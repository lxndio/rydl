use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

use crate::editor::Editor;

pub trait LuaHandler {
    fn exec_cmd(&mut self, cmd: String, args: Vec<String>) -> Result<()>;
}

impl LuaHandler for Editor {
    fn exec_cmd(&mut self, cmd: String, args: Vec<String>) -> Result<()> {
        let lua = Lua::new();

        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();

            let editor_table = lua_ctx.create_table()?;
            editor_table.set("width", self.width)?;
            editor_table.set("height", self.height)?;
            editor_table.set("running", self.running)?;
            globals.set("editor", editor_table)?;
            
            assert_eq!(lua_ctx.load(r#"editor[width]"#).eval::<i16>()?, 2);

            Ok(())
        })
    }
}