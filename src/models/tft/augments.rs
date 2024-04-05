use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use crate::models::shared::{has_image, BasicDatum, Image};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Augments {
    pub version: String,
    #[serde(rename = "augment-container")]
    pub augment_container: AugmentContainer,
    pub data: HashMap<String, BasicDatum>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AugmentContainer {
    pub name: String,
    pub image: Image,
}

has_image!(AugmentContainer);

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum AugmentContainerFields {
    Name,
    Image,
}

impl<'de> Deserialize<'de> for AugmentContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AugmentContainerVisitor;

        impl<'de> Visitor<'de> for AugmentContainerVisitor {
            type Value = AugmentContainer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct AugmentContainer")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let name = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let image = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                Ok(AugmentContainer { name, image })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut name = None;
                let mut image = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        AugmentContainerFields::Name => {
                            name = Some(map.next_value()?);
                        }
                        AugmentContainerFields::Image => {
                            image = Some(map.next_value()?);
                        }
                    }
                }

                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let image = image.ok_or_else(|| serde::de::Error::missing_field("image"))?;

                Ok(AugmentContainer { name, image })
            }
        }

        const FIELDS: &[&str] = &["name", "image"];
        deserializer.deserialize_struct("AugmentContainer", FIELDS, AugmentContainerVisitor)
    }
}
