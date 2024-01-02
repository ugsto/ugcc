use strum::EnumIter;

use crate::errors::CurrencyCodeError;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, EnumIter)]
pub enum CurrencyCode {
    Aed,
    Afn,
    All,
    Amd,
    Ang,
    Aoa,
    Ars,
    Aud,
    Awg,
    Azn,
    Bam,
    Bbd,
    Bdt,
    Bgn,
    Bhd,
    Bif,
    Bmd,
    Bnd,
    Bob,
    Bov,
    Brl,
    Bsd,
    Btn,
    Bwp,
    Byn,
    Bzd,
    Cad,
    Cdf,
    Che,
    Chf,
    Chw,
    Clf,
    Clp,
    Cny,
    Cop,
    Cou,
    Crc,
    Cup,
    Cve,
    Czk,
    Djf,
    Dkk,
    Dop,
    Dzd,
    Egp,
    Ern,
    Etb,
    Eur,
    Fjd,
    Fkp,
    Gbp,
    Gel,
    Ghs,
    Gip,
    Gmd,
    Gnf,
    Gtq,
    Gyd,
    Hkd,
    Hnl,
    Htg,
    Huf,
    Idr,
    Ils,
    Inr,
    Iqd,
    Irr,
    Isk,
    Jmd,
    Jod,
    Jpy,
    Kes,
    Kgs,
    Khr,
    Kmf,
    Kpw,
    Krw,
    Kwd,
    Kyd,
    Kzt,
    Lak,
    Lbp,
    Lkr,
    Lrd,
    Lsl,
    Lyd,
    Mad,
    Mdl,
    Mga,
    Mkd,
    Mmk,
    Mnt,
    Mop,
    Mru,
    Mur,
    Mvr,
    Mwk,
    Mxn,
    Mxv,
    Myr,
    Mzn,
    Nad,
    Ngn,
    Nio,
    Nok,
    Npr,
    Nzd,
    Omr,
    Pab,
    Pen,
    Pgk,
    Php,
    Pkr,
    Pln,
    Pyg,
    Qar,
    Ron,
    Rsd,
    Rub,
    Rwf,
    Sar,
    Sbd,
    Scr,
    Sdg,
    Sek,
    Sgd,
    Shp,
    Sle,
    Sll,
    Sos,
    Srd,
    Ssp,
    Stn,
    Svc,
    Syp,
    Szl,
    Thb,
    Tjs,
    Tmt,
    Tnd,
    Top,
    Try,
    Ttd,
    Twd,
    Tzs,
    Uah,
    Ugx,
    Usd,
    Usn,
    Uyi,
    Uyu,
    Uyw,
    Uzs,
    Ved,
    Ves,
    Vnd,
    Vuv,
    Wst,
    Xaf,
    Xag,
    Xau,
    Xba,
    Xbb,
    Xbc,
    Xbd,
    Xcd,
    Xdr,
    Xof,
    Xpd,
    Xpf,
    Xpt,
    Xsu,
    Xts,
    Xua,
    Xxx,
    Yer,
    YerAden,
    Zar,
    Zmw,
    Zwl,
}

