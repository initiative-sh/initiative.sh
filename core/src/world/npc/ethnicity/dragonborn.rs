use super::{Age, Gender, Generate, GenerateSimple};
use rand::prelude::*;

pub struct Ethnicity;

impl GenerateSimple for Ethnicity {
    fn syllable_fname_count_f() -> &'static [(u8, usize)] {
        &[(2, 32), (3, 34), (4, 19), (5, 13), (6, 3), (7, 3)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_f() -> &'static [(&'static str, usize)] {
        &[
            ("Za", 3), ("Na", 3), ("Che", 2), ("Vo", 2), ("Nar", 2), ("Ma", 2), ("Va", 2),
            ("Arel", 1), ("Arveia", 1), ("Asho", 1), ("Au", 1), ("Bi", 1), ("Bo", 1), ("Ca", 1),
            ("Cae", 1), ("Clau", 1), ("Cly", 1), ("Dar", 1), ("Dheub", 1), ("Emy", 1), ("Ere", 1),
            ("Essem", 1), ("Fll'", 1), ("Fy", 1), ("Gar", 1), ("Gau", 1), ("Ges", 1), ("Ghau", 1),
            ("Ica", 1), ("Idriz", 1), ("Isen", 1), ("Iskda", 1), ("Ja", 1), ("Je", 1), ("Jha", 1),
            ("Jhi", 1), ("Ka", 1), ("Kar", 1), ("Kas", 1), ("Ke", 1), ("Khor", 1), ("Lham", 1),
            ("Min", 1), ("Mis", 1), ("Mo", 1), ("Ni", 1), ("Orlar", 1), ("Orma", 1), ("Otaa", 1),
            ("Ou", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_f() -> &'static [(&'static str, usize)] {
        &[
            ("la", 6), ("ra", 6), ("tha", 5), ("na", 5), ("tar", 3), ("ri", 3), ("rith", 2),
            ("va", 2), ("thra", 2), ("deh", 2), ("rath", 2), ("rak", 2), ("ka", 2), ("lax", 1),
            ("leen", 1), ("les", 1), ("lian", 1), ("lin", 1), ("lon", 1), ("loss", 1), ("lym", 1),
            ("lynx", 1), ("ma", 1), ("mar", 1), ("mi", 1), ("mix", 1), ("ni", 1), ("niius", 1),
            ("nos", 1), ("pyl", 1), ("race", 1), ("racht", 1), ("raele", 1), ("rakh", 1),
            ("ree", 1), ("rene", 1), ("rial", 1), ("ris", 1), ("rose", 1), ("ru", 1), ("ryu", 1),
            ("ryx", 1), ("sar", 1), ("saya", 1), ("shkin", 1), ("shna", 1), ("shni", 1),
            ("shva", 1), ("sjach", 1), ("ta", 1),
        ]
    }

    fn syllable_fname_count_m() -> &'static [(u8, usize)] {
        &[(2, 82), (3, 68), (4, 35), (5, 14), (6, 5), (7, 2)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_m() -> &'static [(&'static str, usize)] {
        &[
            ("Ba", 4), ("Sa", 3), ("Nar", 2), ("Me", 2), ("Ca", 2), ("Hes", 2), ("Ver", 2),
            ("Ma", 2), ("Mi", 2), ("Des", 2), ("Olo", 2), ("La", 2), ("Tha", 2), ("Aswi", 1),
            ("Asta", 1), ("Angla", 1), ("Ashar", 1), ("Asa", 1), ("Anda", 1), ("Aeg", 1),
            ("Cha", 1), ("Fy", 1), ("Char", 1), ("Charth", 1), ("Chel", 1), ("Con", 1),
            ("Cryo", 1), ("Arjha", 1), ("Do", 1), ("Dom", 1), ("Bu", 1), ("Brom", 1), ("Argu", 1),
            ("Ami", 1), ("Fel", 1), ("Eskor", 1), ("Bha", 1), ("Eshu", 1), ("Fer", 1), ("Fir", 1),
            ("Friz", 1), ("Ful", 1), ("Esham-", 1), ("Ga", 1), ("Gar", 1), ("Gesh", 1),
            ("Ghau", 1), ("Ghed", 1), ("Ghon", 1), ("Arcti", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_m() -> &'static [(&'static str, usize)] {
        &[
            ("ros", 3), ("thor", 3), ("dar", 3), ("rash", 3), ("rax", 3), ("fang", 2), ("das", 2),
            ("noth", 2), ("tar", 2), ("tor", 2), ("rinn", 2), ("gar", 2), ("roth", 2), ("dyr", 2),
            ("rac", 2), ("mark", 2), ("reth", 2), ("rin", 2), ("thon", 2), ("zar", 2), ("lar", 2),
            ("thar", 2), ("nak", 2), ("sar", 2), ("far", 1), ("dan", 1), ("chaud", 1), ("dusk", 1),
            ("dun", 1), ("farn", 1), ("dain", 1), ("droth", 1), ("drel", 1), ("dac", 1),
            ("can", 1), ("bar", 1), ("dos", 1), ("dorg", 1), ("cus", 1), ("gyrt", 1), ("groth", 1),
            ("dor", 1), ("hael", 1), ("hen", 1), ("ho", 1), ("hor", 1), ("kaan", 1), ("kan", 1),
            ("kar", 1), ("gras", 1),
        ]
    }

    fn syllable_fname_count() -> &'static [(u8, usize)] {
        &[(2, 125), (3, 120), (4, 62), (5, 29), (6, 8), (7, 6)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Ver", 4), ("Nar", 4), ("Na", 4), ("Za", 4), ("Sa", 4), ("Ba", 4), ("Ma", 4),
            ("Tha", 3), ("Ve", 3), ("Ha", 3), ("Me", 3), ("Ca", 3), ("Rau", 2), ("Si", 2),
            ("Olo", 2), ("Ana", 2), ("Vo", 2), ("Ta", 2), ("Slar", 2), ("Va", 2), ("Ri", 2),
            ("To", 2), ("Ni", 2), ("Ze", 2), ("Ra", 2), ("Ry", 2), ("Mi", 2), ("Au", 2),
            ("Che", 2), ("La", 2), ("Kar", 2), ("Gor", 2), ("Ghau", 2), ("Fy", 2), ("Des", 2),
            ("Gar", 2), ("Hes", 2), ("Ven", 2), ("Aur", 1), ("Angla", 1), ("Athaug", 1),
            ("Anda", 1), ("Akre", 1), ("Aswi", 1), ("Asta", 1), ("Ana-", 1), ("Cryo", 1),
            ("Asho", 1), ("Con", 1), ("Cly", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last() -> &'static [(&'static str, usize)] {
        &[
            ("ra", 7), ("tar", 6), ("la", 6), ("tha", 6), ("na", 6), ("dar", 4), ("thor", 4),
            ("xis", 4), ("rax", 4), ("sar", 3), ("rin", 3), ("ros", 3), ("rath", 3), ("rash", 3),
            ("roth", 3), ("ri", 3), ("mar", 3), ("rak", 3), ("zar", 3), ("nos", 3), ("trix", 2),
            ("thax", 2), ("va", 2), ("rith", 2), ("rinn", 2), ("ris", 2), ("thar", 2), ("thon", 2),
            ("thra", 2), ("tor", 2), ("ran", 2), ("gar", 2), ("deh", 2), ("noth", 2), ("ni", 2),
            ("rac", 2), ("reth", 2), ("fang", 2), ("das", 2), ("nak", 2), ("dyr", 2), ("mi", 2),
            ("lon", 2), ("mark", 2), ("lan", 2), ("lar", 2), ("kas", 2), ("ka", 2), ("lian", 2),
            ("naar", 2),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("ra", 20), ("la", 18), ("ma", 12), ("ri", 8), ("ti", 8), ("na", 8), ("li", 7),
            ("da", 6), ("ro", 6), ("lo", 5), ("mi", 5), ("za", 5), ("ta", 5), ("va", 5), ("sa", 4),
            ("nar", 4), ("ren", 4), ("ga", 4), ("the", 3), ("tu", 3), ("tha", 3), ("no", 3),
            ("me", 3), ("run", 3), ("lan", 3), ("ryn", 3), ("ly", 3), ("ni", 3), ("thi", 2),
            ("thon", 2), ("clu", 2), ("dri", 2), ("than", 2), ("sin", 2), ("se", 2), ("ry", 2),
            ("ron", 2), ("go", 2), ("ral", 2), ("nes", 2), ("mor", 2), ("de", 2), ("le", 2),
            ("lin", 2), ("di", 2), ("ge", 2), ("lar", 2), ("gau", 2), ("kan", 2), ("han", 2),
        ]
    }

    fn syllable_lname_count() -> &'static [(u8, usize)] {
        &[(2, 17), (3, 2), (5, 1)]
    }

    #[rustfmt::skip]
    fn syllable_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Las", 2), ("Arden", 1), ("Crown", 1), ("Cyn", 1), ("Drag", 1), ("Dre", 1),
            ("Dread", 1), ("Dup", 1), ("Flame", 1), ("Ga", 1), ("He", 1), ("Just", 1),
            ("Alaerth", 1), ("Sil", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("ka", 1), ("res", 1), ("tis", 1), ("ton", 1), ("ver", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("san", 2), ("dark", 1), ("dusk", 1), ("gore", 1), ("gue", 1), ("kesh", 1),
            ("maugh", 1), ("renth", 1), ("shield", 1), ("thyl", 1), ("thyn", 1), ("va", 1),
            ("claw", 1), ("wing", 1),
        ]
    }

    fn compound_word_probability() -> f64 {
        0.01652892561983471
    }

    #[rustfmt::skip]
    fn word_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Crown", 1), ("Drag", 1), ("Dread", 1), ("Flame", 1), ("Just", 1), ("Silver", 1),
        ]
    }

    #[rustfmt::skip]
    fn word_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("claw", 1), ("dark", 1), ("gore", 1), ("shield", 1), ("tongue", 1), ("wing", 1),
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
    use crate::world::Npc;

    #[test]
    fn gen_name_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let age = Age::Adult(0);
        let m = Gender::Masculine;
        let f = Gender::Feminine;
        let t = Gender::NonBinaryThey;

        assert_eq!(
            [
                "Astaneshor Dragva",
                "Ghonthor Crowngore",
                "Gaumadasar Dreva",
                "Bitar Lasthyl",
                "Halalorax Flamesan",
                "Akrena Ardendark",
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
        let mut npc = Npc::default();
        npc.gender.replace(*gender);
        npc.age.replace(*age);
        npc.ethnicity.replace(Ethnicity::Dragonborn);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
