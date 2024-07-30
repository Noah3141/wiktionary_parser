use crate::models::wiktionary_macro::inflection_of::InflectionOf;
use crate::models::language::Language::Russian;
use crate::models::section_header::SectionHeader::*;

#[test]
fn parses() {
    let examples: [InflectionOf; 50] = [
            InflectionOf {
                page_id: 44665,
                page_title: "лук".into(),
                language: Russian,
                section: Etymology4,
                macro_text: "{{inflection of|ru|лука\u{301}||gen|p}}".into(),
            },
            InflectionOf {
                page_id: 93934,
                page_title: "носок".into(),
                language: Russian,
                section: Etymology4,
                macro_text: "{{inflection of|ru|но\u{301}ский||short|m|s}}".into(),
            },
            InflectionOf {
                page_id: 106400,
                page_title: "рода".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|род||acc|p}}".into(),
            },
            InflectionOf {
                page_id: 188634,
                page_title: "банка".into(),
                language: Russian,
                section: Etymology3,
                macro_text: "{{inflection of|ru|банк||gen|s}}".into(),
            },
            InflectionOf {
                page_id: 196079,
                page_title: "часы".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|час||nom//acc|p}}".into(),
            },
            InflectionOf {
                page_id: 197639,
                page_title: "джиннами".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|джи\u{301}нн||ins|p}}".into(),
            },
            InflectionOf {
                page_id: 201823,
                page_title: "паханы".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|паха\u{301}н||nom|p}}".into(),
            },
            InflectionOf {
                page_id: 207515,
                page_title: "заек".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|за\u{301}йка||gen//acc|p}}".into(),
            },
            InflectionOf {
                page_id: 217675,
                page_title: "элементы".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|элеме\u{301}нт||nom|p|;|in|acc|p}}".into(),
            },
            InflectionOf {
                page_id: 233160,
                page_title: "глиста".into(),
                language: Russian,
                section: Etymology2,
                macro_text: "{{inflection of|ru|глист||gen//acc|s}}".into(),
            },
            InflectionOf {
                page_id: 250435,
                page_title: "спас".into(),
                language: Russian,
                section: Etymology2,
                macro_text: "{{inflection of|ru|спасти\u{301}||m|s|past|ind|pfv}}".into(),
            },
            InflectionOf {
                page_id: 262908,
                page_title: "баклажанов".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|баклажа\u{301}н||gen|p}}".into(),
            },
            InflectionOf {
                page_id: 270117,
                page_title: "дорога".into(),
                language: Russian,
                section: Etymology2,
                macro_text: "{{inflection of|ru|дорого\u{301}й||short|f|s}}".into(),
            },
            InflectionOf {
                page_id: 287865,
                page_title: "внучата".into(),
                language: Russian,
                section: Etymology2,
                macro_text: "{{inflection of|ru|внучо\u{301}нок||nom|p}}".into(),
            },
            InflectionOf {
                page_id: 294914,
                page_title: "умиленья".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|умиле\u{301}нье||gen|s|;|nom//acc|p}}".into(),
            },
            InflectionOf {
                page_id: 329439,
                page_title: "поэтам".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|поэ\u{301}т||dat|p}}".into(),
            },
            InflectionOf {
                page_id: 455461,
                page_title: "обычно".into(),
                language: Russian,
                section: Adjective,
                macro_text: "{{inflection of|ru|обы\u{301}чный||short|n|s}}".into(),
            },
            InflectionOf {
                page_id: 460431,
                page_title: "использующая".into(),
                language: Russian,
                section: Participle,
                macro_text: "{{inflection of|ru|испо\u{301}льзующий||f|s|nom}}".into(),
            },
            InflectionOf {
                page_id: 460468,
                page_title: "используемы".into(),
                language: Russian,
                section: Participle,
                macro_text: "{{inflection of|ru|испо\u{301}льзуемый||short|p}}".into(),
            },
            InflectionOf {
                page_id: 486601,
                page_title: "нею".into(),
                language: Russian,
                section: Pronoun,
                macro_text: "{{inflection of|ru|она\u{301}||ins}}".into(),
            },
            InflectionOf {
                page_id: 498827,
                page_title: "ничего".into(),
                language: Russian,
                section: Pronoun,
                macro_text: "{{infl of|ru|ничто\u{301}||gen}}".into(),
            },
            InflectionOf {
                page_id: 527476,
                page_title: "ордера".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|о\u{301}рдер||nom//acc|p}}".into(),
            },
            InflectionOf {
                page_id: 537825,
                page_title: "села".into(),
                language: Russian,
                section: Etymology2,
                macro_text: "{{inflection of|ru|сесть||f|s|past|ind|pfv}}".into(),
            },
            InflectionOf {
                page_id: 599809,
                page_title: "одам".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|о\u{301}да||dat|p}}".into(),
            },
            InflectionOf {
                page_id: 681195,
                page_title: "бардам".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|бард||dat|p}}".into(),
            },
            InflectionOf {
                page_id: 710952,
                page_title: "подвода".into(),
                language: Russian,
                section: Etymology2,
                macro_text: "{{inflection of|ru|подво\u{301}д||gen|s}}".into(),
            },
            InflectionOf {
                page_id: 745746,
                page_title: "бросаться".into(),
                language: Russian,
                section: Verb,
                macro_text: "{{infl of|ru|броса\u{301}ть||pass}}".into(),
            },
            InflectionOf {
                page_id: 761406,
                page_title: "редкое".into(),
                language: Russian,
                section: Adjective,
                macro_text: "{{inflection of|ru|ре\u{301}дкий||nom//acc|n|s}}".into(),
            },
            InflectionOf {
                page_id: 789295,
                page_title: "себе".into(),
                language: Russian,
                section: Pronoun,
                macro_text: "{{inflection of|ru|себя\u{301}||dat//pre}}".into(),
            },
            InflectionOf {
                page_id: 804507,
                page_title: "стану".into(),
                language: Russian,
                section: Etymology1,
                macro_text: "{{inflection of|ru|стать||1|s|fut|ind|pfv}}".into(),
            },
            InflectionOf {
                page_id: 814560,
                page_title: "цветы".into(),
                language: Russian,
                section: Etymology1,
                macro_text: "{{inflection of|ru|цвето\u{301}к||nom//acc|p|t=[[flower]]ing [[plant]]}}".into(),
            },
            InflectionOf {
                page_id: 848599,
                page_title: "ворона".into(),
                language: Russian,
                section: Etymology2,
                macro_text: "{{inflection of|ru|во\u{301}рон||gen//acc|s}}".into(),
            },
            InflectionOf {
                page_id: 864499,
                page_title: "наковало".into(),
                language: Russian,
                section: Verb,
                macro_text: "{{inflection of|ru|накова\u{301}ть||n|s|past|ind|pfv}}".into(),
            },
            InflectionOf {
                page_id: 900915,
                page_title: "звонок".into(),
                language: Russian,
                section: Etymology2,
                macro_text: "{{inflection of|ru|зво\u{301}нкий||short|m|s}}".into(),
            },
            InflectionOf {
                page_id: 972908,
                page_title: "тайного агента".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|та\u{301}йный аге\u{301}нт||gen|s|gloss=[[secret agent]]}}".into(),
            },
            InflectionOf {
                page_id: 1019740,
                page_title: "ране".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|ра\u{301}на||dat//pre|s}}".into(),
            },
            InflectionOf {
                page_id: 1068915,
                page_title: "люты".into(),
                language: Russian,
                section: Adjective,
                macro_text: "{{inflection of|ru|лю\u{301}тый||short|p}}".into(),
            },
            InflectionOf {
                page_id: 1116030,
                page_title: "твоего".into(),
                language: Russian,
                section: Pronoun,
                macro_text: "{{inflection of|ru|твой||m//n|gen|s|;|m|an|acc|s}}".into(),
            },
            InflectionOf {
                page_id: 1175320,
                page_title: "найтись".into(),
                language: Russian,
                section: Verb,
                macro_text: "{{infl of|ru|найти\u{301}||pass}}".into(),
            },
            InflectionOf {
                page_id: 1311722,
                page_title: "курс".into(),
                language: Russian,
                section: Etymology2,
                macro_text: "{{inflection of|ru|курса\u{301}||gen//acc|p}}".into(),
            },
            InflectionOf {
                page_id: 1337108,
                page_title: "подробен".into(),
                language: Russian,
                section: Adjective,
                macro_text: "{{inflection of|ru|подро\u{301}бный||short|m|s}}".into(),
            },
            InflectionOf {
                page_id: 1367786,
                page_title: "морок".into(),
                language: Russian,
                section: Etymology2,
                macro_text: "{{inflection of|ru|моро\u{301}ка||gen|p}}".into(),
            },
            InflectionOf {
                page_id: 1472944,
                page_title: "цель".into(),
                language: Russian,
                section: Etymology2,
                macro_text: "{{inflection of|ru|це\u{301}лить||2|s|imp|impfv}}".into(),
            },
            InflectionOf {
                page_id: 1538516,
                page_title: "оргазму".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|орга\u{301}зм||dat|s}}".into(),
            },
            InflectionOf {
                page_id: 1553481,
                page_title: "ведьмы".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|ве\u{301}дьма||gen|s|;|nom|p}}".into(),
            },
            InflectionOf {
                page_id: 1581283,
                page_title: "дела".into(),
                language: Russian,
                section: Etymology1,
                macro_text: "{{inflection of|ru|де\u{301}ло||gen|s}}".into(),
            },
            InflectionOf {
                page_id: 1591869,
                page_title: "акробатики".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|акроба\u{301}тика||gen|s|;|nom//acc|p}}".into(),
            },
            InflectionOf {
                page_id: 1592323,
                page_title: "визитки".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|визи\u{301}тка||gen|s|;|nom//acc|p}}".into(),
            },
            InflectionOf {
                page_id: 1623561,
                page_title: "колку".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|коло\u{301}к||dat|s}}".into(),
            },
            InflectionOf {
                page_id: 1630670,
                page_title: "австралийки".into(),
                language: Russian,
                section: Noun,
                macro_text: "{{inflection of|ru|австрали\u{301}йка||gen|s|;|nom|p}}".into(),
            },
    ];

    assert_eq!(examples[0].lemma(), "лука\u{301}");
    assert_eq!(examples[1].lemma(), "но\u{301}ский");
    assert_eq!(examples[2].lemma(), "род");
    assert_eq!(examples[3].lemma(), "банк");
    assert_eq!(examples[4].lemma(), "час");
    assert_eq!(examples[5].lemma(), "джи\u{301}нн");
} 

