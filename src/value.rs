use crate::r#type::Type;



struct Value {
    r#type: Type,
    data: Box<[u8]>,
}