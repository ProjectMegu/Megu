pub type HirRefers = Vec<HirRef>;
/// (nspace name, item_name in file)
pub type HirRef = (Vec<String>, String);