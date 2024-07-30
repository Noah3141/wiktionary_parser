use crate::models::wiktionary_macro::RuNounTable;
use crate::models::language::Language::Russian;
use crate::models::section_header::SectionHeader::*;


#[tokio::test]
async fn basic_word() {
    let examples: [RuNounTable; 40] = [
        RuNounTable {
            page_id: 104531,
            page_title: String::from("Боливия"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|Боли\u{301}вия|n=sg}}"),
        },
        RuNounTable {
            page_id: 105021,
            page_title: String::from("палочка для еды"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|[[па\u{301}лочка]]|*|_|*[[для]]|$|_|[[еда|еды\u{301}]]|$}}"),
        },
        RuNounTable {
            page_id: 172703,
            page_title: String::from("Солнце"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|Со\u{301}лнце}}"),
        },
        RuNounTable {
            page_id: 178821,
            page_title: String::from("Австралия"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|Австра\u{301}лия|n=sg}}"),
        },
        RuNounTable {
            page_id: 180363,
            page_title: String::from("град"),
            language: Russian,
            section: Etymology2,
            macro_text: String::from("{{ru-noun-table}}"),
        },
        RuNounTable {
            page_id: 180536,
            page_title: String::from("британка"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|брита\u{301}нка|*|a=an}}"),
        },
        RuNounTable {
            page_id: 181201,
            page_title: String::from("жирафа"),
            language: Russian,
            section: Etymology1,
            macro_text: String::from("{{ru-noun-table|жира\u{301}фа|a=an}}"),
        },
        RuNounTable {
            page_id: 182656,
            page_title: String::from("слоновая кость"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|[[слоновый|слоно\u{301}вая]]|+|_|e|[[кость]]|f}}"),
        },
        RuNounTable {
            page_id: 183325,
            page_title: String::from("Жираф"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|Жира\u{301}ф|n=sg|a=an}}"),
        },
        RuNounTable {
            page_id: 183893,
            page_title: String::from("автомобиль"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|автомоби\u{301}ль|m}}"),
        },
        RuNounTable {
            page_id: 184606,
            page_title: String::from("нога"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|f'|нога\u{301}|acc_sg_tailall=*|notes=* ''Often unstressed ([[enclitic]]) with some prepositions, e.g. {{m|ru|[[за\u{301}]] '''ногу'''}}, {{m|ru|[[на\u{301}]] '''ногу'''}}, {{m|ru|[[на\u{301}]] '''ноги'''}}, {{m|ru|[[по\u{301}д]] '''ноги'''}}.''}}"),
        },
        RuNounTable {
            page_id: 185560,
            page_title: String::from("Армения"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|Арме\u{301}ния|n=sg}}"),
        },
        RuNounTable {
            page_id: 185929,
            page_title: String::from("люди"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|лю\u{301}ди|m|gen_pl=люде\u{301}й|ins_pl=людьми\u{301}|a=an}}"),
        },
        RuNounTable {
            page_id: 187204,
            page_title: String::from("Бар"),
            language: Russian,
            section: Etymology2,
            macro_text: String::from("{{ru-noun-table}}"),
        },
        RuNounTable {
            page_id: 187833,
            page_title: String::from("время"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|c|вре\u{301}мя}}"),
        },
        RuNounTable {
            page_id: 188569,
            page_title: String::from("абсцесс"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|абсце\u{301}сс}}"),
        },
        RuNounTable {
            page_id: 188611,
            page_title: String::from("анализ"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|ана\u{301}лиз}}"),
        },
        RuNounTable {
            page_id: 188636,
            page_title: String::from("беда"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|d|беда\u{301}}}"),
        },
        RuNounTable {
            page_id: 188656,
            page_title: String::from("блузка"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|блу\u{301}зка|*}}"),
        },
        RuNounTable {
            page_id: 188687,
            page_title: String::from("ванна"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|ва\u{301}нна}}"),
        },
        RuNounTable {
            page_id: 188822,
            page_title: String::from("яйцо"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|d|яйцо\u{301}|gen_pl=яи\u{301}ц,я\u{301}иц*|notes=* Colloquial.}}"),
        },
        RuNounTable {
            page_id: 188960,
            page_title: String::from("стреха"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|d|стреха\u{301}}}"),
        },
        RuNounTable {
            page_id: 190046,
            page_title: String::from("дурак"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|b|дура\u{301}к|a=an}}"),
        },
        RuNounTable {
            page_id: 192400,
            page_title: String::from("лесоматериал"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|лесоматериа\u{301}л}}"),
        },
        RuNounTable {
            page_id: 193275,
            page_title: String::from("смоква"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|смо\u{301}ква}}"),
        },
        RuNounTable {
            page_id: 193877,
            page_title: String::from("кокаин"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|кокаи\u{301}н}}"),
        },
        RuNounTable {
            page_id: 194172,
            page_title: String::from("Российская Федерация"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|[[российский|Росси\u{301}йская]]|+|_|[[федерация|Федера\u{301}ция]]|n=sg}}"),
        },
        RuNounTable {
            page_id: 194482,
            page_title: String::from("Молдова"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|Молдо\u{301}ва|n=sg}}"),
        },
        RuNounTable {
            page_id: 194993,
            page_title: String::from("Мелхиседек"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|Мелхи\u{301}седек|a=an|n=sg}}"),
        },
        RuNounTable {
            page_id: 195661,
            page_title: String::from("Жёлтая река"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|[[жёлтый|Жёлтая]]|+|_|d'|[[река\u{301}]]|n=sg}}"),
        },
        RuNounTable {
            page_id: 196323,
            page_title: String::from("Евразия"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|Евра\u{301}зия|n=sg}}"),
        },
        RuNounTable {
            page_id: 197114,
            page_title: String::from("шлюха"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|шлю\u{301}ха|a=an}}"),
        },
        RuNounTable {
            page_id: 197623,
            page_title: String::from("Левант"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|Лева\u{301}нт|n=sg}}"),
        },
        RuNounTable {
            page_id: 197902,
            page_title: String::from("Испания"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|Испа\u{301}ния|n=sg}}"),
        },
        RuNounTable {
            page_id: 198175,
            page_title: String::from("вина"),
            language: Russian,
            section: Etymology2,
            macro_text: String::from("{{ru-noun-table|ви\u{301}на}}"),
        },
        RuNounTable {
            page_id: 198897,
            page_title: String::from("Маскат"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|Маска\u{301}т|n=sg}}"),
        },
        RuNounTable {
            page_id: 199477,
            page_title: String::from("Уэлен"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|Уэле\u{301}н|n=sg}}"),
        },
        RuNounTable {
            page_id: 200386,
            page_title: String::from("хлеб"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table}}"),
        },
        RuNounTable {
            page_id: 201574,
            page_title: String::from("Рамазан"),
            language: Russian,
            section: ProperNoun,
            macro_text: String::from("{{ru-noun-table|Рамаза\u{301}н|n=sg}}"),
        },
        RuNounTable {
            page_id: 202002,
            page_title: String::from("исток"),
            language: Russian,
            section: Noun,
            macro_text: String::from("{{ru-noun-table|исто\u{301}к}}"),
        }
    ];

    let grad = &examples[4];

    let form_lemmas = grad.form_and_lemma(&reqwest::Client::new()).await;
    dbg!(&form_lemmas);

    assert!(
        form_lemmas.contains(&(String::from("гра\u{301}да"), "град".to_string())) 
    )
}



#[test]
fn жир() {
    let noun_table = RuNounTable {
        page_id: 495299,
        page_title: "жир".to_string(),
        language: Russian,
        section: Noun,
        macro_text: "{{ru-noun-table|c|loc=+|par=+}}".to_string(),
    };
    assert_eq!(
        noun_table.lemma(),
        "жир"
    )
}

#[test]
fn свобода_слова() {
    let noun_table = RuNounTable {
        page_id: 507878,
        page_title: "свобода слова".to_string(),
        language: Russian,
        section: Noun,
        macro_text: "{{ru-noun-table|[[свобо\u{301}да]]|_|[[слово|сло\u{301}ва]]|$}}".to_string(),
    };
    assert_eq!(
        noun_table.lemma(),
        "свобо\u{301}да сло\u{301}ва"
    )
}

#[test]
fn килограмм() {
    let noun_table = RuNounTable {
        page_id: 525259,
        page_title: "килограмм".to_string(),
        language: Russian,
        section: Noun,
        macro_text: "{{ru-noun-table|килогра\u{301}мм|gen_pl=килогра\u{301}ммов,килогра\u{301}мм*|notes=* Colloquial.}}".to_string(),
    };
    assert_eq!(
        noun_table.lemma(),
        "килогра\u{301}мм"
    )
}

#[test]
fn аббатиса() {
    let noun_table = RuNounTable {
        page_id: 535568,
        page_title: "аббатиса".to_string(),
        language: Russian,
        section: Noun,
        macro_text: "{{ru-noun-table|аббати\u{301}са|a=an}}".to_string(),
    };
    assert_eq!(
        noun_table.lemma(),
        "аббати\u{301}са"
    )
}

#[test]
fn колосс() {
    let noun_table = RuNounTable {
        page_id: 548105,
        page_title: "колосс".to_string(),
        language: Russian,
        section: Noun,
        macro_text: "{{ru-noun-table|коло\u{301}сс}}".to_string(),
    };
    assert_eq!(
        noun_table.lemma(),
        "коло\u{301}сс"
    )
}

#[test]
fn варёное() {
    let noun_table = RuNounTable {
        page_id: 562312,
        page_title: "варёное яйцо".to_string(),
        language: Russian,
        section: Noun,
        macro_text: "{{ru-noun-table|[[варёный|варёное]]|+|_|d|[[яйцо\u{301}]]|gen_pl2=яи\u{301}ц}}".to_string(),
    };
    assert_eq!(
        noun_table.lemma(),
        "варёное яйцо\u{301}"
    )
}

#[test]
fn кошмар() {
    let noun_table = RuNounTable {
        page_id: 630250,
        page_title: "кошмар".to_string(),
        language: Russian,
        section: Noun,
        macro_text: "{{ru-noun-table|кошма\u{301}р}}".to_string(),
    };
    assert_eq!(
        noun_table.lemma(),
        "кошма\u{301}р"
    )
}

#[test]
fn приёмник() {
    let noun_table = RuNounTable {
        page_id: 647606,
        page_title: "приёмник воздушного давления".to_string(),
        language: Russian,
        section: Noun,
        macro_text: "{{ru-noun-table|[[приёмник]]|_|[[воздушный|возду\u{301}шного]]|$|_|[[давление|давле\u{301}ния]]|$}}".to_string(),
    };
    assert_eq!(
        noun_table.lemma(),
        "приёмник возду\u{301}шного давле\u{301}ния"
    )

}

#[test]
fn волшебник() {
    let noun_table = RuNounTable {
        page_id: 677621,
        page_title: "волшебник".to_string(),
        language: Russian,
        section: Noun,
        macro_text: "{{ru-noun-table|волше\u{301}бник|a=an}}".to_string(),
    };
    assert_eq!(
        noun_table.lemma(),
        "волше\u{301}бник"
    )
}

#[test]
fn кобальт() {
    let noun_table = RuNounTable {
        page_id: 690540,
        page_title: "кобальт".to_string(),
        language: Russian,
        section: Noun,
        macro_text: "{{ru-noun-table|ко\u{301}бальт}}".to_string(),
    };
    assert_eq!(
        noun_table.lemma(),
        "ко\u{301}бальт"
    )
}

#[test]
fn хассий() {
    let noun_table = RuNounTable {
        page_id: 690736,
        page_title: "хассий".to_string(),
        language: Russian,
        section: Noun,
        macro_text: "{{ru-noun-table|ха\u{301}ссий}}".to_string(),
    };
    assert_eq!(
        noun_table.lemma(),
        "ха\u{301}ссий"
    )
}

#[test]
fn разъём() {
    let noun_table = RuNounTable {
        page_id: 706107,
        page_title: "разъём".to_string(),
        language: Russian,
        section: Noun,
        macro_text: "{{ru-noun-table}}".to_string(),
    };
    assert_eq!(
        noun_table.lemma(),
        "разъём"
    )
}

#[test]
fn автомашина() {
    let noun_table = RuNounTable {
        page_id: 724630,
        page_title: "автомашина".to_string(),
        language: Russian,
        section: Noun,
        macro_text: "{{ru-noun-table|автомаши\u{301}на}}".to_string(),
    };
    assert_eq!(
        noun_table.lemma(),
        "автомаши\u{301}на"
    )
}

#[test]
fn спина() {
    let noun_table = RuNounTable {
        page_id: 732715,
        page_title: "спина".to_string(),
        language: Russian,
        section: Noun,
        macro_text: "{{ru-noun-table|old=1|d'|спина\u{301}}}".to_string(),
    };
    assert_eq!(
        noun_table.lemma(),
        "спина\u{301}"
    )
}
