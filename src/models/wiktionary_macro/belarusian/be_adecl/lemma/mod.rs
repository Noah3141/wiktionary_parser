#[cfg(test)]
mod test;

use scraper::Html;
use crate::{models::wiktionary_macro::Expand, utils::select_from};
use super::BeADecl;

// todo 
// NULL, ((цяжкі́,ця́жкі)), NULL, Adjective, {"lemma":"((цяжкі́,ця́жкі))","ipa":"[t͡sʲaʂˈkʲi]","nom_masc":"цяжкі́","acc_masc":"цяжко́га","gen_masc":"цяжко́га","dat_masc":"цяжко́му","ins_masc":"цяжкі́м","pre_masc":"цяжкі́м","nom_fem":"цяжка́я","acc_fem":"цяжку́ю","gen_fem":"цяжко́й","dat_fem":"цяжко́й","ins_fem":"цяжко́й","pre_fem":"цяжко́й","nom_neut":"цяжко́е","acc_neut":"цяжко́е","gen_neut":"цяжко́га","dat_neut":"цяжко́му","ins_neut":"цяжкі́м","pre_neut":"цяжкі́м","nom_plur":"цяжкі́я","acc_plur":"цяжкі́я","gen_plur":"цяжкі́х","dat_plur":"цяжкі́м","ins_plur":"цяжкі́мі","pre_plur":"цяжкі́х","m_short":null,"f_short":null,"n_short":null,"p_short":null}
// NULL, ((расяны́,рася́ны)), NULL, Adjective, {"lemma":"((расяны́,рася́ны))","ipa":"[rasʲaˈnɨ]","nom_masc":"расяны́","acc_masc":"расяно́га","gen_masc":"расяно́га","dat_masc":"расяно́му","ins_masc":"расяны́м","pre_masc":"расяны́м","nom_fem":"расяна́я","acc_fem":"расяну́ю","gen_fem":"расяно́й","dat_fem":"расяно́й","ins_fem":"расяно́й","pre_fem":"расяно́й","nom_neut":"расяно́е","acc_neut":"расяно́е","gen_neut":"расяно́га","dat_neut":"расяно́му","ins_neut":"расяны́м","pre_neut":"расяны́м","nom_plur":"расяны́я","acc_plur":"расяны́я","gen_plur":"расяны́х","dat_plur":"расяны́м","ins_plur":"расяны́мі","pre_plur":"расяны́х","m_short":null,"f_short":null,"n_short":null,"p_short":null}

impl BeADecl {
    /// Parses the accented lemma from the macro text. Requires no expansion but has to navigate the template syntax.
    pub fn lemma(&self) -> &str {
        let without_brackets = self.macro_text
            .strip_prefix("{{")
            .expect("Starting brackets")
            .strip_suffix("}}")
            .expect("ending brackets");

        let parts: Vec<&str> = without_brackets.split("|").collect();

        let tag_is_expected = parts[0] == "be-adecl" ;
        if !tag_is_expected { println!("\n\nUnexpected tag! {parts:#?}\n\n"); }

        match parts.get(1) {
            Some(segment) => {
                if *segment == "" { return &self.page_title }

                if *segment == "-" && without_brackets.contains("manual") {
                    if let Some(lemma) = select_from(without_brackets, "nom_mp=", "|") {
                        return lemma.trim()
                    };
                    if let Some(lemma) = select_from(without_brackets, "nom_m=", "|") {
                        return lemma.trim()
                    };
                    if let Some(lemma) = select_from(without_brackets, "nom_sg=", "|") {
                        return lemma.trim()
                    };
                    panic!("Couldn't figure out this manual template: {{{without_brackets}}}")
                }

                if let Some(lt) = segment.find("&lt;") {
                    return &segment[..lt]
                } else {
                    return segment
                }
            },
            None => &self.page_title,
        }
    }

    pub async fn form_and_lemma(&self, client: &reqwest::Client) -> Vec<(String, &str)> {

        let mut form_lemma_tuples: Vec<(String, &str)> = Vec::with_capacity(6);
        let res = self.expand_with(&client).await;
        let fragment = Html::parse_fragment(&res);

        for classname in super::class_selectors::ALL {
            if let Some(form) = self.get_form(&fragment, classname) {
                form_lemma_tuples.push( (form, self.lemma()) )
            }
        }

        form_lemma_tuples
    }
}