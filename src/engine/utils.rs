pub mod pq;
pub mod types;
pub mod concurrency;

use rdb::ColumnFamily;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

pub fn get_serialized<T: DeserializeOwned>(
    instance: &rocksdb::DB,
    cf: &ColumnFamily,
    key: &str,
) -> Result<Option<T>, String> {
    match instance.get_cf(cf, key) {
        Ok(opt) => match opt {
            Some(found) => match String::from_utf8(found) {
                Ok(s) => match serde_json::from_str::<T>(&s) {
                    Ok(t) => Ok(Some(t)),
                    Err(err) => Err(format!("Failed to deserialize: {:?}", err)),
                },
                Err(err) => Err(format!("Failed to convert to String: {:?}", err)),
            },
            None => Ok(None),
        },
        Err(err) => Err(format!("Failed to get from ColumnFamily: {:?}", err)),
    }
}

pub fn put_serialized<T: Serialize + std::fmt::Debug>(
    instance: &mut rocksdb::DB,
    cf: &ColumnFamily,
    key: &str,
    value: &T,
) -> Result<(), String> {
    match serde_json::to_string(&value) {
        Ok(serialized) => instance
            .put_cf(cf, &key, serialized.into_bytes())
            .map_err(|err| format!("Failed to put to ColumnFamily:{:?}", err)),
        Err(err) => Err(format!(
            "Failed to serialize to String. T: {:?}, err: {:?}",
            value, err
        )),
    }
}