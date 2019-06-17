use rocksdb;
use bincode::{serialize, deserialize};
use serde::ser::Serialize;
use serde::de::Deserialize;

lazy_static! {
    static ref DB: rocksdb::DB =
        { rocksdb::DB::open_default("./tmp").unwrap() };
}

pub fn set<'a, 'b, Key, Value>(key: &'a Key, value: &'b Value) -> Result<(), rocksdb::Error>
where Key: Serialize + Deserialize<'a>, Value: 'b + Serialize + Deserialize<'b>
{
    DB.put(
        serialize(&key).ok().unwrap()[..].as_ref(),
        serialize(&value).ok().unwrap()[..].as_ref(),
    )
}

pub fn get<'a, 'b, Key,  Value>(key: &'a Key) -> Option<&'b Value>
where Key: Serialize + Deserialize<'a>, &'b Value: Serialize + Deserialize<'b>
{
    let encoded: &[u8] = &serialize(&key).ok().unwrap()[..];
    match DB.get(encoded) {
        Ok(Some(value)) => {
            let vec = value.to_vec();
            deserialize(&vec[..]).ok()
        },
        _ => None,
    }
}

pub fn delete<'a, Key>(key: &'a Key) -> Result<(), rocksdb::Error>
where Key: Serialize + Deserialize<'a>
{
    DB.delete(serialize(&key).ok().unwrap()[..].as_ref())
}
