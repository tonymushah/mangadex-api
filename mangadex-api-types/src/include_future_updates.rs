use crate::error::Error;
use serde::{Deserialize, Serialize};

use crate::include_enums;

include_enums!(IncludeFutureUpdates);

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::to_string;

    #[derive(Debug, Serialize)]
    struct TestStruct {
        value: IncludeFutureUpdates,
    }
    #[test]
    fn test_serialization() {
        assert_eq!(
            to_string(&TestStruct {
                value: IncludeFutureUpdates::Exclude
            })
            .unwrap(),
            r#"{"value":0}"#
        );
    }
}
