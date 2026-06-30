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

type __CEnumSerializedPrimitive = nil | boolean | number | string

type __CEnumSerializedEnumItem = {
	__syncType: "CustomEnum",
	enumName: string,
	value: number,
}

type __CEnumSerializedValue = __CEnumSerializedPrimitive | __CEnumSerializedEnumItem | __CEnumSerializedMap | { [any]: unknown }

type __CEnumSerializedMapEntry = {
	key: __CEnumSerializedValue,
	value: __CEnumSerializedValue,
}

type __CEnumSerializedMap = {
	__syncType: "Map",
	entries: { __CEnumSerializedMapEntry },
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
        &format!("{keyword} __CEnumCustomEnumSyncType = \"CustomEnum\""),
    );
    line(
        output,
        0,
        &format!("{keyword} __CEnumMapSyncType = \"Map\""),
    );
    output.push('\n');
    line(
        output,
        0,
        &format!("{keyword} function __CEnumIsPrimitiveKey(value: any): boolean"),
    );
    output.push_str(
        r#"	local valueType = type(value)
	return valueType == "number" or valueType == "string"
end

"#,
    );
    line(
        output,
        0,
        &format!(
            "{keyword} function __CEnumGetSerializedEnumItem(value: any): __CEnumSerializedEnumItem?"
        ),
    );
    output.push_str(
        r#"	if type(value) ~= "table" or getmetatable(value) ~= __CEnumEnumItemMap then
		return nil
	end

	local enumTypeName = value._enumTypeName
	local enumValue = value.Value
	if type(enumTypeName) ~= "string" or type(enumValue) ~= "number" then
		return nil
	end

	return {
		__syncType = __CEnumCustomEnumSyncType,
		enumName = enumTypeName,
		value = enumValue,
	}
end

"#,
    );
    line(
        output,
        0,
        &format!(
            "{keyword} function __CEnumSerializeValue(value: any, seen: {{ [any]: boolean }}): __CEnumSerializedValue"
        ),
    );
    output.push_str(
        r#"	local serializedEnumItem = __CEnumGetSerializedEnumItem(value)
	if serializedEnumItem ~= nil then
		return serializedEnumItem
	end

	if type(value) ~= "table" then
		return value
	end

	if seen[value] then
		error("cannot serialize cyclic table", 2)
	end

	seen[value] = true

	local serializedTable: { [any]: __CEnumSerializedValue } = {}
	local serializedEntries: { __CEnumSerializedMapEntry } = {}
	local shouldUseMap = false

	for key, entryValue in value do
		local serializedKey = __CEnumSerializeValue(key, seen)
		local serializedEntryValue = __CEnumSerializeValue(entryValue, seen)

		serializedEntries[#serializedEntries + 1] = {
			key = serializedKey,
			value = serializedEntryValue,
		}

		if serializedKey ~= key or not __CEnumIsPrimitiveKey(serializedKey) then
			shouldUseMap = true
		else
			serializedTable[serializedKey] = serializedEntryValue
		end
	end

	if serializedTable.__syncType == __CEnumCustomEnumSyncType or serializedTable.__syncType == __CEnumMapSyncType then
		shouldUseMap = true
	end

	seen[value] = nil

	if shouldUseMap then
		return {
			__syncType = __CEnumMapSyncType,
			entries = serializedEntries,
		}
	end

	return serializedTable
end

"#,
    );
    line(
        output,
        0,
        &format!("{keyword} function __CEnumSerialize(_: any, value: any): __CEnumSerializedValue"),
    );
    output.push_str(
        r#"	return __CEnumSerializeValue(value, {})
end

"#,
    );
    line(
        output,
        0,
        &format!("{keyword} function __CEnumIsSerializedEnumItem(value: {{ [any]: any }}): boolean"),
    );
    output.push_str(
        r#"	return value.__syncType == __CEnumCustomEnumSyncType
		and type(value.enumName) == "string"
		and type(value.value) == "number"
end

"#,
    );
    line(
        output,
        0,
        &format!("{keyword} function __CEnumIsSerializedMap(value: {{ [any]: any }}): boolean"),
    );
    output.push_str(
        r#"	return value.__syncType == __CEnumMapSyncType and type(value.entries) == "table"
end

"#,
    );
    line(
        output,
        0,
        &format!(
            "{keyword} function __CEnumDeserializeEnumItem(enums: {{ [string]: any }}, serializedEnumItem: __CEnumSerializedEnumItem): __CEnumEnumItem?"
        ),
    );
    output.push_str(
        r#"	local enumSet = enums[serializedEnumItem.enumName]
	if type(enumSet) ~= "table" then
		return nil
	end

	local enumItem = enumSet:FromValue(serializedEnumItem.value)
	if type(enumItem) ~= "table" or getmetatable(enumItem) ~= __CEnumEnumItemMap then
		return nil
	end

	return enumItem
end

"#,
    );
    line(
        output,
        0,
        &format!("{keyword} function __CEnumDeserializeValue(enums: {{ [string]: any }}, value: unknown): unknown"),
    );
    output.push_str(
        r#"	if type(value) ~= "table" then
		return value
	end

	local tableValue = value :: { [any]: any }

	if __CEnumIsSerializedEnumItem(tableValue) then
		return __CEnumDeserializeEnumItem(enums, tableValue :: __CEnumSerializedEnumItem)
	end

	if __CEnumIsSerializedMap(tableValue) then
		local mapValue = tableValue :: __CEnumSerializedMap
		local deserializedMap = {}

		for _, entry in mapValue.entries do
			if type(entry) == "table" then
				local deserializedKey = __CEnumDeserializeValue(enums, entry.key)
				local deserializedValue = __CEnumDeserializeValue(enums, entry.value)

				if deserializedKey ~= nil and deserializedValue ~= nil then
					deserializedMap[deserializedKey] = deserializedValue
				end
			end
		end

		return deserializedMap
	end

	local deserializedTable = {}

	for key, entryValue in tableValue do
		local deserializedKey = __CEnumDeserializeValue(enums, key)
		local deserializedValue = __CEnumDeserializeValue(enums, entryValue)

		if deserializedKey ~= nil and deserializedValue ~= nil then
			deserializedTable[deserializedKey] = deserializedValue
		end
	end

	return deserializedTable
end

"#,
    );
    line(
        output,
        0,
        &format!("{keyword} function __CEnumDeserialize(enums: {{ [string]: any }}, value: unknown): unknown"),
    );
    output.push_str(
        r#"	if type(value) == "table" and getmetatable(value :: any) == __CEnumEnumItemMap then
		return value
	end

	return __CEnumDeserializeValue(enums, value)
end

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
