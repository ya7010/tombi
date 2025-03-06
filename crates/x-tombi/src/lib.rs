mod array_values_order;
mod table_keys_order;

pub const X_TOMBI_TOML_VERSION: &str = "x-tombi-toml-version";
pub const X_TOMBI_ARRAY_VALUES_ORDER: &str = "x-tombi-array-values-order";
pub const X_TOMBI_TABLE_KEYS_ORDER: &str = "x-tombi-table-keys-order";

pub use array_values_order::ArrayValuesOrder;
pub use table_keys_order::TableKeysOrder;
