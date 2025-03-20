use crate::core::entity::zettel::document;


pub type ConversionError = ();


pub fn json_to_document(json: &serde_json::Value) -> Result<document::Document, ConversionError> {
    serde_json::from_value(json.clone()).map_err(|_| ())
}