impl FromStr for CurrencyCode {
    type Err = CurrencyCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AED" => Ok(Self::Aed),
            "AFN" => Ok(Self::Afn),
            "ALL" => Ok(Self::All),
            "AMD" => Ok(Self::Amd),
            "ANG" => Ok(Self::Ang),
            "AOA" => Ok(Self::Aoa),
            "ARS" => Ok(Self::Ars),
            "AUD" => Ok(Self::Aud),
            "AWG" => Ok(Self::Awg),
            "AZN" => Ok(Self::Azn),
            "BAM" => Ok(Self::Bam),
            "BBD" => Ok(Self::Bbd),
            "BDT" => Ok(Self::Bdt),
            "BGN" => Ok(Self::Bgn),
            "BHD" => Ok(Self::Bhd),
            "BIF" => Ok(Self::Bif),
            "BMD" => Ok(Self::Bmd),
            "BND" => Ok(Self::Bnd),
            "BOB" => Ok(Self::Bob),
            "BOV" => Ok(Self::Bov),
            "BRL" => Ok(Self::Brl),
            "BSD" => Ok(Self::Bsd),
            "BTN" => Ok(Self::Btn),
            "BWP" => Ok(Self::Bwp),
            "BYN" => Ok(Self::Byn),
            "BZD" => Ok(Self::Bzd),
            "CAD" => Ok(Self::Cad),
            "CDF" => Ok(Self::Cdf),
            "CHE" => Ok(Self::Che),
            "CHF" => Ok(Self::Chf),
            "CHW" => Ok(Self::Chw),
            "CLF" => Ok(Self::Clf),
            "CLP" => Ok(Self::Clp),
            "CNY" => Ok(Self::Cny),
            "COP" => Ok(Self::Cop),
            "COU" => Ok(Self::Cou),
            "CRC" => Ok(Self::Crc),
            "CUP" => Ok(Self::Cup),
            "CVE" => Ok(Self::Cve),
            "CZK" => Ok(Self::Czk),
            "DJF" => Ok(Self::Djf),
            "DKK" => Ok(Self::Dkk),
            "DOP" => Ok(Self::Dop),
            "DZD" => Ok(Self::Dzd),
            "EGP" => Ok(Self::Egp),
            "ERN" => Ok(Self::Ern),
            "ETB" => Ok(Self::Etb),
            "EUR" => Ok(Self::Eur),
            "FJD" => Ok(Self::Fjd),
            "FKP" => Ok(Self::Fkp),
            "GBP" => Ok(Self::Gbp),
            "GEL" => Ok(Self::Gel),
            "GHS" => Ok(Self::Ghs),
            "GIP" => Ok(Self::Gip),
            "GMD" => Ok(Self::Gmd),
            "GNF" => Ok(Self::Gnf),
            "GTQ" => Ok(Self::Gtq),
            "GYD" => Ok(Self::Gyd),
            "HKD" => Ok(Self::Hkd),
            "HNL" => Ok(Self::Hnl),
            "HTG" => Ok(Self::Htg),
            "HUF" => Ok(Self::Huf),
            "IDR" => Ok(Self::Idr),
            "ILS" => Ok(Self::Ils),
            "INR" => Ok(Self::Inr),
            "IQD" => Ok(Self::Iqd),
            "IRR" => Ok(Self::Irr),
            "ISK" => Ok(Self::Isk),
            "JMD" => Ok(Self::Jmd),
            "JOD" => Ok(Self::Jod),
            "JPY" => Ok(Self::Jpy),
            "KES" => Ok(Self::Kes),
            "KGS" => Ok(Self::Kgs),
            "KHR" => Ok(Self::Khr),
            "KMF" => Ok(Self::Kmf),
            "KPW" => Ok(Self::Kpw),
            "KRW" => Ok(Self::Krw),
            "KWD" => Ok(Self::Kwd),
            "KYD" => Ok(Self::Kyd),
            "KZT" => Ok(Self::Kzt),
            "LAK" => Ok(Self::Lak),
            "LBP" => Ok(Self::Lbp),
            "LKR" => Ok(Self::Lkr),
            "LRD" => Ok(Self::Lrd),
            "LSL" => Ok(Self::Lsl),
            "LYD" => Ok(Self::Lyd),
            "MAD" => Ok(Self::Mad),
            "MDL" => Ok(Self::Mdl),
            "MGA" => Ok(Self::Mga),
            "MKD" => Ok(Self::Mkd),
            "MMK" => Ok(Self::Mmk),
            "MNT" => Ok(Self::Mnt),
            "MOP" => Ok(Self::Mop),
            "MRU" => Ok(Self::Mru),
            "MUR" => Ok(Self::Mur),
            "MVR" => Ok(Self::Mvr),
            "MWK" => Ok(Self::Mwk),
            "MXN" => Ok(Self::Mxn),
            "MXV" => Ok(Self::Mxv),
            "MYR" => Ok(Self::Myr),
            "MZN" => Ok(Self::Mzn),
            "NAD" => Ok(Self::Nad),
            "NGN" => Ok(Self::Ngn),
            "NIO" => Ok(Self::Nio),
            "NOK" => Ok(Self::Nok),
            "NPR" => Ok(Self::Npr),
            "NZD" => Ok(Self::Nzd),
            "OMR" => Ok(Self::Omr),
            "PAB" => Ok(Self::Pab),
            "PEN" => Ok(Self::Pen),
            "PGK" => Ok(Self::Pgk),
            "PHP" => Ok(Self::Php),
            "PKR" => Ok(Self::Pkr),
            "PLN" => Ok(Self::Pln),
            "PYG" => Ok(Self::Pyg),
            "QAR" => Ok(Self::Qar),
            "RON" => Ok(Self::Ron),
            "RSD" => Ok(Self::Rsd),
            "RUB" => Ok(Self::Rub),
            "RWF" => Ok(Self::Rwf),
            "SAR" => Ok(Self::Sar),
            "SBD" => Ok(Self::Sbd),
            "SCR" => Ok(Self::Scr),
            "SDG" => Ok(Self::Sdg),
            "SEK" => Ok(Self::Sek),
            "SGD" => Ok(Self::Sgd),
            "SHP" => Ok(Self::Shp),
            "SLE" => Ok(Self::Sle),
            "SLL" => Ok(Self::Sll),
            "SOS" => Ok(Self::Sos),
            "SRD" => Ok(Self::Srd),
            "SSP" => Ok(Self::Ssp),
            "STN" => Ok(Self::Stn),
            "SVC" => Ok(Self::Svc),
            "SYP" => Ok(Self::Syp),
            "SZL" => Ok(Self::Szl),
            "THB" => Ok(Self::Thb),
            "TJS" => Ok(Self::Tjs),
            "TMT" => Ok(Self::Tmt),
            "TND" => Ok(Self::Tnd),
            "TOP" => Ok(Self::Top),
            "TRY" => Ok(Self::Try),
            "TTD" => Ok(Self::Ttd),
            "TWD" => Ok(Self::Twd),
            "TZS" => Ok(Self::Tzs),
            "UAH" => Ok(Self::Uah),
            "UGX" => Ok(Self::Ugx),
            "USD" => Ok(Self::Usd),
            "USN" => Ok(Self::Usn),
            "UYI" => Ok(Self::Uyi),
            "UYU" => Ok(Self::Uyu),
            "UYW" => Ok(Self::Uyw),
            "UZS" => Ok(Self::Uzs),
            "VED" => Ok(Self::Ved),
            "VES" => Ok(Self::Ves),
            "VND" => Ok(Self::Vnd),
            "VUV" => Ok(Self::Vuv),
            "WST" => Ok(Self::Wst),
            "XAF" => Ok(Self::Xaf),
            "XAG" => Ok(Self::Xag),
            "XAU" => Ok(Self::Xau),
            "XBA" => Ok(Self::Xba),
            "XBB" => Ok(Self::Xbb),
            "XBC" => Ok(Self::Xbc),
            "XBD" => Ok(Self::Xbd),
            "XCD" => Ok(Self::Xcd),
            "XDR" => Ok(Self::Xdr),
            "XOF" => Ok(Self::Xof),
            "XPD" => Ok(Self::Xpd),
            "XPF" => Ok(Self::Xpf),
            "XPT" => Ok(Self::Xpt),
            "XSU" => Ok(Self::Xsu),
            "XTS" => Ok(Self::Xts),
            "XUA" => Ok(Self::Xua),
            "XXX" => Ok(Self::Xxx),
            "YER" => Ok(Self::Yer),
            "YER ADEN" => Ok(Self::YerAden),
            "ZAR" => Ok(Self::Zar),
            "ZMW" => Ok(Self::Zmw),
            "ZWL" => Ok(Self::Zwl),
            _ => Err(CurrencyCodeError::InvalidCurrencyCode(s.to_string())),
        }
    }
}

