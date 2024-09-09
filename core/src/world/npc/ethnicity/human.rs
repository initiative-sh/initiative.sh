use super::{Age, Gender, Generate, GenerateSimple};
use rand::prelude::*;

pub struct Ethnicity;

impl GenerateSimple for Ethnicity {
    fn syllable_fname_count_f() -> &'static [(u8, usize)] {
        &[(2, 717), (3, 357), (4, 27), (5, 1)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_f() -> &'static [(&'static str, usize)] {
        &[
            ("Ja", 19), ("Ta", 18), ("Ha", 16), ("Sha", 14), ("Sa", 14), ("Na", 14), ("Fa", 11),
            ("Ka", 10), ("Be", 8), ("Ma", 8), ("Ba", 8), ("Se", 8), ("Da", 8), ("La", 7),
            ("Mar", 7), ("Mi", 6), ("Ne", 6), ("Mu", 5), ("Ra", 5), ("Ca", 5), ("Ri", 5),
            ("Shar", 5), ("Za", 5), ("Me", 5), ("Ya", 5), ("Dar", 5), ("Li", 5), ("Ju", 5),
            ("De", 5), ("Bel", 5), ("Co", 5), ("Ki", 5), ("Del", 5), ("So", 5), ("Te", 4),
            ("Lu", 4), ("He", 4), ("Myr", 4), ("Shan", 4), ("Jan", 4), ("Nu", 4), ("Ara", 4),
            ("Hel", 4), ("Mo", 4), ("Shei", 4), ("Ky", 4), ("Alu", 4), ("Va", 4), ("Jha", 4),
            ("Ko", 4),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_f() -> &'static [(&'static str, usize)] {
        &[
            ("ra", 123), ("na", 88), ("la", 61), ("sa", 35), ("da", 33), ("tha", 25), ("dra", 19),
            ("ta", 18), ("lia", 13), ("tra", 12), ("ri", 12), ("sha", 11), ("ni", 11), ("nya", 11),
            ("ma", 8), ("ria", 8), ("thra", 8), ("ris", 7), ("ka", 6), ("ko", 6), ("lah", 5),
            ("rah", 5), ("san", 4), ("wyn", 4), ("cia", 4), ("rin", 4), ("the", 4), ("ja", 4),
            ("lynn", 4), ("len", 4), ("za", 4), ("lin", 4), ("non", 3), ("va", 3), ("nia", 3),
            ("shi", 3), ("nor", 3), ("si", 3), ("rie", 3), ("nar", 3), ("naa", 3), ("riel", 3),
            ("dia", 3), ("ga", 3), ("sia", 3), ("lyn", 3), ("wa", 2), ("set", 2), ("ly", 2),
            ("ca", 2),
        ]
    }

    fn syllable_fname_count_m() -> &'static [(u8, usize)] {
        &[(2, 2684), (3, 561), (4, 61), (5, 5)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_m() -> &'static [(&'static str, usize)] {
        &[
            ("Ha", 38), ("Ma", 34), ("Ta", 30), ("Ka", 27), ("Na", 24), ("Sa", 24), ("Ja", 23),
            ("Ra", 23), ("Ho", 20), ("Be", 18), ("Ba", 18), ("To", 17), ("Ga", 17), ("Go", 17),
            ("Ni", 15), ("Har", 15), ("Mar", 15), ("Ko", 15), ("Bel", 15), ("Da", 15), ("Re", 14),
            ("Fa", 13), ("La", 13), ("Hi", 12), ("Jo", 12), ("Bar", 11), ("Mi", 11), ("Hel", 11),
            ("Dar", 11), ("Ca", 11), ("Dun", 11), ("Te", 10), ("Car", 10), ("Gar", 10), ("Ya", 10),
            ("Tor", 10), ("Ki", 9), ("Mu", 9), ("Za", 9), ("Tha", 9), ("Ro", 9), ("Hal", 9),
            ("Del", 9), ("De", 9), ("Mor", 9), ("Va", 9), ("Rein", 9), ("Du", 8), ("Pa", 8),
            ("Dra", 8),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_m() -> &'static [(&'static str, usize)] {
        &[
            ("dar", 25), ("gar", 23), ("ter", 22), ("ro", 21), ("rin", 20), ("der", 20),
            ("lar", 19), ("ran", 18), ("ren", 17), ("man", 16), ("to", 15), ("mar", 15),
            ("ra", 14), ("len", 14), ("ji", 14), ("ron", 13), ("mo", 13), ("rik", 13), ("rak", 13),
            ("dor", 13), ("ther", 13), ("ri", 13), ("mon", 12), ("lan", 12), ("lo", 12),
            ("ton", 12), ("gan", 12), ("da", 12), ("don", 12), ("run", 11), ("ril", 11),
            ("tar", 11), ("jo", 11), ("ros", 11), ("kar", 10), ("har", 10), ("wa", 10), ("ko", 10),
            ("ric", 10), ("bar", 10), ("do", 10), ("lin", 10), ("son", 10), ("nar", 10),
            ("rim", 9), ("nan", 9), ("go", 9), ("rus", 9), ("reth", 9), ("ki", 8),
        ]
    }

    fn syllable_fname_count() -> &'static [(u8, usize)] {
        &[(2, 3502), (3, 945), (4, 90), (5, 6)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Ha", 56), ("Ta", 49), ("Ma", 42), ("Ja", 42), ("Na", 40), ("Sa", 38), ("Ka", 37),
            ("Ra", 30), ("Ba", 27), ("Be", 26), ("Fa", 25), ("Da", 23), ("Mar", 22), ("Ho", 22),
            ("La", 21), ("Bel", 21), ("Sha", 21), ("Ko", 20), ("Ga", 19), ("Har", 19), ("Go", 19),
            ("Mi", 18), ("To", 18), ("Ni", 17), ("Ca", 17), ("Re", 16), ("Dar", 16), ("Hel", 16),
            ("Se", 16), ("Za", 15), ("Ya", 15), ("De", 15), ("Mu", 14), ("Ki", 14), ("Del", 14),
            ("Hi", 14), ("Te", 14), ("Jo", 14), ("Tha", 13), ("Ne", 13), ("Va", 13), ("Car", 13),
            ("Co", 12), ("Ri", 12), ("Bar", 12), ("Mor", 12), ("Me", 11), ("He", 11), ("Ro", 11),
            ("Yu", 11),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last() -> &'static [(&'static str, usize)] {
        &[
            ("ra", 145), ("na", 92), ("la", 63), ("da", 46), ("sa", 37), ("ri", 28), ("dar", 27),
            ("tha", 26), ("rin", 24), ("gar", 23), ("ta", 23), ("ter", 22), ("lar", 22),
            ("ren", 21), ("der", 21), ("ro", 21), ("dra", 19), ("mar", 19), ("ran", 19),
            ("ni", 18), ("len", 18), ("to", 18), ("man", 17), ("ko", 16), ("ka", 15), ("don", 14),
            ("sha", 14), ("ji", 14), ("ril", 14), ("ma", 14), ("lin", 14), ("mon", 14),
            ("lan", 13), ("ther", 13), ("dor", 13), ("nar", 13), ("rak", 13), ("ris", 13),
            ("ron", 13), ("lia", 13), ("rik", 13), ("gan", 13), ("lo", 13), ("tra", 13),
            ("mo", 13), ("run", 12), ("nya", 12), ("son", 12), ("san", 12), ("jo", 12),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("la", 47), ("li", 34), ("ra", 32), ("na", 31), ("da", 27), ("ta", 24), ("ri", 22),
            ("lan", 21), ("ma", 20), ("ni", 20), ("sa", 18), ("do", 18), ("ran", 17), ("de", 16),
            ("ti", 16), ("lo", 16), ("ka", 14), ("mi", 14), ("re", 13), ("ga", 13), ("ro", 12),
            ("a", 12), ("mo", 11), ("ki", 11), ("le", 11), ("va", 10), ("si", 10), ("to", 10),
            ("ko", 10), ("tha", 9), ("ris", 9), ("las", 8), ("se", 8), ("hi", 7), ("za", 7),
            ("shi", 7), ("lin", 7), ("les", 7), ("go", 7), ("ha", 6), ("lae", 5), ("bi", 5),
            ("ba", 5), ("man", 5), ("rin", 5), ("bu", 5), ("te", 5), ("ne", 5), ("no", 5),
            ("chi", 5),
        ]
    }

    fn syllable_lname_count() -> &'static [(u8, usize)] {
        &[(2, 2195), (3, 824), (4, 99), (5, 1)]
    }

    #[rustfmt::skip]
    fn syllable_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("al-", 0), ("Obar", 81), ("Ta", 28), ("Ha", 27), ("Ka", 26), ("Har", 23), ("Ma", 23),
            ("Ra", 21), ("Ca", 20), ("Hel", 18), ("Na", 17), ("Mar", 15), ("De", 15), ("Sa", 15),
            ("Cor", 15), ("Sil", 15), ("Black", 14), ("Ja", 14), ("Da", 14), ("Me", 13),
            ("Be", 13), ("Storm", 13), ("Mi", 13), ("Moon", 12), ("Ba", 12), ("Se", 12),
            ("Tan", 12), ("Tha", 10), ("Del", 9), ("Ill", 9), ("Dark", 9), ("Ki", 8), ("Su", 8),
            ("Au", 8), ("Dag", 8), ("Dra", 8), ("Red", 8), ("Blood", 8), ("Ne", 8), ("Thorn", 8),
            ("Ken", 8), ("Sha", 8), ("Thun", 8), ("Fa", 7), ("Mel", 7), ("Star", 7), ("Iron", 7),
            ("Harp", 7), ("Olaun", 7), ("La", 7),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("ra", 33), ("sil", 19), ("ta", 18), ("sa", 17), ("na", 17), ("lan", 14), ("la", 14),
            ("to", 14), ("ma", 14), ("cas", 12), ("ri", 12), ("go", 12), ("ver", 12), ("mae", 12),
            ("der", 11), ("de", 11), ("da", 11), ("man", 10), ("ger", 9), ("ven", 9), ("ka", 9),
            ("li", 9), ("win", 8), ("hi", 8), ("su", 8), ("ro", 8), ("mas", 8), ("shi", 7),
            ("mer", 7), ("lo", 7), ("no", 7), ("ing", 7), ("va", 7), ("a", 7), ("ter", 7),
            ("ti", 7), ("le", 6), ("Ja", 6), ("ha", 6), ("Ka", 6), ("ne", 6), ("ki", 6), ("Ga", 6),
            ("Za", 6), ("mi", 6), ("vern", 6), ("gas", 5), ("tham", 5), ("do", 5), ("feat", 5),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("skyr", 81), ("ter", 43), ("ra", 25), ("ver", 24), ("tle", 23), ("tar", 22),
            ("hand", 19), ("ro", 16), ("dar", 15), ("lin", 15), ("mer", 15), ("mar", 14),
            ("star", 14), ("ril", 14), ("horn", 14), ("son", 13), ("ri", 13), ("la", 13),
            ("na", 13), ("lar", 12), ("li", 12), ("man", 12), ("ka", 12), ("tharn", 11),
            ("wood", 11), ("ren", 11), ("to", 11), ("der", 11), ("ther", 10), ("da", 10),
            ("ger", 10), ("gar", 10), ("ker", 10), ("ton", 9), ("ru", 9), ("ance", 9), ("ris", 9),
            ("ven", 9), ("mo", 8), ("ji", 8), ("shield", 8), ("ki", 8), ("ryn", 8), ("blade", 7),
            ("bar", 7), ("ras", 7), ("mane", 7), ("dran", 7), ("nok", 7), ("tree", 7),
        ]
    }

    fn compound_word_probability() -> f64 {
        0.08863080684596578
    }

    #[rustfmt::skip]
    fn word_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Black", 12), ("Storm", 12), ("Moon", 8), ("Blood", 8), ("Thorn", 7), ("Red", 7),
            ("Silver", 6), ("Green", 6), ("Dark", 6), ("Iron", 5), ("Thunder", 5), ("Hawk", 5),
            ("Far", 4), ("Ten", 4), ("White", 4), ("Raven", 4), ("Fire", 4), ("Gold", 4),
            ("Star", 4), ("Grey", 3), ("Ser", 3), ("Long", 3), ("Night", 3), ("Bright", 3),
            ("Keen", 3), ("Dagger", 3), ("Shadow", 2), ("Dry", 2), ("Good", 2), ("Scat", 2),
            ("Tall", 2), ("Horns", 2), ("Talon", 2), ("Deep", 2), ("High", 2), ("Flame", 2),
            ("Winds", 2), ("Thin", 2), ("Moss", 2), ("Bow", 2), ("Stone", 2), ("Falcon", 2),
            ("Grain", 2), ("Del", 2), ("Mirror", 2), ("Stout", 2), ("Wood", 2), ("Gray", 2),
            ("Frost", 2), ("Wind", 2),
        ]
    }

    #[rustfmt::skip]
    fn word_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("hand", 13), ("horn", 8), ("tar", 7), ("son", 7), ("mantle", 7), ("castle", 6),
            ("wind", 6), ("her", 6), ("sword", 5), ("tree", 5), ("wood", 5), ("silver", 5),
            ("feat", 5), ("winter", 5), ("blade", 5), ("fist", 5), ("mane", 4), ("helm", 4),
            ("pent", 4), ("star", 3), ("kin", 3), ("shar", 3), ("eye", 3), ("hawk", 3),
            ("singer", 3), ("gar", 3), ("crown", 3), ("seer", 3), ("word", 3), ("bough", 3),
            ("stone", 3), ("hair", 3), ("shield", 3), ("bane", 3), ("bridge", 3), ("dale", 3),
            ("man", 3), ("dark", 2), ("ara", 2), ("shine", 2), ("runner", 2), ("crest", 2),
            ("wing", 2), ("dusk", 2), ("wild", 2), ("hale", 2), ("wise", 2), ("cloak", 2),
            ("breaker", 2), ("sar", 2),
        ]
    }
}

