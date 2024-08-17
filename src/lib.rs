//! 
//! # num2en
//! This is a crate with functions for converting any integer or decimal number below
//! 2<sup>128</sup> (about 340 undecillion) to words.
//! <br> It supports converting to ***cardinal*** and ***ordinal*** numbers.
//! 
//! # Functions
//! To convert any integer `n` of type `X` to ***cardinal*** number words, call `X_to_words(n)`.
//! 
//! ```rust
//! use num2en::*;
//! assert_eq!(    u8_to_words(1),    "one");
//! assert_eq!(    i8_to_words(2),    "two");
//! assert_eq!(   u16_to_words(3),    "three");
//! assert_eq!(   i16_to_words(4),    "four");
//! assert_eq!(   u32_to_words(5),    "five");
//! assert_eq!(   i32_to_words(6),    "six");
//! assert_eq!(   u64_to_words(70),   "seventy");
//! assert_eq!(   i64_to_words(71),   "seventy-one");
//! assert_eq!(  u128_to_words(180),  "one hundred eighty");
//! assert_eq!(  i128_to_words(211),  "two hundred eleven");
//! assert_eq!( usize_to_words(1050), "one thousand fifty");
//! assert_eq!( isize_to_words(2012), "two thousand twelve");
//! ```
//! 
//! To convert any unsigned integer `n` of type `X` to ***ordinal*** number words, call
//! `X_to_ord_words(n)`.
//! 
//! ```rust
//! # use num2en::*;
//! assert_eq!(    u8_to_ord_words(1),    "first");
//! assert_eq!(   u16_to_ord_words(3),    "third");
//! assert_eq!(   u32_to_ord_words(5),    "fifth");
//! assert_eq!(   u64_to_ord_words(70),   "seventieth");
//! assert_eq!(  u128_to_ord_words(180),  "one hundred eightieth");
//! assert_eq!( usize_to_ord_words(2012), "two thousand twelfth");
//! ```
//! 
//! To convert any float `f` of type `Y` to number words, call `Y_to_words(f)`.
//! 
//! ```rust
//! # use num2en::*;
//! assert_eq!(  f32_to_words(15.2),  Ok("fifteen point two".to_string()));
//! assert_eq!(  f64_to_words(42.42), Ok("forty-two point four two".to_string()));
//! ```
//! 
//! To convert a string representation of a number to number words, call [`str_to_words`]`(&string)`.
//! 
//! ```rust
//! # use num2en::*;
//! assert_eq!( str_to_words("123.456"),
//!              Ok("one hundred twenty-three point four five six".to_string()) );
//! ```
//! 
//! To spell all digits in a string of digits individually, call [`str_digits_to_words`]`(&digits)`.
//! 
//! ```rust
//! # use num2en::*;
//! assert_eq!( str_digits_to_words("001247"), Ok("zero zero one two four seven".to_string()) );
//! ```
//! 
//! 
//! This crate has been thoroughly tested, but if you find any function working incorrectly
//! for some input, please [open an issue on Github](https://github.com/simon-sovic/num2en/issues/new).
//!


fn lt1000(n: u16, words: &mut Vec<String>) {
    let hundreds = n / 100;
    if hundreds != 0 {
        lt100(hundreds as u8, words);
        words.push("hundred".to_string());
    }
    let ones_and_tens = n % 100;
    if ones_and_tens != 0 {
        lt100(ones_and_tens as u8, words);
    }
}

