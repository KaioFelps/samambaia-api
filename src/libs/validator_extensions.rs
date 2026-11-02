use std::collections::HashMap;

use serde::Serialize;
use serde::de::DeserializeOwned;
use validator::Validate;

pub trait FilterQueryExt: Validate + Serialize + DeserializeOwned {
    fn validate_and_sanitize(mut self) -> (Self, HashMap<String, String>) {
        let mut error_messages = HashMap::new();

        if let Err(errors) = self.validate() {
            for (field, field_errors) in errors.field_errors() {
                for err in field_errors {
                    if let Some(msg) = &err.message {
                        error_messages.insert(field.to_string(), msg.to_string());
                    }
                }
            }

            if let Ok(mut json_value) = serde_json::to_value(&self) {
                if let Some(obj) = json_value.as_object_mut() {
                    for field_name in errors.field_errors().keys() {
                        obj.insert(field_name.to_string(), serde_json::Value::Null);
                    }
                }
                if let Ok(cleaned_struct) = serde_json::from_value(json_value) {
                    self = cleaned_struct;
                }
            }
        }

        (self, error_messages)
    }
}

impl<T: Validate + Serialize + DeserializeOwned> FilterQueryExt for T {}
