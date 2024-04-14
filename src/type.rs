/// Defines the possible types of data in Aura
/// 
/// - `Primitive`: the predefined basic atomic data types 
/// - `Compound`: types that agregate multiple types inside of it (product type)
/// - `Alternative`: a type that can be different types (only one at a time) (sum type)
/// - `Object`: the version of `Compound` where each component has a custom identifier
/// - `Enumeration`: the version of `Alternative` where each variant has a custom identifier
/// - `Collection`: meant to store several items of a same type
pub enum Type {
    Primitive,
    Compound,
    Alternative,
    Object,
    Enumeration,
    Collection,
}