fn lt100(n: u8, words: &mut Vec<String>) {
    const NUMS_SMALLER_THAN_20: [&str; 19] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    ];
    const NUMS_SMALLER_THAN_20_OFFSET: usize = 1;
    const MULTIPLES_OF_10: [&str; 8] = [
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    const MULTIPLES_OF_10_OFFSET: usize = 2;

    if n < 20 {
        words.push(NUMS_SMALLER_THAN_20[n as usize - NUMS_SMALLER_THAN_20_OFFSET].to_string());
    }
    else {
        let tens = n / 10;  // guaranteed to be at least 2 (because of previous check)
        let ones = n % 10;
        let mut word = MULTIPLES_OF_10[tens as usize - MULTIPLES_OF_10_OFFSET].to_string();
        if ones != 0 {
            word += "-";
            word += NUMS_SMALLER_THAN_20[ones as usize - NUMS_SMALLER_THAN_20_OFFSET];
        }
        words.push(word);
    }
}


/// names of periods (10 ** 3k)
const PERIODS: [&str; 12] = [
    "thousand", "million", "billion", "trillion", "quadrillion", "quintillion",
    "sextillion", "septillion", "octillion", "nonillion", "decillion", "undecillion",
];

macro_rules! create_public_conversion_func_of_unsigned_int {
    ( $t:ty, $name:ident, $num_of_periods:literal ) => {
        /// Converts any
        #[doc = concat!("`", stringify!($t), "`")]
        /// value to its **cardinal** number representation in words (***one, two, three*** etc.).
        ///
        /// # Arguments
        ///
        /// - `n`: An unsigned integer
        #[doc = concat!("(`", stringify!($t), "`)")]
        /// that represents the number to be converted.
        ///
        /// # Returns
        ///
        /// A [`String`] containing the English words that represent the input cardinal number.
        ///
        #[doc = concat!(
            "# Example\n\
            ```\n\
            use num2en::", stringify!($name), ";\n\n\
            let number = 12_142;\n\
            let words = ", stringify!($name), "(number);\n\
            assert_eq!(words, \"twelve thousand one hundred forty-two\");\n\
            ```"
        )]
        ///
        /// # Notes
        ///
        /// - The function includes hyphens for numbers between 21 and 99 (e.g., "twenty-one").
        pub fn $name(n: $t) -> String {
            if n == 0 {
                return "zero".to_string();
            }

            let mut words = Vec::<String>::new();

            let mut divisor = (1000 as $t).pow($num_of_periods);
            let mut idx = $num_of_periods;
            while divisor >= 1000 {
                idx -= 1;
                let current_period = (n / divisor) % 1000;
                if current_period != 0 {
                    lt1000(current_period as u16, &mut words);
                    words.push(PERIODS[idx].to_string());
                }
                divisor /= 1000;
            }

            lt1000((n % 1000) as u16, &mut words);

            return words.join(" ");
        }
    };
}

#[cfg(target_pointer_width = "64")]
create_public_conversion_func_of_unsigned_int!(usize, usize_to_words, 6);
#[cfg(target_pointer_width = "32")]
create_public_conversion_func_of_unsigned_int!(usize, usize_to_words, 3);
create_public_conversion_func_of_unsigned_int!(u128, u128_to_words, 12);
create_public_conversion_func_of_unsigned_int!(u64, u64_to_words, 6);
create_public_conversion_func_of_unsigned_int!(u32, u32_to_words, 3);
create_public_conversion_func_of_unsigned_int!(u16, u16_to_words, 1);
/// Converts any `u8` value to its **cardinal** number representation in words (***one, two, three*** etc.).
///
/// # Arguments
/// - `n`: An unsigned integer (`u8`) that represents the number to be converted.
///
/// # Returns
/// A [`String`] containing the English words that represent the input cardinal number.
///
/// # Example
/// ```
/// use num2en::u8_to_words;
///
/// let number = 142;
/// let words = num2en::u8_to_words(number);
/// assert_eq!(words, "one hundred forty-two");
/// ```
///
/// # Notes
/// - The function includes hyphens for numbers between 21 and 99 (e.g., "twenty-one").
pub fn u8_to_words(n: u8) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    let mut words = Vec::<String>::new();
    lt1000(n as u16, &mut words);
    return words.join(" ");
}


