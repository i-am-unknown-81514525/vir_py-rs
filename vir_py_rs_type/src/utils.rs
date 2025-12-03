use crate::base::VirPyType;

pub fn reveal<T>(data_type: T) -> String where T : VirPyType {
    std::any::type_name::<T>().to_string()
}