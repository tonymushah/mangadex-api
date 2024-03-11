use crate::{error::Error, include_enums};
use serde::{Deserialize, Serialize};

// Flag to include future updates in the results.

include_enums! {
    IncludeExternalUrl
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::to_string;

    #[derive(Debug, Serialize)]
    struct TestStruct {
        value: IncludeExternalUrl,
    }
    #[test]
    fn test_serialization() {
        assert_eq!(
            to_string(&TestStruct {
                value: IncludeExternalUrl::Exclude
            })
            .unwrap(),
            r#"{"value":0}"#
        );
    }
}