impl Generate for Ethnicity {
    fn gen_name(rng: &mut impl Rng, _age: &Age, gender: &Gender) -> String {
        format!(
            "{} {}",
            Self::gen_fname_simple(rng, gender),
            Self::gen_lname_simple(rng),
        )
    }
}

#[cfg(test)]
mod test_generate_for_ethnicity {
    use super::*;
    use crate::world::npc::ethnicity::{regenerate, Ethnicity};
    use crate::world::npc::NpcData;

    #[test]
    fn gen_name_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let age = Age::Adult;
        let m = Gender::Masculine;
        let f = Gender::Feminine;
        let t = Gender::NonBinaryThey;

        assert_eq!(
            [
                "Gorik Aulandran",
                "Carleder Helkidran",
                "Selania Maren",
                "Delrasa Marton",
                "Rina Tater",
                "Mini Obarlin",
            ],
            [
                gen_name(&mut rng, &age, &m),
                gen_name(&mut rng, &age, &m),
                gen_name(&mut rng, &age, &f),
                gen_name(&mut rng, &age, &f),
                gen_name(&mut rng, &age, &t),
                gen_name(&mut rng, &age, &t),
            ],
        );
    }

    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        let mut npc = NpcData::default();
        npc.gender.replace(*gender);
        npc.age.replace(*age);
        npc.ethnicity.replace(Ethnicity::Human);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
