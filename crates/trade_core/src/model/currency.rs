// Supporting G20 Currency Codes only
// We could move this to a config later

use serde::{Deserialize, Serialize};
/// Allow for conversion between currency codes and their string representations
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum Currency {
    #[strum(serialize = "ARS")]
    ARS, // Argentine Peso
    #[strum(serialize = "AUD")]
    AUD, // Australian Dollar
    #[strum(serialize = "BRL")]
    BRL, // Brazilian Real
    #[strum(serialize = "CAD")]
    CAD, // Canadian Dollar
    #[strum(serialize = "CNY")]
    CNY, // Chinese Yuan
    #[strum(serialize = "EUR")]
    EUR, // Euro
    #[strum(serialize = "INR")]
    INR, // Indian Rupee
    #[strum(serialize = "IDR")]
    IDR, // Indonesian Rupiah
    #[strum(serialize = "JPY")]
    JPY, // Japanese Yen
    #[strum(serialize = "KRW")]
    KRW, // South Korean Won
    #[strum(serialize = "MXN")]
    MXN, // Mexican Peso
    #[strum(serialize = "RUB")]
    RUB, // Russian Ruble
    #[strum(serialize = "SAR")]
    SAR, // Saudi Riyal
    #[strum(serialize = "ZAR")]
    ZAR, // South African Rand
    #[strum(serialize = "TRY")]
    TRY, // Turkish Lira
    #[strum(serialize = "GBP")]
    GBP, // British Pound Sterling
    #[strum(serialize = "USD")]
    USD, // US Dollar
}

impl Currency {
    fn name(&self) -> &'static str {
        match self {
            Currency::ARS => "Argentine Peso",
            Currency::AUD => "Australian Dollar",
            Currency::BRL => "Brazilian Real",
            Currency::CAD => "Canadian Dollar",
            Currency::CNY => "Chinese Yuan",
            Currency::EUR => "Euro",
            Currency::INR => "Indian Rupee",
            Currency::IDR => "Indonesian Rupiah",
            Currency::JPY => "Japanese Yen",
            Currency::KRW => "South Korean Won",
            Currency::MXN => "Mexican Peso",
            Currency::RUB => "Russian Ruble",
            Currency::SAR => "Saudi Riyal",
            Currency::ZAR => "South African Rand",
            Currency::TRY => "Turkish Lira",
            Currency::GBP => "British Pound Sterling",
            Currency::USD => "US Dollar",
        }
    }

    fn numeric_code(&self) -> u16 {
        match self {
            Currency::ARS => 32,
            Currency::AUD => 36,
            Currency::BRL => 986,
            Currency::CAD => 124,
            Currency::CNY => 156,
            Currency::EUR => 978,
            Currency::INR => 356,
            Currency::IDR => 360,
            Currency::JPY => 392,
            Currency::KRW => 410,
            Currency::MXN => 484,
            Currency::RUB => 643,
            Currency::SAR => 682,
            Currency::ZAR => 710,
            Currency::TRY => 949,
            Currency::GBP => 826,
            Currency::USD => 840,
        }
    }
}