const ORD_NUMS_EXCEPTIONS: [(&str, &str); 7] = [
    ("one", "first"), ("two", "second"), ("three", "third"), ("five", "fifth"),
    ("eight", "eighth"), ("nine", "ninth"), ("twelve", "twelfth"),
];

macro_rules! create_public_conversion_func_of_unsigned_int_ord {
    ( $t:ty, $name:ident, $num_of_periods:literal ) => {
        /// Converts any
        #[doc = concat!("`", stringify!($t), "`")]
        /// value to its **ordinal** number representation in words (***first, second, third*** etc.).
        ///
        /// # Arguments
        ///
        /// - `n`: An unsigned integer
        #[doc = concat!("(`", stringify!($t), "`)")]
        /// that represents the number to be converted.
        ///
        /// # Returns
        ///
        /// A [`String`] containing the English words that represent the input ordinal number.
        ///
        #[doc = concat!(
            "# Example\n\
            ```\n\
            use num2en::", stringify!($name), ";\n\n\
            let number = 12;\n\
            let words = ", stringify!($name), "(number);\n\
            assert_eq!(words, \"twelfth\");\n\n\
            let number = 12_142;\n\
            let words = ", stringify!($name), "(number);\n\
            assert_eq!(words, \"twelve thousand one hundred forty-second\");\n\
            ```"
        )]
        ///
        /// # Notes
        ///
        /// - The function includes hyphens for numbers between 21 and 99 (e.g., "twenty-first").
        pub fn $name(n: $t) -> String {
            if n == 0 {
                return "zeroth".to_string();
            }

            let mut words = Vec::<String>::new();

            let mut divisor = (1000 as $t).pow($num_of_periods);
            let mut idx = $num_of_periods;
            while divisor >= 1000 {
                idx -= 1;
                let current_period = (n / divisor) % 1000;
                if current_period != 0 {
                    lt1000(current_period as u16, &mut words);
                    words.push(PERIODS[idx].to_string());
                }
                divisor /= 1000;
            }

            lt1000((n % 1000) as u16, &mut words);

            // Modify the last word to an ordinal word
            let mut last_word = &words.pop().unwrap()[..];
            let mut penultimate_word = "";
            if let Some(hyphen_index) = last_word.find('-') {
                penultimate_word = &last_word[.. hyphen_index + 1];
                last_word = &last_word[hyphen_index + 1 ..];
            }
            if let Some(index) = ORD_NUMS_EXCEPTIONS.iter().position(|x| x.0 == last_word) {
                words.push(penultimate_word.to_string() + ORD_NUMS_EXCEPTIONS[index].1);
            }
            else if last_word.ends_with("y") {
                words.push(penultimate_word.to_string() + &last_word[.. last_word.len() - 1] + "ieth");
            }
            else {
                words.push(penultimate_word.to_string() + last_word + "th");
            }

            return words.join(" ");
        }
    };
}

#[cfg(target_pointer_width = "64")]
create_public_conversion_func_of_unsigned_int_ord!(usize, usize_to_ord_words, 6);
#[cfg(target_pointer_width = "32")]
create_public_conversion_func_of_unsigned_int_ord!(usize, usize_to_ord_words, 3);
create_public_conversion_func_of_unsigned_int_ord!(u128, u128_to_ord_words, 12);
create_public_conversion_func_of_unsigned_int_ord!(u64, u64_to_ord_words, 6);
create_public_conversion_func_of_unsigned_int_ord!(u32, u32_to_ord_words, 3);
create_public_conversion_func_of_unsigned_int_ord!(u16, u16_to_ord_words, 1);
/// Converts any `u8` value to its **ordinal** number representation in words (***first, second, third*** etc.).
///
/// # Arguments
/// - `n`: An unsigned integer (`u8`) that represents the number to be converted.
///
/// # Returns
/// A [`String`] containing the English words that represent the input ordinal number.
///
/// # Examples
/// ```
/// use num2en::u8_to_ord_words;
/// 
/// let number = 13;
/// let words = u8_to_ord_words(number);
/// assert_eq!(words, "thirteenth");
/// 
/// let number = 142;
/// let words = u8_to_ord_words(number);
/// assert_eq!(words, "one hundred forty-second");
/// ```
///
/// # Notes
/// - The function includes hyphens for numbers between 21 and 99 (e.g., "twenty-first").
pub fn u8_to_ord_words(n: u8) -> String { u16_to_ord_words(n as u16) }


