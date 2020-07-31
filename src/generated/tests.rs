#[cfg(test)]
mod tests {
    use crate::generated::galaxy::*;
    use crate::runtime::*;

    fn to_num_vec(v: Value) -> Vec<i64> {
        v.unwrap_list()
            .into_iter()
            .map(|v| v.unwrap_number())
            .rev() // Stored in reverse
            .collect()
    }

    #[test]
    fn test_1029() {
        assert_eq!(
            vec![7, 123229502148636],
            to_num_vec(__1029(Value::Number(0)))
        );
    }
}
