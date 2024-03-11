use crate::{error::Error, include_enums};
use serde::{Deserialize, Serialize};

include_enums! {IncludeFuturePages}
#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::to_string;

    #[derive(Debug, Serialize)]
    struct TestStruct {
        value: IncludeFuturePages,
    }
    #[test]
    fn test_serialization() {
        assert_eq!(
            to_string(&TestStruct {
                value: IncludeFuturePages::Exclude
            })
            .unwrap(),
            r#"{"value":0}"#
        );
    }
    #[test]
    fn parse_error() {
        if let Err(e) = IncludeFuturePages::try_from(2u8) {
            if let Error::IncludeEnumsParsing(e) = e {
                assert_eq!(e, "IncludeFuturePages");
            } else {
                panic!("Should return an IncludeEnumsParsing error")
            }
        } else {
            panic!("Should return an error")
        }
    }
}
