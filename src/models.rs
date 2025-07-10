use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternateNameEntry {
    pub alternate_name_id: i64,
    pub geoname_id: i64,
    pub isolanguage: String,
    pub alternate_name: String,
    pub is_preferred_name: Option<bool>,
    pub is_short_name: Option<bool>,
    pub is_colloquial: Option<bool>,
    pub is_historic: Option<bool>,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildEntry {
    pub id: i64, // Changed from i32 to i64
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedGeoNameEntry {
    pub name: String,
    pub children: Vec<ChildEntry>,
}

#[derive(Debug, Clone)]
pub struct GeoName {
    pub geoname_id: i64, // Changed from i32 to i64
    pub name: String,
    pub feature_class: char,
    pub feature_code: String,
    pub country_code: String,
    pub admin1_code: String,
}

impl GeoName {
    pub fn is_continent(&self) -> bool {
        self.feature_class == 'L' && self.feature_code == "CONT"
    }

    pub fn is_country(&self) -> bool {
        self.feature_class == 'A' && matches!(self.feature_code.as_str(), "PCLI" | "PCLF" | "PCLS")
    }

    pub fn is_state_region(&self) -> bool {
        self.feature_class == 'A' && self.feature_code == "ADM1"
    }

    pub fn is_city_town(&self) -> bool {
        self.feature_class == 'P' && self.feature_code == "PPL"
    }
}
