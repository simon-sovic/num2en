# num2en
This is a crate with functions for converting any integer or decimal number below
2<sup>128</sup> (about 340 undecillion) to words.
<br> It supports converting to ***cardinal*** and ***ordinal*** numbers.

# Functions
To convert any integer `n` of type `X` to ***cardinal*** number words, call `X_to_words(n)`.

```rust
assert_eq!(    u8_to_words(1),    "one");
assert_eq!(    i8_to_words(2),    "two");
assert_eq!(   u16_to_words(3),    "three");
assert_eq!(   i16_to_words(4),    "four");
assert_eq!(   u32_to_words(5),    "five");
assert_eq!(   i32_to_words(6),    "six");
assert_eq!(   u64_to_words(70),   "seventy");
assert_eq!(   i64_to_words(71),   "seventy-one");
assert_eq!(  u128_to_words(180),  "one hundred eighty");
assert_eq!(  i128_to_words(211),  "two hundred eleven");
assert_eq!( usize_to_words(1050), "one thousand fifty");
assert_eq!( isize_to_words(2012), "two thousand twelve");
```

To convert any unsigned integer `n` of type `X` to ***ordinal*** number words, call
`X_to_ord_words(n)`.

```rust
assert_eq!(    u8_to_ord_words(1),    "first");
assert_eq!(   u16_to_ord_words(3),    "third");
assert_eq!(   u32_to_ord_words(5),    "fifth");
assert_eq!(   u64_to_ord_words(70),   "seventieth");
assert_eq!(  u128_to_ord_words(180),  "one hundred eightieth");
assert_eq!( usize_to_ord_words(2012), "two thousand twelfth");
```

To convert any float `f` of type `Y` to number words, call `Y_to_words(f)`.

```rust
assert_eq!(  f32_to_words(15.2),  Ok("fifteen point two".to_string()));
assert_eq!(  f64_to_words(42.42), Ok("forty-two point four two".to_string()));
```

To convert a string representation of a number to number words, call `str_to_words`.

```rust
assert_eq!( str_to_words("123.456"), Ok("one hundred twenty-three point four five six".to_string()) );
```

To spell all digits in a string of digits individually, call `str_digits_to_words`.

```rust
assert_eq!( str_digits_to_words("001247"), Ok("zero zero one two four seven".to_string()) );
```


This crate has been thoroughly tested, but if you find any function working incorrectly
for some input, please [open an issue on Github](https://github.com/simon-sovic/num2en/issues/new).
