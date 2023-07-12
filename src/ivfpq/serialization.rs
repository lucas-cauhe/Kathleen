use serde::{Serialize, Deserialize, de::Visitor};
use super::ivfpq::AvlWrapper;
use super::primitive_types::IVListEntry;
use avl::AvlTreeMap;

impl Serialize for AvlWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        
        serializer.serialize_newtype_struct("AvlWrapper", &to_json(&self))
    }
}

impl<'de> Deserialize<'de> for AvlWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {

                struct AvlVisitor;
                impl<'de> Visitor<'de> for AvlVisitor {
                    type Value = AvlWrapper;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("struct AvlWrapper")
                    }

                    /* fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error, {
                        
                    } */

                    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error, {
                        let json_avl_wrapper: String = serde_cbor::from_slice(v).expect("Error deserializing in visitor");
                        Ok(AvlWrapper::from(from_json(json_avl_wrapper)))
                    }
                }
                deserializer.deserialize_tuple_struct("AvlWrapper", 1, AvlVisitor)
    }
}

fn to_json(source: &AvlTreeMap<u32, Box<IVListEntry>>) -> String {
    "{".to_string() + 
    &source
        .iter()
        .map(|(index, entry)| index.to_string() + ": " + &entry.to_string() + "\n")
        .collect::<String>() +
    "}"
}

fn from_json(mut source: String) -> AvlTreeMap<u32, Box<IVListEntry>> {
    source.remove(0);
    source.remove(source.len()-1);
    source.remove(source.len()-1);
    let splitted = source.split('\n');
    let mut avl = AvlTreeMap::new();
    splitted
        .for_each(|line| {
            let splitted = line.split(':').collect::<Vec<&str>>();
            let key = splitted[0];
            let value = splitted[1];
            avl.insert(key.parse::<u32>().unwrap(), Box::new(IVListEntry::from_str(value)));
        });
    avl
}

#[cfg(test)]
mod tests {
    use crate::ivfpq::ivfpq::CODE_SIZE;

    use super::*;

    #[test]
    fn serialization_works() {
        
    }

    #[test]
    fn avl_tree_map_serialization_works() {
        let mut avl = AvlWrapper::new();
        avl.insert(132, Box::new(IVListEntry::new([1; CODE_SIZE], 0)));
        avl.insert(132, Box::new(IVListEntry::new([2; CODE_SIZE], 1)));
        let curr_avl = avl.clone();
        
        let avl_bytes = to_json(&avl);
        let des_avl = from_json(avl_bytes);
        
        assert_eq!(to_json(&curr_avl), to_json(&des_avl));
    }

}