macro_rules! create_public_conversion_func_of_signed_int {
    ( $t:tt, $name:ident, $num_of_periods:literal ) => {
        /// Converts any
        #[doc = concat!("`", stringify!($t), "`")]
        /// value to its **cardinal** number representation in words (***one, two, three*** etc.).
        ///
        /// # Arguments
        ///
        /// - `n`: A signed integer
        #[doc = concat!("(`", stringify!($t), "`)")]
        /// that represents the number to be converted.
        ///
        /// # Returns
        ///
        /// A [`String`] containing the English words that represent the input cardinal number.
        ///
        #[doc = concat!(
            "# Example\n\
            ```\n\
            use num2en::", stringify!($name), ";\n\n\
            let number = 1969;\n\
            let words = ", stringify!($name), "(number);\n\
            assert_eq!(words, \"one thousand nine hundred sixty-nine\");\n\n\
            let number = -2918;\n\
            let words = ", stringify!($name), "(number);\n\
            assert_eq!(words, \"negative two thousand nine hundred eighteen\");\n\
            ```"
        )]
        ///
        /// # Notes
        ///
        /// - The function includes hyphens for numbers between 21 and 99 (e.g., "twenty-one").
        pub fn $name(n: $t) -> String {
            if n == 0 {
                return "zero".to_string();
            }

            let mut words = Vec::<String>::new();

            type UnsignedType = signed_to_unsigned!($t);
            let mut nonnegative_n = n as UnsignedType;
            if n < 0 {
                words.push("negative".to_string());
                if n > <$t>::MIN {
                    // values in range (iX::MIN, 0) don't map correctly to uX without negating first
                    nonnegative_n = -n as UnsignedType;
                }
            }

            let mut divisor = (1000 as UnsignedType).pow($num_of_periods);
            let mut idx = $num_of_periods;
            while divisor >= 1000 {
                idx -= 1;
                let current_period = (nonnegative_n / divisor) % 1000;
                if current_period != 0 {
                    lt1000(current_period as u16, &mut words);
                    words.push(PERIODS[idx].to_string());
                }
                divisor /= 1000;
            }

            lt1000((nonnegative_n % 1000) as u16, &mut words);

            return words.join(" ");
        }
    };
}

macro_rules! signed_to_unsigned {
    (i16) => { u16 };
    (i32) => { u32 };
    (i64) => { u64 };
    (i128) => { u128 };
    (isize) => { usize };
}

#[cfg(target_pointer_width = "64")]
create_public_conversion_func_of_signed_int!(isize, isize_to_words, 6);
#[cfg(target_pointer_width = "32")]
create_public_conversion_func_of_signed_int!(isize, isize_to_words, 3);
create_public_conversion_func_of_signed_int!(i128, i128_to_words, 12);
create_public_conversion_func_of_signed_int!(i64, i64_to_words, 6);
create_public_conversion_func_of_signed_int!(i32, i32_to_words, 3);
create_public_conversion_func_of_signed_int!(i16, i16_to_words, 1);
/// Converts any `u8` value to its **cardinal** number representation in words (***one, two, three*** etc.).
///
/// # Arguments
/// - `n`: A signed integer (`u8`) that represents the number to be converted.
///
/// # Returns
/// A [`String`] containing the English words that represent the input cardinal number.
///
/// # Examples
/// ```
/// use num2en::i8_to_words;
///
/// let number = 120;
/// let words = i8_to_words(number);
/// assert_eq!(words, "one hundred twenty");
///
/// let number = -111;
/// let words = i8_to_words(number);
/// assert_eq!(words, "negative one hundred eleven");
/// ```
///
/// # Notes
/// - The function includes hyphens for numbers between 21 and 99 (e.g., "twenty-one").
pub fn i8_to_words(n: i8) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    let mut words = Vec::<String>::new();
    let mut nonnegative_n = n as u8;
    if n < 0 {
        words.push("negative".to_string());
        if n > i8::MIN {
            nonnegative_n = -n as u8;
        }
    }
    lt1000(nonnegative_n as u16, &mut words);
    return words.join(" ");
}


