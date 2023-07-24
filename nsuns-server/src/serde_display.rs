//! Serialize/Deserialize a field using std::fmt::Display

use std::fmt::Display;
use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serializer};

pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    serializer.collect_str(value)
}

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse()
        .map_err(de::Error::custom)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[test]
    fn struct_fields() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Test {
            #[serde(with = "super")]
            u: u64,
            #[serde(with = "super")]
            i: i64,
        }

        let raw = r#" { "u": "18446744073709551615", "i": "-9223372036854775808" } "#;
        let de: Test = serde_json::from_str(raw).unwrap();

        let s = Test {
            i: -9223372036854775808,
            u: 18446744073709551615,
        };

        let ser = serde_json::to_string(&s).unwrap();

        assert_eq!(de.i, -9223372036854775808);
        assert_eq!(de.u, 18446744073709551615);

        assert_eq!(serde_json::from_str::<Test>(&ser).unwrap(), s);
    }

    #[test]
    fn single_de() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Test(#[serde(with = "super")] u64);

        let raw = r#""18446744073709551615""#;
        let de: Test = serde_json::from_str(raw).unwrap();

        assert_eq!(de.0, 18446744073709551615);
    }

    #[test]
    fn vec_de() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Test(#[serde(with = "super")] u64);

        let raw = r#"["18446744073709551615","18446744073709551615"]"#;
        let de: Vec<Test> = serde_json::from_str(raw).unwrap();

        assert_eq!(
            de,
            vec![Test(18446744073709551615), Test(18446744073709551615)]
        );
    }

    #[test]
    fn vec_ser() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Test(#[serde(with = "super")] u64);

        let de = vec![Test(18446744073709551615), Test(18446744073709551615)];

        let ser = serde_json::to_string(&de).unwrap();

        assert_eq!(ser, r#"["18446744073709551615","18446744073709551615"]"#);
    }
}
