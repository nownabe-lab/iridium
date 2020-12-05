use nom::types::CompleteStr;
use nom::digit;

use assembler::Token;
use assembler::label_parsers::label_usage;
use assembler::register_parsers::register;

named!(pub operand<CompleteStr, Token>,
    alt!(
        integer_operand |
        label_usage |
        register |
        irstring
    )
);

named!(integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            reg_num: digit >>
            (
                Token::IntegerOperand{value: reg_num.parse::<i32>().unwrap()}
            )
        )
    )
);

named!(irstring<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("'") >>
            content: take_until!("'") >>
            tag!("'") >>
            (
                Token::IrString{ name: content.to_string() }
            )
        )
    )
);

mod tests {
    #![allow(unused_imports)]

    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = integer_operand(CompleteStr("#10"));
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::IntegerOperand{value: 10});
        let result = integer_operand(CompleteStr("10"));
        assert_eq!(result.is_ok(), false)
    }

    #[test]
    fn test_parse_irstring_operand() {
        let result = irstring(CompleteStr("'This is a test'"));
        assert_eq!(true, result.is_ok());
        assert_eq!(
            Ok((CompleteStr(""), Token::IrString{ name: "This is a test".to_string() })),
            result
        );
    }
}