#[derive(Debug, PartialEq)]
/// Represents the possible error that can occur when calling [str_digits_to_words].
pub enum DigitConversionError {
    /// Indicates that the string contains a character other than `0`, `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, or `9`.
    InvalidCharacter,
}

/// Converts any string of digits (`0`-`9`) to a string of all the digits spelled out individually.
///
/// # Arguments
/// - `digits`: `&str` of digits to be converted.
///
/// # Returns
/// [`Result`]`<`[`String`]`, `[`DigitConversionError`]`>`
/// 
/// The string contains all the digits spelled out individually.
/// 
/// For example, `"123"` becomes `"one two three"`.
///
/// # Examples
/// ```
/// use num2en::str_digits_to_words;
/// # use num2en::DigitConversionError;
/// 
/// let digits = "12408842";
/// let result = str_digits_to_words(digits);
/// assert_eq!(result, Ok("one two four zero eight eight four two".to_string()));
/// 
/// let digits = "00015000";
/// let result = str_digits_to_words(digits);
/// assert_eq!(result, Ok("zero zero zero one five zero zero zero".to_string()));
/// 
/// // A string with non-digit characters results in an error.
/// let invalid_string = "124brb";
/// let result = str_digits_to_words(invalid_string);
/// assert_eq!(result, Err(DigitConversionError::InvalidCharacter));
/// 
/// // An empty string doesn't do anything.
/// let empty_string = "";
/// let result = str_digits_to_words(empty_string);
/// assert_eq!(result, Ok("".to_string()));
/// ```
pub fn str_digits_to_words(digits: &str) -> Result<String, DigitConversionError> {
    let mut words = Vec::with_capacity(digits.len());
    for digit in digits.chars() {
        words.push(match digit {
            '0' => "zero",
            '1' => "one",
            '2' => "two",
            '3' => "three",
            '4' => "four",
            '5' => "five",
            '6' => "six",
            '7' => "seven",
            '8' => "eight",
            '9' => "nine",
            _ => return Err(DigitConversionError::InvalidCharacter)
        });
    }
    Ok(words.join(" "))
}


#[derive(Debug, PartialEq)]
/// Represents the possible errors that can occur when calling [str_to_words].
pub enum StrConversionError {
    /// This could mean the string contains invalid characters or is in an incorrect format.
    InvalidString,
    /// Indicates that the value is too large to be converted.
    TooLarge,
}

