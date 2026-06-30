use crate::config::BuildOptions;
use crate::model::EnumDef;

use super::{line, string_literal, SolverEmitter};

pub struct BasicEmitter;

pub fn emit_runtime(output: &mut String, options: &BuildOptions) {
    let keyword = options.declaration_keyword();

    output.push_str("-- Runtime Declarations\n");
    line(
        output,
        0,
        &format!("{keyword} function __CEnumNewBasicEnum<T>(enum: T): T"),
    );
    output.push_str(
        r#"	local inverse = {}
	local result = {}

	for key, value in pairs(enum :: any) do
		assert(inverse[value] == nil or result[key] == nil, `Duplicate enum value on {key} ({value})`)
		result[key] = value
		inverse[value] = key
	end

	return setmetatable(result, {
		__index = inverse,
		__newindex = function(_, key: any, value: any)
			error(`Attempt to assign enum value {key} = {value}`)
		end,
	}) :: any
end

"#,
    );
}

impl SolverEmitter for BasicEmitter {
    fn emit_enum(&self, output: &mut String, enum_def: &EnumDef, _options: &BuildOptions) {
        let key_alias = format!("{}Key", enum_def.name);

        line(output, 0, &format!("-- {}", enum_def.name));
        let item_name_union = enum_def
            .items
            .iter()
            .map(|item| string_literal(item))
            .collect::<Vec<_>>()
            .join(" | ");
        line(output, 0, &format!("export type {key_alias} = {item_name_union}"));
        line(output, 0, &format!("export type {} = number", enum_def.name));
        line(output, 0, &format!("local {} = __CEnumNewBasicEnum({{", enum_def.name));
        for (index, item) in enum_def.items.iter().enumerate() {
            line(output, 1, &format!("{item} = {},", index + 1));
        }
        line(output, 0, "}) :: {");
        for item in &enum_def.items {
            line(output, 1, &format!("{item}: number,"));
        }
        line(output, 1, &format!("[number]: {key_alias},"));
        line(output, 0, "}");
        output.push('\n');
    }
}
