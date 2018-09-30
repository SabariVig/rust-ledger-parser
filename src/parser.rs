use std::str::FromStr;
use nom::*;
use nom::types::CompleteStr;
use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Debug,PartialEq,Eq)]
pub enum CommodityPosition {
    Left,
    Right
}

#[derive(Debug,PartialEq,Eq)]
pub struct Commodity {
    pub name: String,
    pub position: CommodityPosition
}

#[derive(Debug,PartialEq,Eq)]
pub struct Amount {
    pub quantity: Decimal,
    pub commodity: Commodity,
}

pub enum CustomError {
    NonExistingDate
}

fn is_digit(c: char) -> bool {
    (c >= '0' && c <= '9')
}

fn is_commodity_first_char(c: char) -> bool {
    (c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '$' || c > 0x7F as char)
}

fn is_commodity_char(c: char) -> bool {
    (c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '$' || c > 0x7F as char)
}

fn is_white_char(c: char) -> bool {
    (c == ' ' || c == '\t' || c == '\r' || c == '\n')
}

named_args!(numberN(n: usize)<CompleteStr, i32>,
    map_res!(take_while_m_n!(n, n, is_digit), |s: CompleteStr| { i32::from_str(s.0) })
);

named!(parse_date_internal<CompleteStr, (i32, i32, i32)>,
    do_parse!(
        year: call!(numberN, 4) >>
        alt!(tag!("-") | tag!("/") | tag!(".")) >>
        month: call!(numberN, 2) >>
        alt!(tag!("-") | tag!("/") | tag!(".")) >>
        day: call!(numberN, 2) >>
        ((year, month, day))
    )
);

pub fn parse_date(text: CompleteStr) -> IResult<CompleteStr, NaiveDate> {
    let res = parse_date_internal(text)?;

    let rest = res.0;
    let value = res.1;

    let parsed_opt = NaiveDate::from_ymd_opt(value.0, value.1 as u32, value.2 as u32);
    if let Some(parsed) = parsed_opt {
        Ok((rest, parsed))
    } else {
        Err(Err::Error(error_position!(CompleteStr(&text.0[0..10]), ErrorKind::Custom(CustomError::NonExistingDate as u32))))
    }
}

named!(parse_quantity<CompleteStr, Decimal>,
    map_res!(
        recognize!(
            tuple!(
                opt!(tag!("-")),
                digit,
                opt!(tuple!(tag!("."), digit))
            )
        ),
        |s: CompleteStr| { Decimal::from_str(s.0) }
    )
);

named!(string_between_quotes<CompleteStr, &str>,
    map!(
        delimited!(char!('\"'), is_not!("\""), char!('\"')),
        |s: CompleteStr| { s.0 }
    )
);

named!(commodity_without_quotes<CompleteStr, &str>,
    map!(
        recognize!(
            tuple!(
                take_while_m_n!(1, 1, is_commodity_first_char),
                take_while!(is_commodity_char)
            )
        ),
        |s: CompleteStr| { s.0 }
    )
);

named!(parse_commodity<CompleteStr, &str>,
    alt!(string_between_quotes | commodity_without_quotes)
);

named!(parse_amount<CompleteStr, Amount>,
    ws!(
        alt!(
            do_parse!(
                neg_opt: opt!(tag!("-")) >>
                commodity: parse_commodity >>
                quantity: parse_quantity >>
                (Amount {
                    quantity: if let Some(_) = neg_opt {
                        quantity * Decimal::new(-1, 0)
                    } else { quantity },
                    commodity: Commodity {
                        name: commodity.to_string(),
                        position: CommodityPosition::Left
                    }
                })
            )
            |
            do_parse!(
                quantity: parse_quantity >>
                commodity: parse_commodity >>
                (Amount {
                    quantity: quantity,
                    commodity: Commodity {
                        name: commodity.to_string(),
                        position: CommodityPosition::Right
                    }
                })
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::ErrorKind::Custom;
    use nom::Context::Code;
    use nom::Err::Error;
    use nom::types::CompleteStr;

    #[test]
    fn parse_date_test() {
        assert_eq!(Ok((CompleteStr(""), NaiveDate::from_ymd(2017, 03, 24))), parse_date(CompleteStr("2017-03-24")));
        assert_eq!(Ok((CompleteStr(""), NaiveDate::from_ymd(2017, 03, 24))), parse_date(CompleteStr("2017/03/24")));
        assert_eq!(Ok((CompleteStr(""), NaiveDate::from_ymd(2017, 03, 24))), parse_date(CompleteStr("2017.03.24")));
        assert_eq!(Err(Error(Code(CompleteStr("2017-13-24"), Custom(CustomError::NonExistingDate as u32)))), parse_date(CompleteStr("2017-13-24")));
    }

    #[test]
    fn parse_quantity_test() {
        assert_eq!(Ok((CompleteStr(""), Decimal::new(202, 2))), parse_quantity(CompleteStr("2.02")));
        assert_eq!(Ok((CompleteStr(""), Decimal::new(-1213, 2))), parse_quantity(CompleteStr("-12.13")));
        assert_eq!(Ok((CompleteStr(""), Decimal::new(1, 1))), parse_quantity(CompleteStr("0.1")));
        assert_eq!(Ok((CompleteStr(""), Decimal::new(3, 0))), parse_quantity(CompleteStr("3")));
    }

    #[test]
    fn parse_commodity_test() {
        assert_eq!(Ok((CompleteStr(""), "ABC 123")), parse_commodity(CompleteStr("\"ABC 123\"")));
        assert_eq!(Ok((CompleteStr(" "), "ABC")), parse_commodity(CompleteStr("ABC ")));
        assert_eq!(Ok((CompleteStr("1"), "$")), parse_commodity(CompleteStr("$1")));
    }

    #[test]
    fn parse_amount_test() {
        assert_eq!(Ok((CompleteStr(""), Amount { quantity: Decimal::new(120, 2), commodity: Commodity { name: "$".to_string(), position: CommodityPosition::Left }})), parse_amount(CompleteStr("$1.20")));
        assert_eq!(Ok((CompleteStr(""), Amount { quantity: Decimal::new(-120, 2), commodity: Commodity { name: "$".to_string(), position: CommodityPosition::Left }})), parse_amount(CompleteStr("$-1.20")));
        assert_eq!(Ok((CompleteStr(""), Amount { quantity: Decimal::new(-120, 2), commodity: Commodity { name: "$".to_string(), position: CommodityPosition::Left }})), parse_amount(CompleteStr("-$1.20")));
        assert_eq!(Ok((CompleteStr(""), Amount { quantity: Decimal::new(-120, 2), commodity: Commodity { name: "$".to_string(), position: CommodityPosition::Left }})), parse_amount(CompleteStr("- $ 1.20")));
        assert_eq!(Ok((CompleteStr(""), Amount { quantity: Decimal::new(120, 2), commodity: Commodity { name: "USD".to_string(), position: CommodityPosition::Right }})), parse_amount(CompleteStr("1.20USD")));
        assert_eq!(Ok((CompleteStr(""), Amount { quantity: Decimal::new(-120, 2), commodity: Commodity { name: "USD".to_string(), position: CommodityPosition::Right }})), parse_amount(CompleteStr("-1.20 USD")));
    }
}