/// Converts any* string of a (decimal) number to a number representation in words.
///
/// # Arguments
/// - `string`: `&str` representing a number in the `... xxxxxx.xxxxxx ...` format, where `x` is any digit.
/// <br> * The integer part must be 2<sup>128</sup> - 1 (~ 340 undecillion) or smaller, while
/// the decimal part is unrestricted.
///
/// # Returns
/// [`Result`]`<`[`String`]`, `[`StrConversionError`]`>`
/// 
/// The string contains the English words that represent the input number.
/// 
/// For example, `"123.456"` becomes `"one hundred twenty-three point four five six"`.
///
/// # Examples
/// ```
/// use num2en::str_to_words;
/// # use num2en::StrConversionError;
/// 
/// let number = "123.123";
/// let result = str_to_words(number);
/// assert_eq!(result, Ok("one hundred twenty-three point one two three".to_string()));
/// 
/// let number = "1095";
/// let result = str_to_words(number);
/// assert_eq!(result, Ok("one thousand ninety-five".to_string()));
/// 
/// let number = "0.0042";
/// let result = str_to_words(number);
/// assert_eq!(result, Ok("zero point zero zero four two".to_string()));
///
/// let number = ".0042";
/// let result = str_to_words(number);
/// assert_eq!(result, Ok("point zero zero four two".to_string()));
/// 
/// let number = "1095.";
/// let result = str_to_words(number);
/// assert_eq!(result, Ok("one thousand ninety-five point".to_string()));
/// 
/// // Leading zeros are ignored.
/// let number = "0003000";
/// let result = str_to_words(number);
/// assert_eq!(result, Ok("three thousand".to_string()));
/// 
/// // This is (almost) the largest allowed number (it could have any number of nines):
/// let number = "340282366920938463463374607431768211455.99999999";
/// let result = str_to_words(number);
/// assert_eq!(result, Ok("three hundred forty undecillion two hundred eighty-two \
/// decillion three hundred sixty-six nonillion nine hundred twenty octillion nine \
/// hundred thirty-eight septillion four hundred sixty-three sextillion four hundred \
/// sixty-three quintillion three hundred seventy-four quadrillion six hundred seven \
/// trillion four hundred thirty-one billion seven hundred sixty-eight million two \
/// hundred eleven thousand four hundred fifty-five point nine nine nine nine nine \
/// nine nine nine".to_string()));
/// 
/// // A string with invalid characters results in an error.
/// let invalid_string = "235:53";
/// let result = str_to_words(invalid_string);
/// assert_eq!(result, Err(StrConversionError::InvalidString));
/// 
/// // An empty string doesn't do anything.
/// let empty_string = "";
/// let result = str_to_words(empty_string);
/// assert_eq!(result, Ok("".to_string()));
/// ```
/// 
/// # Notes
/// - Scientific notation (e.g. `"4.2e1"`) is not supported.
/// - This function supports only numbers between `-u128::MAX-1` (exclusive) and `u128::MAX+1` (exclusive).
/// - The function includes hyphens for numbers between 21 and 99 (e.g., "twenty-one").
/// - This function uses [u128_to_words] and [str_digits_to_words] behind the curtains.
pub fn str_to_words(string: &str) -> Result<String, StrConversionError> {
    use std::num::IntErrorKind;

    if string.len() == 0 {
        return Ok("".to_string());
    }

    // Validity check
    let mut decimal_point_flag = false;
    let mut at_least_one_digit_flag = false;
    for (i, byte) in string.bytes().enumerate() {
        if byte == b'.' {
            if decimal_point_flag {
                return Err(StrConversionError::InvalidString);
            }
            decimal_point_flag = true;
            continue;
        }
        if byte >= b'0' && byte <= b'9' {
            at_least_one_digit_flag = true;
        }
        else if !(i == 0 && byte == b'-') {
            return Err(StrConversionError::InvalidString);
        }
    }
    if !at_least_one_digit_flag {
        return Err(StrConversionError::InvalidString)
    }

    let mut string = string;

    let mut words = Vec::<String>::new();

    if string.bytes().nth(0).unwrap() == b'-' {
        words.push("negative".to_string());
        string = &string[1..];
    }

    let floating_point_index_option = string.find('.');

    let integer_part_result = string[..floating_point_index_option.unwrap_or(string.len())].parse::<u128>();

    match integer_part_result {
        Err(parse_int_err) => {
            match parse_int_err.kind() {
                IntErrorKind::Empty => {},
                IntErrorKind::InvalidDigit => unreachable!(),
                IntErrorKind::NegOverflow => unreachable!(),
                IntErrorKind::PosOverflow => {
                    return Err(StrConversionError::TooLarge);
                },
                IntErrorKind::Zero => unreachable!(),
                _ => unreachable!(),
            }
        },
        Ok(integer_part) => {
            words.push(u128_to_words(integer_part));
        }
    }

    if let Some(floating_point_index) = floating_point_index_option {
        words.push("point".to_string());
        if floating_point_index < string.len() - 1 {
            let decimal_part = &string[floating_point_index + 1..];
            words.push(str_digits_to_words(decimal_part).unwrap());
        }
    }

    return Ok(words.join(" "));
}


