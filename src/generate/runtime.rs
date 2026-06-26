use crate::config::BuildOptions;

use super::line;

pub fn emit_runtime(output: &mut String, options: &BuildOptions) {
    let keyword = options.declaration_keyword();

    output.push_str("-- Runtime Types\n");
    output.push_str(
        r#"type __CEnumEnumItem = {
	Name: string,
	Value: number,
	EnumType: any,
	IsA: (self: any, category: string) -> boolean,
}

type __CEnumEnumSet = {
	FromName: (self: any, name: string) -> __CEnumEnumItem?,
	FromValue: (self: any, value: number) -> __CEnumEnumItem?,
	GetEnumItems: (self: any) -> { __CEnumEnumItem },
}

type __CEnumEnumItemData = {
	Name: string,
	Value: number,
	EnumType: any,
	_enumTypeName: string,
}

"#,
    );

    output.push_str("-- Runtime Declarations\n");
    line(output, 0, &format!("{keyword} __CEnumEnumItemMap = {{}}"));
    output.push_str(
        r#"__CEnumEnumItemMap.__index = __CEnumEnumItemMap
__CEnumEnumItemMap.__newindex = function(_, key: any)
	error(`cannot assign enum item property {tostring(key)}`, 2)
end
__CEnumEnumItemMap.__tostring = function(self: __CEnumEnumItemData): string
	return `{self._enumTypeName}.{self.Name}`
end
function __CEnumEnumItemMap.IsA(self: __CEnumEnumItemData, category: string): boolean
	return self._enumTypeName == category
end
table.freeze(__CEnumEnumItemMap)

"#,
    );

    line(output, 0, &format!("{keyword} __CEnumEnumSetMethods = {{}}"));
    output.push_str(
        r#"__CEnumEnumSetMethods.FromName = function(self: { [string]: __CEnumEnumItem }, name: string): __CEnumEnumItem?
"#,
    );
    line(output, 1, &format!("{keyword} enumItem = self[name]"));
    output.push_str(
        r#"	if enumItem == nil then
		return nil
	end

	return enumItem
end
__CEnumEnumSetMethods.FromValue = function(self: { [string]: __CEnumEnumItem }, value: number): __CEnumEnumItem?
	for _, enumItem in self do
		if enumItem.Value == value then
			return enumItem
		end
	end

	return nil
end
__CEnumEnumSetMethods.GetEnumItems = function(self: { [string]: __CEnumEnumItem }): { __CEnumEnumItem }
	local enumItems: { __CEnumEnumItem } = {}

	for _, enumItem in self do
		enumItems[enumItem.Value] = enumItem
	end

	return enumItems
end
table.freeze(__CEnumEnumSetMethods)

"#,
    );

    line(
        output,
        0,
        &format!("{keyword} __CEnumEnumSetMetatable = table.freeze({{"),
    );
    output.push_str(
        r#"	__index = __CEnumEnumSetMethods,
	__newindex = function(_, key: any)
		error(`cannot assign enum member {tostring(key)}`, 2)
	end,
})

"#,
    );

    line(
        output,
        0,
        &format!(
            "{keyword} function __CEnumNewEnumEntry(name: string, value: number, enumTypeName: string, enumSet: any): __CEnumEnumItem"
        ),
    );
    output.push_str(
        r#"	local data: __CEnumEnumItemData = {
		Name = name,
		Value = value,
		EnumType = enumSet,
		_enumTypeName = enumTypeName,
	}

	return table.freeze(setmetatable(data, __CEnumEnumItemMap)) :: any
end

"#,
    );

    line(
        output,
        0,
        &format!("{keyword} function __CEnumNew(enumTypeName: string, items: {{ string }}): __CEnumEnumSet"),
    );
    output.push_str(
        r#"	assert(typeof(items) == "table", "second argument must be a table")

	local itemSet: { [string]: boolean } = {}
	for _, name in items do
		assert(itemSet[name] == nil, "enum items must be unique")
		itemSet[name] = true
	end

	local result: { [string]: __CEnumEnumItem } = {}

	for value, name in items do
		assert((__CEnumEnumSetMethods :: any)[name] == nil, `enum item name {name} is reserved`)
		result[name] = __CEnumNewEnumEntry(name, value, enumTypeName, result)
	end

	return table.freeze(setmetatable(result, __CEnumEnumSetMetatable)) :: any
end

"#,
    );
}
