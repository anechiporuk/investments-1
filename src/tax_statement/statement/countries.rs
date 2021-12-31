use crate::core::{EmptyResult, GenericResult};

use super::encoding::TaxStatementType;
use super::parser::{TaxStatementReader, TaxStatementWriter};
use super::types::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountryCode {
    Russia,
    Usa,
    Other(Integer),
}

impl CountryCode {
    pub fn new(country: &str) -> GenericResult<CountryCode> {
        Ok(CountryCode::from_code(get_code(country)?.into()))
    }

    fn from_code(code: Integer) -> CountryCode {
        match code {
            643 => CountryCode::Russia,
            840 => CountryCode::Usa,
            code => CountryCode::Other(code),
        }
    }

    fn to_code(self) -> Integer {
        match self {
            CountryCode::Russia => 643,
            CountryCode::Usa => 840,
            CountryCode::Other(code) => code,
        }
    }
}

impl TaxStatementType for CountryCode {
    fn read(reader: &mut TaxStatementReader) -> GenericResult<CountryCode> {
        Ok(CountryCode::from_code(reader.read_value()?))
    }

    fn write(&self, writer: &mut TaxStatementWriter) -> EmptyResult {
        writer.write_value(&self.to_code())?;
        Ok(())
    }
}

fn get_code(country: &str) -> GenericResult<u16> {
    Ok(match country {
        // https://ru.wikipedia.org/wiki/Общероссийский_классификатор_стран_мира
        // cat countries | sed -r 's/^.*\.svg\s+//' | awk -F '\t' '{ printf "\"%s\" => %3d, // %s\n", $3, $5, $1 }'
        "AB" => 895, // АБХАЗИЯ
        "AU" =>  36, // АВСТРАЛИЯ
        "AT" =>  40, // АВСТРИЯ
        "AZ" =>  31, // АЗЕРБАЙДЖАН
        "AL" =>   8, // АЛБАНИЯ
        "DZ" =>  12, // АЛЖИР
        "AS" =>  16, // АМЕРИКАНСКОЕ САМОА
        "AI" => 660, // АНГИЛЬЯ
        "AO" =>  24, // АНГОЛА
        "AD" =>  20, // АНДОРРА
        "AQ" =>  10, // АНТАРКТИДА
        "AG" =>  28, // АНТИГУА И БАРБУДА
        "AR" =>  32, // АРГЕНТИНА
        "AM" =>  51, // АРМЕНИЯ
        "AW" => 533, // АРУБА
        "AF" =>   4, // АФГАНИСТАН
        "BS" =>  44, // БАГАМЫ
        "BD" =>  50, // БАНГЛАДЕШ
        "BB" =>  52, // БАРБАДОС
        "BH" =>  48, // БАХРЕЙН
        "BY" => 112, // БЕЛАРУСЬ
        "BZ" =>  84, // БЕЛИЗ
        "BE" =>  56, // БЕЛЬГИЯ
        "BJ" => 204, // БЕНИН
        "BM" =>  60, // БЕРМУДЫ
        "BG" => 100, // БОЛГАРИЯ
        "BO" =>  68, // БОЛИВИЯ, МНОГОНАЦИОНАЛЬНОЕ ГОСУДАРСТВО
        "BQ" => 535, // БОНЭЙР, СИНТ-ЭСТАТИУС И САБА
        "BA" =>  70, // БОСНИЯ И ГЕРЦЕГОВИНА
        "BW" =>  72, // БОТСВАНА
        "BR" =>  76, // БРАЗИЛИЯ
        "IO" =>  86, // БРИТАНСКАЯ ТЕРРИТОРИЯ В ИНДИЙСКОМ ОКЕАНЕ
        "BN" =>  96, // БРУНЕЙ-ДАРУССАЛАМ
        "BF" => 854, // БУРКИНА-ФАСО
        "BI" => 108, // БУРУНДИ
        "BT" =>  64, // БУТАН
        "VU" => 548, // ВАНУАТУ
        "HU" => 348, // ВЕНГРИЯ
        "VE" => 862, // ВЕНЕСУЭЛА (БОЛИВАРИАНСКАЯ РЕСПУБЛИКА)
        "VG" =>  92, // ВИРГИНСКИЕ ОСТРОВА, БРИТАНСКИЕ
        "VI" => 850, // ВИРГИНСКИЕ ОСТРОВА, США
        "VN" => 704, // ВЬЕТНАМ
        "GA" => 266, // ГАБОН
        "HT" => 332, // ГАИТИ
        "GY" => 328, // ГАЙАНА
        "GM" => 270, // ГАМБИЯ
        "GH" => 288, // ГАНА
        "GP" => 312, // ГВАДЕЛУПА
        "GT" => 320, // ГВАТЕМАЛА
        "GN" => 324, // ГВИНЕЯ
        "GW" => 624, // ГВИНЕЯ-БИСАУ
        "DE" => 276, // ГЕРМАНИЯ
        "GG" => 831, // ГЕРНСИ
        "GI" => 292, // ГИБРАЛТАР
        "HN" => 340, // ГОНДУРАС
        "HK" => 344, // ГОНКОНГ
        "GD" => 308, // ГРЕНАДА
        "GL" => 304, // ГРЕНЛАНДИЯ
        "GR" => 300, // ГРЕЦИЯ
        "GE" => 268, // ГРУЗИЯ
        "GU" => 316, // ГУАМ
        "DK" => 208, // ДАНИЯ
        "JE" => 832, // ДЖЕРСИ
        "DJ" => 262, // ДЖИБУТИ
        "DM" => 212, // ДОМИНИКА
        "DO" => 214, // ДОМИНИКАНСКАЯ РЕСПУБЛИКА
        "EG" => 818, // ЕГИПЕТ
        "ZM" => 894, // ЗАМБИЯ
        "EH" => 732, // ЗАПАДНАЯ САХАРА
        "ZW" => 716, // ЗИМБАБВЕ
        "IL" => 376, // ИЗРАИЛЬ
        "IN" => 356, // ИНДИЯ
        "ID" => 360, // ИНДОНЕЗИЯ
        "JO" => 400, // ИОРДАНИЯ
        "IQ" => 368, // ИРАК
        "IR" => 364, // ИРАН (ИСЛАМСКАЯ РЕСПУБЛИКА)
        "IE" => 372, // ИРЛАНДИЯ
        "IS" => 352, // ИСЛАНДИЯ
        "ES" => 724, // ИСПАНИЯ
        "IT" => 380, // ИТАЛИЯ
        "YE" => 887, // ЙЕМЕН
        "CV" => 132, // КАБО-ВЕРДЕ
        "KZ" => 398, // КАЗАХСТАН
        "KH" => 116, // КАМБОДЖА
        "CM" => 120, // КАМЕРУН
        "CA" => 124, // КАНАДА
        "QA" => 634, // КАТАР
        "KE" => 404, // КЕНИЯ
        "CY" => 196, // КИПР
        "KG" => 417, // КИРГИЗИЯ
        "KI" => 296, // КИРИБАТИ
        "CN" => 156, // КИТАЙ
        "CC" => 166, // КОКОСОВЫЕ (КИЛИНГ) ОСТРОВА
        "CO" => 170, // КОЛУМБИЯ
        "KM" => 174, // КОМОРЫ
        "CG" => 178, // КОНГО
        "CD" => 180, // КОНГО, ДЕМОКРАТИЧЕСКАЯ РЕСПУБЛИКА
        "KP" => 408, // КОРЕЯ, НАРОДНО-ДЕМОКРАТИЧЕСКАЯ РЕСПУБЛИКА
        "KR" => 410, // КОРЕЯ, РЕСПУБЛИКА
        "CR" => 188, // КОСТА-РИКА
        "CI" => 384, // КОТ Д’ИВУАР
        "CU" => 192, // КУБА
        "KW" => 414, // КУВЕЙТ
        "CW" => 531, // КЮРАСАО
        "LA" => 418, // ЛАОССКАЯ НАРОДНО-ДЕМОКРАТИЧЕСКАЯ РЕСПУБЛИКА
        "LV" => 428, // ЛАТВИЯ
        "LS" => 426, // ЛЕСОТО
        "LR" => 430, // ЛИБЕРИЯ
        "LB" => 422, // ЛИВАН
        "LY" => 434, // ЛИВИЯ
        "LT" => 440, // ЛИТВА
        "LI" => 438, // ЛИХТЕНШТЕЙН
        "LU" => 442, // ЛЮКСЕМБУРГ
        "MU" => 480, // МАВРИКИЙ
        "MR" => 478, // МАВРИТАНИЯ
        "MG" => 450, // МАДАГАСКАР
        "YT" => 175, // МАЙОТТА
        "MO" => 446, // МАКАО
        "MW" => 454, // МАЛАВИ
        "MY" => 458, // МАЛАЙЗИЯ
        "ML" => 466, // МАЛИ
        "UM" => 581, // МАЛЫЕ ТИХООКЕАНСКИЕ ОТДАЛЕННЫЕ ОСТРОВА СОЕДИНЕННЫХ ШТАТОВ
        "MV" => 462, // МАЛЬДИВЫ
        "MT" => 470, // МАЛЬТА
        "MA" => 504, // МАРОККО
        "MQ" => 474, // МАРТИНИКА
        "MH" => 584, // МАРШАЛЛОВЫ ОСТРОВА
        "MX" => 484, // МЕКСИКА
        "FM" => 583, // МИКРОНЕЗИЯ, ФЕДЕРАТИВНЫЕ ШТАТЫ
        "MZ" => 508, // МОЗАМБИК
        "MD" => 498, // МОЛДОВА, РЕСПУБЛИКА
        "MC" => 492, // МОНАКО
        "MN" => 496, // МОНГОЛИЯ
        "MS" => 500, // МОНТСЕРРАТ
        "MM" => 104, // МЬЯНМА
        "NA" => 516, // НАМИБИЯ
        "NR" => 520, // НАУРУ
        "NP" => 524, // НЕПАЛ
        "NE" => 562, // НИГЕР
        "NG" => 566, // НИГЕРИЯ
        "NL" => 528, // НИДЕРЛАНДЫ
        "NI" => 558, // НИКАРАГУА
        "NU" => 570, // НИУЭ
        "NZ" => 554, // НОВАЯ ЗЕЛАНДИЯ
        "NC" => 540, // НОВАЯ КАЛЕДОНИЯ
        "NO" => 578, // НОРВЕГИЯ
        "AE" => 784, // ОБЪЕДИНЕННЫЕ АРАБСКИЕ ЭМИРАТЫ
        "OM" => 512, // ОМАН
        "KY" => 136, // ОСТРОВА КАЙМАН
        "CK" => 184, // ОСТРОВА КУКА
        "TC" => 796, // ОСТРОВА ТЕРКС И КАЙКОС
        "BV" =>  74, // ОСТРОВ БУВЕ
        "IM" => 833, // ОСТРОВ МЭН
        "NF" => 574, // ОСТРОВ НОРФОЛК
        "CX" => 162, // ОСТРОВ РОЖДЕСТВА
        "HM" => 334, // ОСТРОВ ХЕРД И ОСТРОВА МАКДОНАЛЬД
        "PK" => 586, // ПАКИСТАН
        "PW" => 585, // ПАЛАУ
        "PS" => 275, // ПАЛЕСТИНА, ГОСУДАРСТВО
        "PA" => 591, // ПАНАМА
        "VA" => 336, // ПАПСКИЙ ПРЕСТОЛ (ГОСУДАРСТВО — ГОРОД ВАТИКАН)
        "PG" => 598, // ПАПУА НОВАЯ ГВИНЕЯ
        "PY" => 600, // ПАРАГВАЙ
        "PE" => 604, // ПЕРУ
        "PN" => 612, // ПИТКЭРН
        "PL" => 616, // ПОЛЬША
        "PT" => 620, // ПОРТУГАЛИЯ
        "PR" => 630, // ПУЭРТО-РИКО
        "RE" => 638, // РЕЮНЬОН
        "RU" => 643, // РОССИЯ
        "RW" => 646, // РУАНДА
        "RO" => 642, // РУМЫНИЯ
        "WS" => 882, // САМОА
        "SM" => 674, // САН-МАРИНО
        "ST" => 678, // САН-ТОМЕ И ПРИНСИПИ
        "SA" => 682, // САУДОВСКАЯ АРАВИЯ
        "SH" => 654, // СВЯТАЯ ЕЛЕНА, ОСТРОВ ВОЗНЕСЕНИЯ, ТРИСТАН-ДА-КУНЬЯ
        "MK" => 807, // СЕВЕРНАЯ МАКЕДОНИЯ
        "MP" => 580, // СЕВЕРНЫЕ МАРИАНСКИЕ ОСТРОВА
        "SC" => 690, // СЕЙШЕЛЫ
        "BL" => 652, // СЕН-БАРТЕЛЕМИ
        "MF" => 663, // СЕН-МАРТЕН (французская часть)
        "SX" => 534, // СЕН-МАРТЕН (нидерландская часть)
        "SN" => 686, // СЕНЕГАЛ
        "VC" => 670, // СЕНТ-ВИНСЕНТ И ГРЕНАДИНЫ
        "KN" => 659, // СЕНТ-КИТС И НЕВИС
        "LC" => 662, // СЕНТ-ЛЮСИЯ
        "PM" => 666, // СЕН-ПЬЕР И МИКЕЛОН
        "RS" => 688, // СЕРБИЯ
        "SG" => 702, // СИНГАПУР
        "SY" => 760, // СИРИЙСКАЯ АРАБСКАЯ РЕСПУБЛИКА
        "SK" => 703, // СЛОВАКИЯ
        "SI" => 705, // СЛОВЕНИЯ
        "GB" => 826, // СОЕДИНЕННОЕ КОРОЛЕВСТВО
        "US" => 840, // СОЕДИНЕННЫЕ ШТАТЫ
        "SB" =>  90, // СОЛОМОНОВЫ ОСТРОВА
        "SO" => 706, // СОМАЛИ
        "SD" => 729, // СУДАН
        "SR" => 740, // СУРИНАМ
        "SL" => 694, // СЬЕРРА-ЛЕОНЕ
        "TJ" => 762, // ТАДЖИКИСТАН
        "TH" => 764, // ТАИЛАНД
        "TW" => 158, // ТАЙВАНЬ (КИТАЙ)
        "TZ" => 834, // ТАНЗАНИЯ, ОБЪЕДИНЕННАЯ РЕСПУБЛИКА
        "TL" => 626, // ТИМОР-ЛЕСТЕ
        "TG" => 768, // ТОГО
        "TK" => 772, // ТОКЕЛАУ
        "TO" => 776, // ТОНГА
        "TT" => 780, // ТРИНИДАД И ТОБАГО
        "TV" => 798, // ТУВАЛУ
        "TN" => 788, // ТУНИС
        "TM" => 795, // ТУРКМЕНИЯ
        "TR" => 792, // ТУРЦИЯ
        "UG" => 800, // УГАНДА
        "UZ" => 860, // УЗБЕКИСТАН
        "UA" => 804, // УКРАИНА
        "WF" => 876, // УОЛЛИС И ФУТУНА
        "UY" => 858, // УРУГВАЙ
        "FO" => 234, // ФАРЕРСКИЕ ОСТРОВА
        "FJ" => 242, // ФИДЖИ
        "PH" => 608, // ФИЛИППИНЫ
        "FI" => 246, // ФИНЛЯНДИЯ
        "FK" => 238, // ФОЛКЛЕНДСКИЕ ОСТРОВА (МАЛЬВИНСКИЕ)
        "FR" => 250, // ФРАНЦИЯ
        "GF" => 254, // ФРАНЦУЗСКАЯ ГВИАНА
        "PF" => 258, // ФРАНЦУЗСКАЯ ПОЛИНЕЗИЯ
        "TF" => 260, // ФРАНЦУЗСКИЕ ЮЖНЫЕ ТЕРРИТОРИИ
        "HR" => 191, // ХОРВАТИЯ
        "CF" => 140, // ЦЕНТРАЛЬНО-АФРИКАНСКАЯ РЕСПУБЛИКА
        "TD" => 148, // ЧАД
        "ME" => 499, // ЧЕРНОГОРИЯ
        "CZ" => 203, // ЧЕХИЯ
        "CL" => 152, // ЧИЛИ
        "CH" => 756, // ШВЕЙЦАРИЯ
        "SE" => 752, // ШВЕЦИЯ
        "SJ" => 744, // ШПИЦБЕРГЕН И ЯН МАЙЕН
        "LK" => 144, // ШРИ-ЛАНКА
        "EC" => 218, // ЭКВАДОР
        "GQ" => 226, // ЭКВАТОРИАЛЬНАЯ ГВИНЕЯ
        "AX" => 248, // ЭЛАНДСКИЕ ОСТРОВА
        "SV" => 222, // ЭЛЬ-САЛЬВАДОР
        "ER" => 232, // ЭРИТРЕЯ
        "SZ" => 748, // ЭСВАТИНИ
        "EE" => 233, // ЭСТОНИЯ
        "ET" => 231, // ЭФИОПИЯ
        "ZA" => 710, // ЮЖНАЯ АФРИКА
        "GS" => 239, // ЮЖНАЯ ДЖОРДЖИЯ И ЮЖНЫЕ САНДВИЧЕВЫ ОСТРОВА
        "OS" => 896, // ЮЖНАЯ ОСЕТИЯ
        "SS" => 728, // ЮЖНЫЙ СУДАН
        "JM" => 388, // ЯМАЙКА
        "JP" => 392, // ЯПОНИЯ
        _ => return Err!("Unknown country code: {:?}", country),
    })
}