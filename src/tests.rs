use std::fs;

use super::*;

fn testdata_filepath(filename: &str) -> String {
    env!("CARGO_MANIFEST_DIR").to_string() + "/testdata/" + filename
}

fn get_inputs_and_expected_outputs(testdata_filename: &str) -> Vec<(String, String)> {
    let lines = fs::read_to_string(testdata_filepath(testdata_filename))
        .expect("There should be a file in testdata")
        .lines()
        .map(|str| str.to_string())
        .collect::<Vec<_>>();

    let mut values = Vec::new();
    for line in lines {
        let split = line.split(";").collect::<Vec<_>>();
        if split.len() < 2 {
            continue;
        }
        values.push((split[0].to_string(), split[1].to_string()));
    }
    return values;
}

fn test_func<I, O>(
    testdata_filename: &str,
    parse_input_func: fn(&str) -> I,
    parse_expected_output_func: fn(&str) -> O,
    test_func: fn(I) -> O
)
where
    I: std::fmt::Display + core::marker::Copy,
    O: std::cmp::PartialEq + std::fmt::Display + std::fmt::Debug,
{
    let inputs_and_expected_outputs = get_inputs_and_expected_outputs(testdata_filename);

    for (input, expected_output) in inputs_and_expected_outputs {
        let input = parse_input_func(&input);
        let expected_output = parse_expected_output_func(&expected_output);
        let actual_output = test_func(input);
        assert_eq!(expected_output, actual_output,
            "input: '{}' expected output: '{}' actual output: '{}'", input, expected_output, actual_output);
    }
}

fn test_result_func<I, O, Err>(
    testdata_filename: &str,
    parse_input_func: fn(&str) -> I,
    parse_expected_output_func: fn(&str) -> Result<O, Err>,
    test_func: fn(I) -> Result<O, Err>
)
where
    I: std::fmt::Display,
    O: std::fmt::Display + std::cmp::PartialEq + std::fmt::Debug,
    Err: std::cmp::PartialEq + std::fmt::Debug,
{
    let inputs_and_expected_outputs = get_inputs_and_expected_outputs(testdata_filename);

    for (input, expected_output) in inputs_and_expected_outputs {
        let input_copy = input.clone();
        let input = parse_input_func(&input);
        let expected_output = parse_expected_output_func(&expected_output);
        let actual_output = test_func(input);
        match (expected_output, actual_output) {
            (Ok(ex_val), Err(_ac_val)) => { panic!("Expected Ok({}), got Err", ex_val); }
            (Err(_ex_val), Ok(ac_val)) => { panic!("Expected Err, got Ok({})", ac_val); }
            (Ok(ex_val), Ok(ac_val)) => {
                assert_eq!(ex_val, ac_val,
                    "\ninput: '{}'\nexpected output: 'Ok({})'\nactual output: 'Ok({})'", input_copy, ex_val, ac_val);
            }
            (Err(ex_val), Err(ac_val)) => {
                assert_eq!(ex_val, ac_val,
                    "\ninput: '{}'\nexpected and actual output errors are different", input_copy);
            }
        }
    }
}

