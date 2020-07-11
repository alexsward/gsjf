enum Type {
    Object,
    Array,
    String,
    Null,
    Boolean(bool),
    Number
}

struct JSONResult {
    json_type: Type
}