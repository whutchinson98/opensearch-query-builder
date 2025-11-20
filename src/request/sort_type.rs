use std::borrow::Cow;

use serde::Serialize;
use serde_json::{Map, Value};

use crate::ToOpenSearchJson;

/// Sort Order
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// Ascending
    Asc,
    /// Descending
    Desc,
}

/// Field Sort
#[derive(Debug, Clone, Serialize)]
pub struct FieldSort<'a> {
    /// The field to sort on
    #[serde(borrow)]
    pub field: Cow<'a, str>,
    /// Sort order
    pub order: SortOrder,
    /// Missing value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(borrow)]
    pub missing: Option<Cow<'a, str>>,
}

/// Score sort with order
#[derive(Debug, Clone, Serialize)]
pub struct ScoreWithOrderSort {
    /// Sort order
    pub order: SortOrder,
}

impl<'a> FieldSort<'a> {
    /// Create a new FieldSort
    pub fn new(field: Cow<'a, str>, order: SortOrder) -> Self {
        Self {
            field,
            order,
            missing: None,
        }
    }

    /// Set the missing value
    pub fn missing(mut self, missing: Cow<'a, str>) -> Self {
        self.missing = Some(missing);
        self
    }
}

impl ScoreWithOrderSort {
    /// Create a new ScoreWithOrderSort
    pub fn new(order: SortOrder) -> Self {
        Self { order }
    }
}

impl<'a> ToOpenSearchJson for FieldSort<'a> {
    fn to_json(&self) -> Value {
        let mut result = Map::new();

        // Use simplified format when there are no additional parameters
        if self.missing.is_none() {
            result.insert(
                self.field.to_string(),
                Value::String(match self.order {
                    SortOrder::Asc => "asc".to_string(),
                    SortOrder::Desc => "desc".to_string(),
                }),
            );
        } else {
            // Use object format when there are additional parameters
            let mut field_obj = Map::new();
            field_obj.insert(
                "order".to_string(),
                Value::String(match self.order {
                    SortOrder::Asc => "asc".to_string(),
                    SortOrder::Desc => "desc".to_string(),
                }),
            );

            if let Some(ref missing) = self.missing {
                field_obj.insert("missing".to_string(), Value::String(missing.to_string()));
            }

            result.insert(self.field.to_string(), Value::Object(field_obj));
        }

        Value::Object(result)
    }
}

impl ToOpenSearchJson for ScoreWithOrderSort {
    fn to_json(&self) -> Value {
        let mut result = Map::new();
        result.insert(
            "_score".to_string(),
            Value::String(match self.order {
                SortOrder::Asc => "asc".to_string(),
                SortOrder::Desc => "desc".to_string(),
            }),
        );
        Value::Object(result)
    }
}

/// Sort Type
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "params")]
pub enum SortType<'a> {
    /// Field sort
    Field(FieldSort<'a>),
    /// Score sort
    Score,
    /// Score with sort order
    ScoreWithOrder(ScoreWithOrderSort),
}
impl<'a> ToOpenSearchJson for SortType<'a> {
    fn to_json(&self) -> Value {
        match self {
            SortType::Field(field_sort) => field_sort.to_json(),
            SortType::Score => serde_json::json!("_score"),
            SortType::ScoreWithOrder(score_sort) => score_sort.to_json(),
        }
    }
}
