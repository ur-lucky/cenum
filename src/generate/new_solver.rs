use crate::config::BuildOptions;
use crate::model::EnumDef;

use super::{enum_item_name_alias, enum_set_alias, line, string_literal, SolverEmitter};

pub struct NewSolverEmitter;

impl SolverEmitter for NewSolverEmitter {
    fn emit_enum(&self, output: &mut String, enum_def: &EnumDef, options: &BuildOptions) {
        let keyword = options.declaration_keyword();
        let item_name_alias = enum_item_name_alias(enum_def);
        let enum_set_alias = enum_set_alias(enum_def);

        line(output, 0, &format!("-- {}", enum_def.name));
        let union = enum_def
            .items
            .iter()
            .map(|item| string_literal(item))
            .collect::<Vec<_>>()
            .join(" | ");
        line(output, 0, &format!("type {item_name_alias} = {union}"));
        for item in &enum_def.items {
            let item_literal = string_literal(item);
            line(
                output,
                0,
                &format!("type __CEnum{}{}Item = {{", enum_def.name, item),
            );
            line(output, 1, &format!("Name: {item_literal},"));
            line(output, 1, "Value: number,");
            line(output, 1, &format!("EnumType: {},", enum_def.name));
            line(output, 1, "IsA: (self: any, category: string) -> boolean,");
            line(output, 0, "}");
        }
        line(output, 0, &format!("export type {} =", enum_def.name));
        for (index, item) in enum_def.items.iter().enumerate() {
            let prefix = if index == 0 { "" } else { "| " };
            line(
                output,
                1,
                &format!("{prefix}__CEnum{}{}Item", enum_def.name, item),
            );
        }
        line(output, 0, &format!("type {enum_set_alias} = {{"));
        for item in &enum_def.items {
            line(
                output,
                1,
                &format!("{item}: __CEnum{}{}Item,", enum_def.name, item),
            );
        }
        line(
            output,
            1,
            &format!("FromName: (self: any, name: {item_name_alias}) -> {}?,", enum_def.name),
        );
        line(
            output,
            1,
            &format!("FromValue: (self: any, value: number) -> {}?,", enum_def.name),
        );
        line(
            output,
            1,
            &format!("GetEnumItems: (self: any) -> {{ {} }},", enum_def.name),
        );
        line(output, 0, "}");
        output.push('\n');

        line(output, 0, &format!("{keyword} {}: {enum_set_alias}", enum_def.name));
        line(output, 0, "do");
        line(output, 1, &format!("{} = {{", enum_def.name));
        for (index, item) in enum_def.items.iter().enumerate() {
            let item_literal = string_literal(item);
            line(output, 2, &format!("{item} = table.freeze(setmetatable({{"));
            line(output, 3, &format!("Name = {item_literal},"));
            line(output, 3, &format!("Value = {},", index + 1));
            line(output, 3, &format!("EnumType = {},", enum_def.name));
            line(output, 2, "}, __CEnumEnumItemMap)),");
        }
        line(output, 1, "} :: any");
        line(output, 0, "end");
        output.push('\n');
        line(output, 0, &format!("setmetatable({}, __CEnumEnumSetMetatable)", enum_def.name));
        line(output, 0, &format!("table.freeze({})", enum_def.name));
    }
}
