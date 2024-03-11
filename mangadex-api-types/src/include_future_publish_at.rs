use crate::{error::Error, include_enums};
use serde::{Deserialize, Serialize};

include_enums!(IncludeFuturePublishAt);

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::to_string;

    #[derive(Debug, Serialize)]
    struct TestStruct {
        value: IncludeFuturePublishAt,
    }
    #[test]
    fn test_serialization() {
        assert_eq!(
            to_string(&TestStruct {
                value: IncludeFuturePublishAt::Exclude
            })
            .unwrap(),
            r#"{"value":0}"#
        );
    }
}
