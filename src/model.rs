use crate::serializer::*;
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use std::fmt;

///
/// Main document. Contains transactions and/or commodity prices.
///
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ledger {
    pub items: Vec<LedgerItem>,
}

impl fmt::Display for Ledger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.to_string_pretty(&SerializerSettings::default())
        )?;
        Ok(())
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LedgerItem {
    EmptyLine,
    LineComment(String),
    Transaction(Transaction),
    CommodityPrice(CommodityPrice),
    Include(String),
    PeriodicTransaction(PeriodicTransaction),
}

impl fmt::Display for LedgerItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.to_string_pretty(&SerializerSettings::default())
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Period {
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Date(NaiveDate),
    EveryNDays(u32),
    EveryNWeeks(u32),
    EveryNMonths(u32),
    EveryNYears(u32),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PeriodicTransaction {
    pub period: Period,
    pub comment: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub postings: Vec<Posting>,
}

///
/// Transaction.
///
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Transaction {
    pub comment: Option<String>,
    pub date: NaiveDate,
    pub effective_date: Option<NaiveDate>,
    pub status: Option<TransactionStatus>,
    pub code: Option<String>,
    pub description: String,
    pub postings: Vec<Posting>,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.to_string_pretty(&SerializerSettings::default())
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TransactionStatus {
    Pending,
    Cleared,
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.to_string_pretty(&SerializerSettings::default())
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Posting {
    pub account: String,
    pub reality: Reality,
    pub amount: Option<PostingAmount>,
    pub balance: Option<Balance>,
    pub status: Option<TransactionStatus>,
    pub comment: Option<String>,
}

impl fmt::Display for Posting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.to_string_pretty(&SerializerSettings::default())
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Reality {
    Real,
    BalancedVirtual,
    UnbalancedVirtual,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PostingAmount {
    pub amount: Amount,
    pub lot_price: Option<Price>,
    pub price: Option<Price>,
}

impl fmt::Display for PostingAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.to_string_pretty(&SerializerSettings::default())
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Amount {
    pub quantity: Decimal,
    pub commodity: Commodity,
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.to_string_pretty(&SerializerSettings::default())
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Commodity {
    pub name: String,
    pub position: CommodityPosition,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommodityPosition {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Price {
    Unit(Amount),
    Total(Amount),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Balance {
    Zero,
    Amount(Amount),
}

impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.to_string_pretty(&SerializerSettings::default())
        )?;
        Ok(())
    }
}

///
/// Commodity price.
///
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CommodityPrice {
    pub datetime: NaiveDateTime,
    pub commodity_name: String,
    pub amount: Amount,
}

impl fmt::Display for CommodityPrice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.to_string_pretty(&SerializerSettings::default())
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use rust_decimal::Decimal;

    #[test]
    fn display_transaction_status() {
        assert_eq!(format!("{}", TransactionStatus::Pending), "!");
        assert_eq!(format!("{}", TransactionStatus::Cleared), "*");
    }

    #[test]
    fn display_amount() {
        assert_eq!(
            format!(
                "{}",
                Amount {
                    quantity: Decimal::new(4200, 2),
                    commodity: Commodity {
                        name: "€".to_owned(),
                        position: CommodityPosition::Right,
                    }
                }
            ),
            "42.00 €"
        );
        assert_eq!(
            format!(
                "{}",
                Amount {
                    quantity: Decimal::new(4200, 2),
                    commodity: Commodity {
                        name: "USD".to_owned(),
                        position: CommodityPosition::Left,
                    }
                }
            ),
            "USD42.00"
        );
    }

    #[test]
    fn display_commodity_price() {
        let actual = format!(
            "{}",
            CommodityPrice {
                datetime: NaiveDate::from_ymd(2017, 11, 12).and_hms(12, 00, 00),
                commodity_name: "mBH".to_owned(),
                amount: Amount {
                    quantity: Decimal::new(500, 2),
                    commodity: Commodity {
                        name: "PLN".to_owned(),
                        position: CommodityPosition::Right
                    }
                }
            }
        );
        let expected = "P 2017-11-12 12:00:00 mBH 5.00 PLN";
        assert_eq!(actual, expected);
    }

    #[test]
    fn display_balance() {
        assert_eq!(
            format!(
                "{}",
                Balance::Amount(Amount {
                    quantity: Decimal::new(4200, 2),
                    commodity: Commodity {
                        name: "€".to_owned(),
                        position: CommodityPosition::Right,
                    }
                })
            ),
            "42.00 €"
        );
        assert_eq!(format!("{}", Balance::Zero), "0");
    }

    #[test]
    fn display_posting() {
        assert_eq!(
            format!(
                "{}",
                Posting {
                    account: "Assets:Checking".to_owned(),
                    reality: Reality::Real,
                    amount: Some(PostingAmount {
                        amount: Amount {
                            quantity: Decimal::new(4200, 2),
                            commodity: Commodity {
                                name: "USD".to_owned(),
                                position: CommodityPosition::Left,
                            }
                        },
                        lot_price: None,
                        price: None,
                    }),
                    balance: Some(Balance::Amount(Amount {
                        quantity: Decimal::new(5000, 2),
                        commodity: Commodity {
                            name: "USD".to_owned(),
                            position: CommodityPosition::Left,
                        }
                    })),
                    status: Some(TransactionStatus::Cleared),
                    comment: Some("asdf".to_owned()),
                }
            ),
            "* Assets:Checking  USD42.00 = USD50.00\n  ; asdf"
        );
    }

    #[test]
    fn display_transaction() {
        let actual = format!(
            "{}",
            Transaction {
                comment: Some("Comment Line 1\nComment Line 2".to_owned()),
                date: NaiveDate::from_ymd(2018, 10, 01),
                effective_date: Some(NaiveDate::from_ymd(2018, 10, 14)),
                status: Some(TransactionStatus::Pending),
                code: Some("123".to_owned()),
                description: "Marek Ogarek".to_owned(),
                postings: vec![
                    Posting {
                        account: "TEST:ABC 123".to_owned(),
                        reality: Reality::Real,
                        amount: Some(PostingAmount {
                            amount: Amount {
                                quantity: Decimal::new(120, 2),
                                commodity: Commodity {
                                    name: "$".to_owned(),
                                    position: CommodityPosition::Left
                                }
                            },
                            lot_price: None,
                            price: None
                        }),
                        balance: None,
                        status: None,
                        comment: Some("dd".to_owned())
                    },
                    Posting {
                        account: "TEST:ABC 123".to_owned(),
                        reality: Reality::Real,
                        amount: Some(PostingAmount {
                            amount: Amount {
                                quantity: Decimal::new(120, 2),
                                commodity: Commodity {
                                    name: "$".to_owned(),
                                    position: CommodityPosition::Left
                                }
                            },
                            lot_price: None,
                            price: None
                        }),
                        balance: None,
                        status: None,
                        comment: None
                    }
                ]
            },
        );
        let expected = r#"2018-10-01=2018-10-14 ! (123) Marek Ogarek
  ; Comment Line 1
  ; Comment Line 2
  TEST:ABC 123  $1.20
  ; dd
  TEST:ABC 123  $1.20"#;
        assert_eq!(actual, expected);
    }

    #[test]
    fn display_ledger() {
        let actual = format!(
            "{}",
            Ledger {
                items: vec![
                    LedgerItem::Transaction(Transaction {
                        comment: Some("Comment Line 1\nComment Line 2".to_owned()),
                        date: NaiveDate::from_ymd(2018, 10, 01),
                        effective_date: Some(NaiveDate::from_ymd(2018, 10, 14)),
                        status: Some(TransactionStatus::Pending),
                        code: Some("123".to_owned()),
                        description: "Marek Ogarek".to_owned(),
                        postings: vec![
                            Posting {
                                account: "TEST:ABC 123".to_owned(),
                                reality: Reality::Real,
                                amount: Some(PostingAmount {
                                    amount: Amount {
                                        quantity: Decimal::new(120, 2),
                                        commodity: Commodity {
                                            name: "$".to_owned(),
                                            position: CommodityPosition::Left
                                        }
                                    },
                                    lot_price: None,
                                    price: None
                                }),
                                balance: None,
                                status: None,
                                comment: Some("dd".to_owned())
                            },
                            Posting {
                                account: "TEST:ABC 123".to_owned(),
                                reality: Reality::Real,
                                amount: Some(PostingAmount {
                                    amount: Amount {
                                        quantity: Decimal::new(120, 2),
                                        commodity: Commodity {
                                            name: "$".to_owned(),
                                            position: CommodityPosition::Left
                                        }
                                    },
                                    lot_price: None,
                                    price: None
                                }),
                                balance: None,
                                status: None,
                                comment: None
                            }
                        ]
                    }),
                    LedgerItem::EmptyLine,
                    LedgerItem::Transaction(Transaction {
                        comment: None,
                        date: NaiveDate::from_ymd(2018, 10, 01),
                        effective_date: Some(NaiveDate::from_ymd(2018, 10, 14)),
                        status: Some(TransactionStatus::Pending),
                        code: Some("123".to_owned()),
                        description: "Marek Ogarek".to_owned(),
                        postings: vec![
                            Posting {
                                account: "TEST:ABC 123".to_owned(),
                                reality: Reality::Real,
                                amount: Some(PostingAmount {
                                    amount: Amount {
                                        quantity: Decimal::new(120, 2),
                                        commodity: Commodity {
                                            name: "$".to_owned(),
                                            position: CommodityPosition::Left
                                        }
                                    },
                                    lot_price: Some(Price::Unit(Amount {
                                        quantity: Decimal::new(500, 2),
                                        commodity: Commodity {
                                            name: "PLN".to_owned(),
                                            position: CommodityPosition::Right
                                        }
                                    })),
                                    price: Some(Price::Unit(Amount {
                                        quantity: Decimal::new(600, 2),
                                        commodity: Commodity {
                                            name: "PLN".to_owned(),
                                            position: CommodityPosition::Right
                                        }
                                    }))
                                }),
                                balance: None,
                                status: None,
                                comment: None
                            },
                            Posting {
                                account: "TEST:ABC 123".to_owned(),
                                reality: Reality::Real,
                                amount: Some(PostingAmount {
                                    amount: Amount {
                                        quantity: Decimal::new(120, 2),
                                        commodity: Commodity {
                                            name: "$".to_owned(),
                                            position: CommodityPosition::Left
                                        }
                                    },
                                    lot_price: Some(Price::Total(Amount {
                                        quantity: Decimal::new(500, 2),
                                        commodity: Commodity {
                                            name: "PLN".to_owned(),
                                            position: CommodityPosition::Right
                                        }
                                    })),
                                    price: Some(Price::Total(Amount {
                                        quantity: Decimal::new(600, 2),
                                        commodity: Commodity {
                                            name: "PLN".to_owned(),
                                            position: CommodityPosition::Right
                                        }
                                    }))
                                }),
                                balance: None,
                                status: None,
                                comment: None
                            }
                        ]
                    }),
                    LedgerItem::EmptyLine,
                    LedgerItem::CommodityPrice(CommodityPrice {
                        datetime: NaiveDate::from_ymd(2017, 11, 12).and_hms(12, 00, 00),
                        commodity_name: "mBH".to_owned(),
                        amount: Amount {
                            quantity: Decimal::new(500, 2),
                            commodity: Commodity {
                                name: "PLN".to_owned(),
                                position: CommodityPosition::Right
                            }
                        }
                    }),
                ]
            }
        );
        let expected = r#"2018-10-01=2018-10-14 ! (123) Marek Ogarek
  ; Comment Line 1
  ; Comment Line 2
  TEST:ABC 123  $1.20
  ; dd
  TEST:ABC 123  $1.20

2018-10-01=2018-10-14 ! (123) Marek Ogarek
  TEST:ABC 123  $1.20 {5.00 PLN} @ 6.00 PLN
  TEST:ABC 123  $1.20 {{5.00 PLN}} @@ 6.00 PLN

P 2017-11-12 12:00:00 mBH 5.00 PLN
"#;
        assert_eq!(actual, expected);
    }
}
