use magnus::Value;

pub fn public_name(value: Value) -> &'static str {
    if value.is_nil() {
        "null"
    } else if value.is_kind_of(magnus::class::integer()) {
        "integer"
    } else if value.is_kind_of(magnus::class::float()) {
        "float"
    } else if value.is_kind_of(magnus::class::string()) {
        "string"
    } else if value.is_kind_of(magnus::class::array()) {
        "list"
    } else if value.is_kind_of(magnus::class::hash()) {
        "object"
    } else if value.is_kind_of(magnus::class::true_class())
        || value.is_kind_of(magnus::class::false_class())
    {
        "boolean"
    } else {
        "unknown"
    }
}