#[test]
fn wiki_example() {
    assert_eq!(
        InflectionOf {
            page_id: 81425630,
            page_title: "цветы".into(),
            language: Russian,
            section: Noun,
            macro_text: "{{inflection of|ru|путь||gen//dat//pre|s|;|nom//acc|p}}".into(),
        }.lemma(),
        "путь"
    )
}

#[test]
fn longer_macro() {
    assert_eq!(
        InflectionOf {
            page_id: 814560,
            page_title: "цветы".into(),
            language: Russian,
            section: Etymology1,
            macro_text: "{{inflection of|ru|цвето\u{301}к||nom//acc|p|t=[[flower]]ing [[plant]]}}".into(),
        }.lemma(),
        "цвето\u{301}к"
    )
}

#[test]
fn comma_delimited() {
    assert_eq!(
        InflectionOf {
            page_id: 0000,
            page_title: "достигнем".into(),
            language: Russian,
            section: Noun,
            macro_text: "{{inflection of|ru|дости́гнуть,дости́чь||1|p|fut|ind|pfv}}".into(),
        }.lemma(),
        "дости́гнуть"
    )
}


#[test]
fn ukrainian() {
    assert_eq!(
        InflectionOf {
            page_id: 0000,
            page_title: "достигнем".into(),
            language: Russian,
            section: Noun,
            macro_text: "{{infl of|uk|перегляда́ти<g:impf>,перегля́нути<g:pf>,перегля́діти<g:pf>||vnoun}}".into(),
        }.lemma(),
        "перегляда́ти"
    )
}