impl From<CurrencyCode> for &'static str {
    fn from(value: CurrencyCode) -> Self {
        match value {
            CurrencyCode::Aed => "AED",
            CurrencyCode::Afn => "AFN",
            CurrencyCode::All => "ALL",
            CurrencyCode::Amd => "AMD",
            CurrencyCode::Ang => "ANG",
            CurrencyCode::Aoa => "AOA",
            CurrencyCode::Ars => "ARS",
            CurrencyCode::Aud => "AUD",
            CurrencyCode::Awg => "AWG",
            CurrencyCode::Azn => "AZN",
            CurrencyCode::Bam => "BAM",
            CurrencyCode::Bbd => "BBD",
            CurrencyCode::Bdt => "BDT",
            CurrencyCode::Bgn => "BGN",
            CurrencyCode::Bhd => "BHD",
            CurrencyCode::Bif => "BIF",
            CurrencyCode::Bmd => "BMD",
            CurrencyCode::Bnd => "BND",
            CurrencyCode::Bob => "BOB",
            CurrencyCode::Bov => "BOV",
            CurrencyCode::Brl => "BRL",
            CurrencyCode::Bsd => "BSD",
            CurrencyCode::Btn => "BTN",
            CurrencyCode::Bwp => "BWP",
            CurrencyCode::Byn => "BYN",
            CurrencyCode::Bzd => "BZD",
            CurrencyCode::Cad => "CAD",
            CurrencyCode::Cdf => "CDF",
            CurrencyCode::Che => "CHE",
            CurrencyCode::Chf => "CHF",
            CurrencyCode::Chw => "CHW",
            CurrencyCode::Clf => "CLF",
            CurrencyCode::Clp => "CLP",
            CurrencyCode::Cny => "CNY",
            CurrencyCode::Cop => "COP",
            CurrencyCode::Cou => "COU",
            CurrencyCode::Crc => "CRC",
            CurrencyCode::Cup => "CUP",
            CurrencyCode::Cve => "CVE",
            CurrencyCode::Czk => "CZK",
            CurrencyCode::Djf => "DJF",
            CurrencyCode::Dkk => "DKK",
            CurrencyCode::Dop => "DOP",
            CurrencyCode::Dzd => "DZD",
            CurrencyCode::Egp => "EGP",
            CurrencyCode::Ern => "ERN",
            CurrencyCode::Etb => "ETB",
            CurrencyCode::Eur => "EUR",
            CurrencyCode::Fjd => "FJD",
            CurrencyCode::Fkp => "FKP",
            CurrencyCode::Gbp => "GBP",
            CurrencyCode::Gel => "GEL",
            CurrencyCode::Ghs => "GHS",
            CurrencyCode::Gip => "GIP",
            CurrencyCode::Gmd => "GMD",
            CurrencyCode::Gnf => "GNF",
            CurrencyCode::Gtq => "GTQ",
            CurrencyCode::Gyd => "GYD",
            CurrencyCode::Hkd => "HKD",
            CurrencyCode::Hnl => "HNL",
            CurrencyCode::Htg => "HTG",
            CurrencyCode::Huf => "HUF",
            CurrencyCode::Idr => "IDR",
            CurrencyCode::Ils => "ILS",
            CurrencyCode::Inr => "INR",
            CurrencyCode::Iqd => "IQD",
            CurrencyCode::Irr => "IRR",
            CurrencyCode::Isk => "ISK",
            CurrencyCode::Jmd => "JMD",
            CurrencyCode::Jod => "JOD",
            CurrencyCode::Jpy => "JPY",
            CurrencyCode::Kes => "KES",
            CurrencyCode::Kgs => "KGS",
            CurrencyCode::Khr => "KHR",
            CurrencyCode::Kmf => "KMF",
            CurrencyCode::Kpw => "KPW",
            CurrencyCode::Krw => "KRW",
            CurrencyCode::Kwd => "KWD",
            CurrencyCode::Kyd => "KYD",
            CurrencyCode::Kzt => "KZT",
            CurrencyCode::Lak => "LAK",
            CurrencyCode::Lbp => "LBP",
            CurrencyCode::Lkr => "LKR",
            CurrencyCode::Lrd => "LRD",
            CurrencyCode::Lsl => "LSL",
            CurrencyCode::Lyd => "LYD",
            CurrencyCode::Mad => "MAD",
            CurrencyCode::Mdl => "MDL",
            CurrencyCode::Mga => "MGA",
            CurrencyCode::Mkd => "MKD",
            CurrencyCode::Mmk => "MMK",
            CurrencyCode::Mnt => "MNT",
            CurrencyCode::Mop => "MOP",
            CurrencyCode::Mru => "MRU",
            CurrencyCode::Mur => "MUR",
            CurrencyCode::Mvr => "MVR",
            CurrencyCode::Mwk => "MWK",
            CurrencyCode::Mxn => "MXN",
            CurrencyCode::Mxv => "MXV",
            CurrencyCode::Myr => "MYR",
            CurrencyCode::Mzn => "MZN",
            CurrencyCode::Nad => "NAD",
            CurrencyCode::Ngn => "NGN",
            CurrencyCode::Nio => "NIO",
            CurrencyCode::Nok => "NOK",
            CurrencyCode::Npr => "NPR",
            CurrencyCode::Nzd => "NZD",
            CurrencyCode::Omr => "OMR",
            CurrencyCode::Pab => "PAB",
            CurrencyCode::Pen => "PEN",
            CurrencyCode::Pgk => "PGK",
            CurrencyCode::Php => "PHP",
            CurrencyCode::Pkr => "PKR",
            CurrencyCode::Pln => "PLN",
            CurrencyCode::Pyg => "PYG",
            CurrencyCode::Qar => "QAR",
            CurrencyCode::Ron => "RON",
            CurrencyCode::Rsd => "RSD",
            CurrencyCode::Rub => "RUB",
            CurrencyCode::Rwf => "RWF",
            CurrencyCode::Sar => "SAR",
            CurrencyCode::Sbd => "SBD",
            CurrencyCode::Scr => "SCR",
            CurrencyCode::Sdg => "SDG",
            CurrencyCode::Sek => "SEK",
            CurrencyCode::Sgd => "SGD",
            CurrencyCode::Shp => "SHP",
            CurrencyCode::Sle => "SLE",
            CurrencyCode::Sll => "SLL",
            CurrencyCode::Sos => "SOS",
            CurrencyCode::Srd => "SRD",
            CurrencyCode::Ssp => "SSP",
            CurrencyCode::Stn => "STN",
            CurrencyCode::Svc => "SVC",
            CurrencyCode::Syp => "SYP",
            CurrencyCode::Szl => "SZL",
            CurrencyCode::Thb => "THB",
            CurrencyCode::Tjs => "TJS",
            CurrencyCode::Tmt => "TMT",
            CurrencyCode::Tnd => "TND",
            CurrencyCode::Top => "TOP",
            CurrencyCode::Try => "TRY",
            CurrencyCode::Ttd => "TTD",
            CurrencyCode::Twd => "TWD",
            CurrencyCode::Tzs => "TZS",
            CurrencyCode::Uah => "UAH",
            CurrencyCode::Ugx => "UGX",
            CurrencyCode::Usd => "USD",
            CurrencyCode::Usn => "USN",
            CurrencyCode::Uyi => "UYI",
            CurrencyCode::Uyu => "UYU",
            CurrencyCode::Uyw => "UYW",
            CurrencyCode::Uzs => "UZS",
            CurrencyCode::Ved => "VED",
            CurrencyCode::Ves => "VES",
            CurrencyCode::Vnd => "VND",
            CurrencyCode::Vuv => "VUV",
            CurrencyCode::Wst => "WST",
            CurrencyCode::Xaf => "XAF",
            CurrencyCode::Xag => "XAG",
            CurrencyCode::Xau => "XAU",
            CurrencyCode::Xba => "XBA",
            CurrencyCode::Xbb => "XBB",
            CurrencyCode::Xbc => "XBC",
            CurrencyCode::Xbd => "XBD",
            CurrencyCode::Xcd => "XCD",
            CurrencyCode::Xdr => "XDR",
            CurrencyCode::Xof => "XOF",
            CurrencyCode::Xpd => "XPD",
            CurrencyCode::Xpf => "XPF",
            CurrencyCode::Xpt => "XPT",
            CurrencyCode::Xsu => "XSU",
            CurrencyCode::Xts => "XTS",
            CurrencyCode::Xua => "XUA",
            CurrencyCode::Xxx => "XXX",
            CurrencyCode::Yer => "YER",
            CurrencyCode::YerAden => "YER ADEN",
            CurrencyCode::Zar => "ZAR",
            CurrencyCode::Zmw => "ZMW",
            CurrencyCode::Zwl => "ZWL",
        }
    }
}