#[test]
fn all_nums_between_0_and_1000() {
    test_func("0_ge_nums_lt_1000.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        u128_to_words);
}

#[test]
fn all_nums_between_0_and_negative_1000() {
    test_func("-1000_gt_nums_lt_0.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        i128_to_words);
}

#[test]
fn nums_represented_by_u8() {
    test_func("u8_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        u8_to_words);
}

#[test]
fn nums_represented_by_i8() {
    test_func("i8_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        i8_to_words);
}

#[test]
fn nums_represented_by_u16() {
    test_func("u16_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        u16_to_words);
}

#[test]
fn nums_represented_by_i16() {
    test_func("i16_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        i16_to_words);
}

#[test]
fn nums_represented_by_u32() {
    test_func("u32_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        u32_to_words);
}

#[test]
fn nums_represented_by_i32() {
    test_func("i32_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        i32_to_words);
}

#[test]
fn nums_represented_by_u64() {
    test_func("u64_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        u64_to_words);
}

#[test]
fn nums_represented_by_i64() {
    test_func("i64_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        i64_to_words);
}

#[test]
fn nums_represented_by_u128() {
    test_func("u128_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        u128_to_words);
}

#[test]
fn nums_represented_by_i128() {
    test_func("i128_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        i128_to_words);
}

#[test]
fn nums_represented_by_usize() {
    #[cfg(target_pointer_width="64")]
    test_func("u64_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        usize_to_words);
    #[cfg(target_pointer_width="32")]
    test_func("u32_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        usize_to_words);
}

#[test]
fn nums_represented_by_isize() {
    #[cfg(target_pointer_width="64")]
    test_func("i64_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        isize_to_words);
    #[cfg(target_pointer_width="32")]
    test_func("i32_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        isize_to_words);
}

#[test]
fn func_str_digits_to_words() {
    test_result_func("spell_digits_ok.csv",
        |i| i.to_string(),
        |o| Ok(o.to_string()),
        |x| str_digits_to_words(&x));

    test_result_func("spell_digits_err.csv",
        |i| i.to_string(),
        |_o| Err(DigitConversionError::InvalidCharacter),
        |x| str_digits_to_words(&x));
}

#[test]
fn nums_represented_by_str() {
    test_result_func("str_nums_ok.csv",
        |i| i.to_string(),
        |o| Ok(o.to_string()),
        |x| str_to_words(&x));

    test_result_func("str_nums_err_invalid.csv",
        |i| i.to_string(),
        |_o| Err(StrConversionError::InvalidString),
        |x| str_to_words(&x));

    test_result_func("str_nums_err_too-large.csv",
        |i| i.to_string(),
        |_o| Err(StrConversionError::TooLarge),
        |x| str_to_words(&x));
}

#[test]
fn nums_represented_by_f32() {
    test_result_func("f32_nums_ok.csv",
        |i| i.parse().unwrap(),
        |o| Ok(o.to_string()),
        f32_to_words);

    test_result_func("f32_nums_err_not-finite.csv",
        |i| i.parse().unwrap(),
        |_o| Err(FloatConversionError::NotFinite),
        f32_to_words);

    test_result_func("f32_nums_err_too-large.csv",
        |i| i.parse().unwrap(),
        |_o| Err(FloatConversionError::TooLarge),
        f32_to_words);
}

#[test]
fn nums_represented_by_f64() {
    test_result_func("f64_nums_ok.csv",
        |i| i.parse().unwrap(),
        |o| Ok(o.to_string()),
        f64_to_words);

    test_result_func("f64_nums_err_not-finite.csv",
        |i| i.parse().unwrap(),
        |_o| Err(FloatConversionError::NotFinite),
        f64_to_words);

    test_result_func("f64_nums_err_too-large.csv",
        |i| i.parse().unwrap(),
        |_o| Err(FloatConversionError::TooLarge),
        f64_to_words);
}

#[test]
fn ord_nums_represented_by_u8() {
    test_func("u8_ord_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        u8_to_ord_words);
}

#[test]
fn ord_nums_represented_by_u16() {
    test_func("u16_ord_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        u16_to_ord_words);
}

#[test]
fn ord_nums_represented_by_u32() {
    test_func("u32_ord_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        u32_to_ord_words);
}

#[test]
fn ord_nums_represented_by_u64() {
    test_func("u64_ord_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        u64_to_ord_words);
}

#[test]
fn ord_nums_represented_by_u128() {
    test_func("u128_ord_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        u128_to_ord_words);
}

#[test]
fn ord_nums_represented_by_usize() {
    #[cfg(target_pointer_width="64")]
    test_func("u64_ord_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        usize_to_ord_words);
    #[cfg(target_pointer_width="32")]
    test_func("u32_ord_nums.csv",
        |i| i.parse().unwrap(),
        |o| o.to_string(),
        usize_to_ord_words);
}