#[derive(Debug, PartialEq)]
/// Represents the possible errors that can occur when calling [f32_to_words] or [f64_to_words].
pub enum FloatConversionError {
    /// Indicates that the value is not finite (i.e., it is either `NaN`, positive infinity, or negative infinity).
    NotFinite,
    /// Indicates that the value is too large to be converted.
    TooLarge,
}

macro_rules! create_public_conversion_func_of_float {
    ( $t:ty, $name:ident ) => {
        /// Converts any*
        #[doc = concat!("`", stringify!($t), "`")]
        /// value of a number to a number representation in words.
        ///
        /// # Arguments
        /// - `float`: A float
        #[doc = concat!("(`", stringify!($t), "`)")]
        /// that represents the number to be converted.
        /// <br> * The number must be 2<sup>128</sup> - 1 (~ 340 undecillion) or smaller,
        /// otherwise a [TooLarge](FloatConversionError::TooLarge) error gets returned.
        ///
        /// # Returns
        /// [`Result`]`<`[`String`]`, `[`FloatConversionError`]`>`
        /// 
        /// The string contains the English words that represent the input number.
        /// 
        /// For example, `"123.456"` becomes `"one hundred twenty-three point four five six"`.
        ///
        #[doc = concat!(
            "# Examples\n\
            ```\n\
            use num2en::", stringify!($name), ";\n\
            # use num2en::FloatConversionError;\n\n\
            let number = 123.123;\n\
            let result = ", stringify!($name), "(number);\n\
            assert_eq!(result, Ok(\"one hundred twenty-three point one two three\".to_string()));\n\n\
            let number = 4e-5;\n\
            let result = ", stringify!($name), "(number);\n\
            assert_eq!(result, Ok(\"zero point zero zero zero zero four\".to_string()));\n\n\
            let number = 34.000;\n\
            let result = ", stringify!($name), "(number);\n\
            assert_eq!(result, Ok(\"thirty-four\".to_string()));\n\n\
            let infinity = ", stringify!($t), "::INFINITY;\n\
            let result = ", stringify!($name), "(infinity);\n\
            assert_eq!(result, Err(FloatConversionError::NotFinite));\n\n\
            let not_a_number = ", stringify!($t), "::NAN;\n\
            let result = ", stringify!($name), "(not_a_number);\n\
            assert_eq!(result, Err(FloatConversionError::NotFinite));\n\
            ```"
        )]
        /// 
        /// # Notes
        /// - This function supports only numbers between `-u128::MAX-1` (exclusive) and `u128::MAX+1` (exclusive).
        /// - The function includes hyphens for numbers between 21 and 99 (e.g., "twenty-one").
        /// - This function uses [str_to_words] behind the curtains.
        pub fn $name(float: $t) -> Result<String, FloatConversionError> {
            if !float.is_finite() {
                return Err(FloatConversionError::NotFinite);
            }

            let float_string = float.to_string();

            match str_to_words(&float_string) {
                Err(StrConversionError::TooLarge) => return Err(FloatConversionError::TooLarge),
                Err(StrConversionError::InvalidString) => unreachable!(),
                Ok(words) => return Ok(words),
            }
        }
    };
}

create_public_conversion_func_of_float!(f32, f32_to_words);
create_public_conversion_func_of_float!(f64, f64_to_words);


#[cfg(test)]
mod tests;
