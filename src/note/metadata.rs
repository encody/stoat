use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Metadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(with = "date_serde", default, skip_serializing_if = "Option::is_none")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(with = "date_serde", default, skip_serializing_if = "Option::is_none")]
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
}

mod date_serde {
    use serde::Deserialize;

    pub fn serialize<S>(
        date: &chrono::DateTime<chrono::Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&date.to_rfc2822())
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<chrono::DateTime<chrono::Utc>>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = if let Ok(s) = String::deserialize(deserializer) {
            s
        } else {
            return Ok(None);
        };

        Ok(Some(dateparser::parse(&s).map_err(|e| {
            <D::Error as serde::de::Error>::custom(format!("invalid date: {}", e))
        })?))
    }
}
