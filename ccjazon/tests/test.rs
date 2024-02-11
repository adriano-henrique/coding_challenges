#[path = "../src/validator.rs"]
mod validator;

#[cfg(test)]
mod validator_tests {
    use super::*;
    use crate::validator::validate_json;

    #[test]
    fn test_step1() {
        let paths = vec![
            "examples/step1/invalid.json".to_string(),
            "examples/step1/valid.json".to_string(),
        ];
        let expected_results = vec![false, true];

        let results: Vec<bool> = paths.iter().map(|path| validate_json(path)).collect();
        assert_eq!(results, expected_results);
    }
}
