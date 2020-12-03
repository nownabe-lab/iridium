use nom::types::CompleteStr;
use nom::alphanumeric;
use nom::multispace;

use assembler::Token;

named!(pub label_declaration<CompleteStr, Token>,
    ws!(
        do_parse!(
            name: alphanumeric >>
            tag!(":") >>
            opt!(multispace) >>
            (
                Token::LabelDeclaration{name: name.to_string()}
            )
        )
    )
);

named!(pub label_usage<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("@") >>
            name: alphanumeric >>
            opt!(multispace) >>
            (
                Token::LabelUsage{name: name.to_string()}
            )
        )
    )
);

mod tests{
    #![allow(unused_imports)]

    use super::*;

    #[test]
    fn test_parse_label_declaration() {
        let result = label_declaration(CompleteStr("test:"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                Token::LabelDeclaration{name: "test".to_string()},
            ))
        );

        let result = label_declaration(CompleteStr("test"));
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_label_usage() {
        let result = label_usage(CompleteStr("@test"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                Token::LabelUsage{name: "test".to_string()},
            ))
        );

        let result = label_usage(CompleteStr("test"));
        assert_eq!(result.is_ok(), false);
    }
}

