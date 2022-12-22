use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;

#[serde_as]
#[derive(Debug, Default, Serialize, Deserialize)]
struct Test {
    #[serde_as(as = "Vec<DisplayFromStr>")]
    data: Vec<i64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct TestStr {
    data: Vec<String>,
}

#[test]
fn test() {
    let test = Test {
        data: vec![1, 2, 3],
    };
    let json = serde_json::to_string(&test).unwrap();
    println!("{}", json);

    let test_str = TestStr {
        data: vec!["1".to_string(), "2".to_string(), "3".to_string()],
    };
    let json = serde_json::to_string(&test_str).unwrap();
    println!("{}", json);
}

#[test]
fn test_de() {
    let json_str = r#"{"data":["1","2","3"]}"#;
    let test: Test = serde_json::from_str(json_str).unwrap();
    println!("{:?}", test);

    let test: TestStr = serde_json::from_str(json_str).unwrap();
    println!("{:?}", test);
}
