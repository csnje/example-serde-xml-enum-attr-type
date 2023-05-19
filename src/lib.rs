use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use serde_value::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeA {
    #[serde(rename = "A")]
    member_a: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeB {
    #[serde(rename = "B", default, skip_serializing_if = "Option::is_none")]
    member_b: Option<i32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeC {
    #[serde(rename = "C")]
    member_c: Rc<RefCell<String>>, // required the serde "rc" feature
}

#[derive(Debug, PartialEq)]
pub enum Choice {
    TypeA(TypeA),
    TypeB(TypeB),
    TypeC(TypeC),
}

impl Serialize for Choice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Notes:
        // - the "@..." form of a serde rename causes the field name to become an
        //   attribute in the XML produced by the major XML serde implementations
        // - the xmlns namespace URI is included to produce valid XML

        const NAMESPACE: &str = "http://www.w3.org/2001/XMLSchema-instance";

        #[derive(Serialize)]
        struct Intermediate<'a, T: 'a> {
            #[serde(rename = "@xsi:type")]
            typ: String,
            #[serde(rename = "@xmlns:xsi")]
            namespace: &'a str,
            #[serde(flatten)]
            object: &'a T,
        }

        match self {
            Self::TypeA(object) => Intermediate {
                typ: "TypeA".to_string(),
                namespace: NAMESPACE,
                object,
            }
            .serialize(serializer),
            Self::TypeB(object) => Intermediate {
                typ: "TypeB".to_string(),
                namespace: NAMESPACE,
                object,
            }
            .serialize(serializer),
            Self::TypeC(object) => Intermediate {
                typ: "TypeC".to_string(),
                namespace: NAMESPACE,
                object,
            }
            .serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Choice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Notes:
        // - the major XML serde implementations strip the namespace from attributes
        //   (i.e. "@xsi:type" becomes "@type")

        const TYPE_ATTR: &str = "@type";

        let mut value = match Value::deserialize(deserializer)? {
            Value::Map(map) => map,
            _ => return Err(serde::de::Error::custom("unhandled deserialized structure")),
        };

        // Retrieve the value of the attribute identifying the type
        let typ = match value.remove(&Value::String(TYPE_ATTR.to_string())) {
            Some(Value::String(typ)) => typ,
            _ => return Err(serde::de::Error::missing_field(TYPE_ATTR)),
        };

        // Reserialize the data; this should correspond to (or be compatible with)
        // the deserializer originally applied
        let s =
            quick_xml::se::to_string_with_root("root", &value).map_err(serde::de::Error::custom)?;

        // Deserialize into the identified type
        match typ.as_str() {
            "TypeA" => Ok(Choice::TypeA(
                quick_xml::de::from_str(&s).map_err(serde::de::Error::custom)?,
            )),
            "TypeB" => Ok(Choice::TypeB(
                quick_xml::de::from_str(&s).map_err(serde::de::Error::custom)?,
            )),
            "TypeC" => Ok(Choice::TypeC(
                quick_xml::de::from_str(&s).map_err(serde::de::Error::custom)?,
            )),
            typ => Err(serde::de::Error::unknown_variant(
                typ,
                &["TypeA", "TypeB", "TypeC"],
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_choice_type_a() {
        let input = Choice::TypeA(TypeA {
            member_a: "test".to_string(),
        });

        let s = quick_xml::se::to_string_with_root("element", &input).unwrap();
        assert_eq!(
            r#"<element xsi:type="TypeA" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"><A>test</A></element>"#,
            s
        );

        let output: Choice = quick_xml::de::from_str(&s).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn test_serde_choice_type_b_none() {
        let input = Choice::TypeB(TypeB { member_b: None });

        let s = quick_xml::se::to_string_with_root("element", &input).unwrap();
        assert_eq!(
            r#"<element xsi:type="TypeB" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"/>"#,
            s
        );

        let output: Choice = quick_xml::de::from_str(&s).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn test_serde_choice_type_b_some() {
        let input = Choice::TypeB(TypeB { member_b: Some(42) });

        let s = quick_xml::se::to_string_with_root("element", &input).unwrap();
        assert_eq!(
            r#"<element xsi:type="TypeB" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"><B>42</B></element>"#,
            s
        );

        let output: Choice = quick_xml::de::from_str(&s).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn test_serde_choice_type_c() {
        let input = Choice::TypeC(TypeC {
            member_c: Rc::new(RefCell::new("test".to_string())),
        });

        let s = quick_xml::se::to_string_with_root("element", &input).unwrap();
        assert_eq!(
            r#"<element xsi:type="TypeC" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"><C>test</C></element>"#,
            s
        );

        let output: Choice = quick_xml::de::from_str(&s).unwrap();
        assert_eq!(input, output);
    }
}
