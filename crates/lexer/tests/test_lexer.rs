use lexer::{tokenize, Token};
use syntax::{SyntaxKind::*, T};

macro_rules!  test_tokens {
    {$(#[test]fn $name:ident($source:expr) -> [
        $(Token($kind:expr, ($line:expr, $column:expr)),)*
    ]);+;} => {
        $(
            #[test]
            fn $name() {
                let tokens = tokenize($source).collect::<Vec<_>>();
                let expected = [
                    $(
                        Ok(Token::new($kind, ($line, $column).into())),
                    )*
                ];
                assert_eq!(tokens, expected);
            }
        )+
    };
}

macro_rules! test_token {
    {$(#[test]fn $name:ident($source:expr) -> Token($kind:expr, ($line:expr, $column:expr)));+;} => {
        $(
            #[test]
            fn $name() {
                let tokens = tokenize(&textwrap::dedent($source).trim()).collect::<Vec<_>>();
                assert_eq!(tokens, [Ok(Token::new($kind, ($line, $column).into()))]);
            }
        )+
    };
}

test_tokens! {
    #[test]
    fn empty_source("") -> [];

    #[test]
    fn comment_line_break("# This is a comment\n") -> [
        Token(COMMENT, (0, 19)),
        Token(LINE_BREAK, (19, 20)),
    ];

    #[test]
    fn comment_line_break_crlf("# This is a comment\r\n") -> [
        Token(COMMENT, (0, 19)),
        Token(LINE_BREAK, (19, 21)),
    ];

    #[test]
    fn whitespace_comment_line_break("   # This is a comment\n") -> [
        Token(WHITESPACE, (0, 3)),
        Token(COMMENT, (3, 22)),
        Token(LINE_BREAK, (22, 23)),
    ];

    #[test]
    fn whitespace_comment_line_break_crlf("   # This is a comment\r\n") -> [
        Token(WHITESPACE, (0, 3)),
        Token(COMMENT, (3, 22)),
        Token(LINE_BREAK, (22, 24)),
    ];

    #[test]
    fn comment_whitespace_line_break("# This is a comment   \n") -> [
        Token(COMMENT, (0, 22)),
        Token(LINE_BREAK, (22, 23)),
    ];

    #[test]
    fn tokens("{},.[]=") -> [
        Token(T!('{'), (0, 1)),
        Token(T!('}'), (1, 2)),
        Token(T!(,), (2, 3)),
        Token(T!(.), (3, 4)),
        Token(T!('['), (4, 5)),
        Token(T!(']'), (5, 6)),
        Token(T!(=), (6, 7)),
    ];

    #[test]
    fn key_value_float_dot_key("3.14159 = \"pi\"") -> [
        Token(FLOAT, (0, 7)),
        Token(WHITESPACE, (7, 8)),
        Token(EQUAL, (8, 9)),
        Token(WHITESPACE, (9, 10)),
        Token(BASIC_STRING, (10, 14)),
    ];

    #[test]
    fn key_value_dotted_keys("apple.type = \"fruit\"") -> [
        Token(BARE_KEY, (0, 5)),
        Token(DOT, (5, 6)),
        Token(BARE_KEY, (6, 10)),
        Token(WHITESPACE, (10, 11)),
        Token(EQUAL, (11, 12)),
        Token(WHITESPACE, (12, 13)),
        Token(BASIC_STRING, (13, 20)),
    ];

    #[test]
    fn table_only_header(r#"[package]"#) -> [
        Token(BRACKET_START, (0, 1)),
        Token(BARE_KEY, (1, 8)),
        Token(BRACKET_END, (8, 9)),
    ];

    #[test]
    fn table(
        r#"
        [package]
        name = "toml"
        version = "0.5.8"
        "#
    ) -> [
            Token(LINE_BREAK, (0, 1)),
            Token(WHITESPACE, (1, 9)),
            Token(BRACKET_START, (9, 10)),
            Token(BARE_KEY, (10, 17)),
            Token(BRACKET_END, (17, 18)),
            Token(LINE_BREAK, (18, 19)),
            Token(WHITESPACE, (19, 27)),
            Token(BARE_KEY, (27, 31)),
            Token(WHITESPACE, (31, 32)),
            Token(EQUAL, (32, 33)),
            Token(WHITESPACE, (33, 34)),
            Token(BASIC_STRING, (34, 40)),
            Token(LINE_BREAK, (40, 41)),
            Token(WHITESPACE, (41, 49)),
            Token(BARE_KEY, (49, 56)),
            Token(WHITESPACE, (56, 57)),
            Token(EQUAL, (57, 58)),
            Token(WHITESPACE, (58, 59)),
            Token(BASIC_STRING, (59, 66)),
            Token(LINE_BREAK, (66, 67)),
            Token(WHITESPACE, (67, 75)),
        ];

    #[test]
    fn inline_table("key = { key2 = \"value\" }") -> [
        Token(BARE_KEY, (0, 3)),
        Token(WHITESPACE, (3, 4)),
        Token(EQUAL, (4, 5)),
        Token(WHITESPACE, (5, 6)),
        Token(BRACE_START, (6, 7)),
        Token(WHITESPACE, (7, 8)),
        Token(BARE_KEY, (8, 12)),
        Token(WHITESPACE, (12, 13)),
        Token(EQUAL, (13, 14)),
        Token(WHITESPACE, (14, 15)),
        Token(BASIC_STRING, (15, 22)),
        Token(WHITESPACE, (22, 23)),
        Token(BRACE_END, (23, 24)),
    ];

    #[test]
    fn invalid_inline_table("key1 = { key2 = 'value") -> [
        Token(BARE_KEY, (0, 4)),
        Token(WHITESPACE, (4, 5)),
        Token(EQUAL, (5, 6)),
        Token(WHITESPACE, (6, 7)),
        Token(BRACE_START, (7, 8)),
        Token(WHITESPACE, (8, 9)),
        Token(BARE_KEY, (9, 13)),
        Token(WHITESPACE, (13, 14)),
        Token(EQUAL, (14, 15)),
        Token(WHITESPACE, (15, 16)),
        Token(INVALID_TOKEN, (16, 22)),
    ];

    #[test]
    fn array_of_table(
        r#"
        [[package]]
        name = "toml"
        version = "0.5.8"

        [[package]]
        name = "json"
        version = "1.2.4"
        "#
    ) -> [
            Token(LINE_BREAK, (0, 1)),
            Token(WHITESPACE, (1, 9)),
            Token(BRACKET_START, (9, 10)),
            Token(BRACKET_START, (10, 11)),
            Token(BARE_KEY, (11, 18)),
            Token(BRACKET_END, (18, 19)),
            Token(BRACKET_END, (19, 20)),
            Token(LINE_BREAK, (20, 21)),
            Token(WHITESPACE, (21, 29)),
            Token(BARE_KEY, (29, 33)),
            Token(WHITESPACE, (33, 34)),
            Token(EQUAL, (34, 35)),
            Token(WHITESPACE, (35, 36)),
            Token(BASIC_STRING, (36, 42)),
            Token(LINE_BREAK, (42, 43)),
            Token(WHITESPACE, (43, 51)),
            Token(BARE_KEY, (51, 58)),
            Token(WHITESPACE, (58, 59)),
            Token(EQUAL, (59, 60)),
            Token(WHITESPACE, (60, 61)),
            Token(BASIC_STRING, (61, 68)),
            Token(LINE_BREAK, (68, 69)),
            Token(LINE_BREAK, (69, 70)),
            Token(WHITESPACE, (70, 78)),
            Token(BRACKET_START, (78, 79)),
            Token(BRACKET_START, (79, 80)),
            Token(BARE_KEY, (80, 87)),
            Token(BRACKET_END, (87, 88)),
            Token(BRACKET_END, (88, 89)),
            Token(LINE_BREAK, (89, 90)),
            Token(WHITESPACE, (90, 98)),
            Token(BARE_KEY, (98, 102)),
            Token(WHITESPACE, (102, 103)),
            Token(EQUAL, (103, 104)),
            Token(WHITESPACE, (104, 105)),
            Token(BASIC_STRING, (105, 111)),
            Token(LINE_BREAK, (111, 112)),
            Token(WHITESPACE, (112, 120)),
            Token(BARE_KEY, (120, 127)),
            Token(WHITESPACE, (127, 128)),
            Token(EQUAL, (128, 129)),
            Token(WHITESPACE, (129, 130)),
            Token(BASIC_STRING, (130, 137)),
            Token(LINE_BREAK, (137, 138)),
            Token(WHITESPACE, (138, 146)),
    ];
}

test_token! {
    #[test]
    fn only_comment("# This is a comment") -> Token(COMMENT, (0, 19));

    #[test]
    fn basic_string1(r#""Hello, World!""#) -> Token(BASIC_STRING, (0, 15));

    #[test]
    fn basic_string2(r#""Hello, \"Taro\"!""#) -> Token(BASIC_STRING, (0, 18));

    #[test]
    fn multi_line_basic_string1(r#""""aaaa""""#) -> Token(MULTI_LINE_BASIC_STRING, (0, 10));

    #[test]
    fn multi_line_basic_string2(
        r#"
        """
        aaaa
        """
        "#
    ) -> Token(MULTI_LINE_BASIC_STRING, (0, 12));

    #[test]
    fn literal_string1("'Hello, World!'") -> Token(LITERAL_STRING, (0, 15));

    #[test]
    fn literal_string2("'Hello, \\'Taro\\'!'") -> Token(LITERAL_STRING, (0, 18));

    #[test]
    fn offset_date_time1("2021-01-01T00:00:00Z") -> Token(OFFSET_DATE_TIME, (0, 20));

    #[test]
    fn offset_date_time2("2021-01-01T00:00:00+09:00") -> Token(OFFSET_DATE_TIME, (0, 25));

    #[test]
    fn offset_date_time3("2021-01-01T00:00:00-09:00") -> Token(OFFSET_DATE_TIME, (0, 25));

    #[test]
    fn offset_date_time4("2021-01-01T00:00:00.123456Z") -> Token(OFFSET_DATE_TIME, (0, 27));

    #[test]
    fn offset_date_time5("2021-01-01T00:00:00.123456+09:00") -> Token(OFFSET_DATE_TIME, (0, 32));

    #[test]
    fn offset_date_time6("2021-01-01T00:00:00.123456-09:00") -> Token(OFFSET_DATE_TIME, (0, 32));

    #[test]
    fn local_date_time1("2021-01-01 00:00:00") -> Token(LOCAL_DATE_TIME, (0, 19));

    #[test]
    fn local_date_time2("2021-01-01 00:00:00.123456") -> Token(LOCAL_DATE_TIME, (0, 26));

    #[test]
    fn local_date_time3("2021-01-01T00:00:00") -> Token(LOCAL_DATE_TIME, (0, 19));

    #[test]
    fn local_date_time4("2021-01-01T00:00:00.123456") -> Token(LOCAL_DATE_TIME, (0, 26));

    #[test]
    fn local_date1("2021-01-01") -> Token(LOCAL_DATE, (0, 10));

    #[test]
    fn local_time1("00:00:00") -> Token(LOCAL_TIME, (0, 8));

    #[test]
    fn local_time2("00:00:00.123456") -> Token(LOCAL_TIME, (0, 15));

    #[test]
    fn boolean1("true") -> Token(BOOLEAN, (0, 4));

    #[test]
    fn boolean2("false") -> Token(BOOLEAN, (0, 5));

    #[test]
    fn key1("key") -> Token(BARE_KEY, (0, 3));

    #[test]
    fn key2("_1234567890") -> Token(BARE_KEY, (0, 11));

    #[test]
    fn key3("key_123") -> Token(BARE_KEY, (0, 7));

    #[test]
    fn integer_dec1("0") -> Token(INTEGER_DEC, (0, 1));

    #[test]
    fn integer_dec2("1") -> Token(INTEGER_DEC, (0, 1));

    #[test]
    fn integer_dec3("1234567890") -> Token(INTEGER_DEC, (0, 10));

    #[test]
    fn integer_dec4("+1234567890") -> Token(INTEGER_DEC, (0, 11));

    #[test]
    fn integer_dec5("-1234567890") -> Token(INTEGER_DEC, (0, 11));

    #[test]
    fn integer_dec6("1_234_567_890") -> Token(INTEGER_DEC, (0, 13));

    #[test]
    fn integer_dec7("+1_234_567_890") -> Token(INTEGER_DEC, (0, 14));

    #[test]
    fn integer_dec8("-1_234_567_890") -> Token(INTEGER_DEC, (0, 14));

    #[test]
    fn invalid_integer_dec1("+_1234567890") -> Token(INVALID_TOKEN, (0, 12));

    #[test]
    fn invalid_integer_dec2("-_1234567890") -> Token(INVALID_TOKEN, (0, 12));

    #[test]
    fn integer_bin1("0b0") -> Token(INTEGER_BIN, (0, 3));

    #[test]
    fn integer_bin2("0b1") -> Token(INTEGER_BIN, (0, 3));

    #[test]
    fn integer_bin3("0b01") -> Token(INTEGER_BIN, (0, 4));

    #[test]
    fn integer_bin4("0b10") -> Token(INTEGER_BIN, (0, 4));

    #[test]
    fn integer_bin5("0b101010") -> Token(INTEGER_BIN, (0, 8));

    #[test]
    fn integer_bin6("0b_1010_10") -> Token(INTEGER_BIN, (0, 10));

    #[test]
    fn integer_bin7("0b10_101_010") -> Token(INTEGER_BIN, (0, 12));

    #[test]
    fn integer_oct1("0o0") -> Token(INTEGER_OCT, (0, 3));

    #[test]
    fn integer_oct2("0o1") -> Token(INTEGER_OCT, (0, 3));

    #[test]
    fn integer_oct3("0o01") -> Token(INTEGER_OCT, (0, 4));

    #[test]
    fn integer_oct4("0o10") -> Token(INTEGER_OCT, (0, 4));

    #[test]
    fn integer_oct5("0o1234567") -> Token(INTEGER_OCT, (0, 9));

    #[test]
    fn integer_oct6("0o_1234_567") -> Token(INTEGER_OCT, (0, 11));

    #[test]
    fn integer_oct7("0o12_34_567") -> Token(INTEGER_OCT, (0, 11));

    #[test]
    fn integer_hex1("0x0") -> Token(INTEGER_HEX, (0, 3));

    #[test]
    fn integer_hex2("0x1") -> Token(INTEGER_HEX, (0, 3));

    #[test]
    fn integer_hex3("0x01") -> Token(INTEGER_HEX, (0, 4));

    #[test]
    fn integer_hex4("0x10") -> Token(INTEGER_HEX, (0, 4));

    #[test]
    fn integer_hex5("0x1234567890abcdef") -> Token(INTEGER_HEX, (0, 18));

    #[test]
    fn integer_hex6("0x_1234_5678_90ab_cdef") -> Token(INTEGER_HEX, (0, 22));

    #[test]
    fn integer_hex7("0x12_34_5678_90ab_cdef") -> Token(INTEGER_HEX, (0, 22));

    #[test]
    fn float1("+1.0") -> Token(FLOAT, (0, 4));

    #[test]
    fn float2("3.1415") -> Token(FLOAT, (0, 6));

    #[test]
    fn float3("-0.01") -> Token(FLOAT, (0, 5));

    #[test]
    fn float4("5e+22") -> Token(FLOAT, (0, 5));

    #[test]
    fn float5("1e06") -> Token(FLOAT, (0, 4));

    #[test]
    fn float6("-2E-2") -> Token(FLOAT, (0, 5));

    #[test]
    fn float7("6.626e-34") -> Token(FLOAT, (0, 9));

    #[test]
    fn float8("224_617.445_991_228") -> Token(FLOAT, (0, 19));

    #[test]
    fn special_float1("inf") -> Token(FLOAT, (0, 3));

    #[test]
    fn special_float2("nan") -> Token(FLOAT, (0, 3));

    #[test]
    fn special_float3("+inf") -> Token(FLOAT, (0, 4));

    #[test]
    fn special_float4("+nan") -> Token(FLOAT, (0, 4));

    #[test]
    fn special_float5("-inf") -> Token(FLOAT, (0, 4));

    #[test]
    fn special_float6("-nan") -> Token(FLOAT, (0, 4));

}
