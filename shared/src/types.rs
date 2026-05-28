use serde::{Deserialize, Serialize};
use std::fmt;

/// All supported currency codes (FR-6.1).
pub const SUPPORTED_CURRENCIES: &[&str] =
    &["CNY", "USD", "EUR", "JPY", "GBP", "HKD", "KRW", "AUD", "CAD"];

/// Returns true if `code` is one of the supported currencies.
pub fn is_valid_currency(code: &str) -> bool {
    SUPPORTED_CURRENCIES.contains(&code)
}

/// Budget period discriminant (FR-5.3).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Period {
    Week,
    Month,
    Year,
}

impl Period {
    pub fn as_str(&self) -> &'static str {
        match self {
            Period::Week  => "week",
            Period::Month => "month",
            Period::Year  => "year",
        }
    }
}

impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<&str> for Period {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "week"  => Ok(Period::Week),
            "month" => Ok(Period::Month),
            "year"  => Ok(Period::Year),
            other   => Err(format!("unknown period: {other}")),
        }
    }
}

/// Transaction type discriminant (FR-3.1).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Income,
    Expense,
    Transfer,
}

impl TransactionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionType::Income   => "income",
            TransactionType::Expense  => "expense",
            TransactionType::Transfer => "transfer",
        }
    }
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<&str> for TransactionType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "income"   => Ok(TransactionType::Income),
            "expense"  => Ok(TransactionType::Expense),
            "transfer" => Ok(TransactionType::Transfer),
            other      => Err(format!("unknown transaction type: {other}")),
        }
    }
}

/// Category type discriminant (FR-4.1).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CategoryType {
    Income,
    Expense,
}

impl CategoryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CategoryType::Income  => "income",
            CategoryType::Expense => "expense",
        }
    }
}
