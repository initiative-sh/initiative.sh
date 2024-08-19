use super::{Age, Gender, Generate, GenerateSimple};
use rand::prelude::*;

pub struct Ethnicity;

impl GenerateSimple for Ethnicity {
    fn syllable_fname_count_f() -> &'static [(u8, usize)] {
        &[(2, 11), (3, 7), (4, 1)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_f() -> &'static [(&'static str, usize)] {
        &[
            ("Alfi", 1), ("Bry", 1), ("Ha", 1), ("He", 1), ("Ka", 1), ("Lo", 1), ("Mac", 1),
            ("Nar", 1), ("Nee", 1), ("Ny", 1), ("Ora", 1), ("Rae", 1), ("Sfe", 1), ("Syl", 1),
            ("Te", 1), ("Va", 1), ("Xal", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_f() -> &'static [(&'static str, usize)] {
        &[
            ("na", 2), ("ra", 2), ("fer", 1), ("la", 1), ("lar", 1), ("leen", 1), ("lia", 1),
            ("lis", 1), ("nosh", 1), ("sa", 1), ("seis", 1), ("shka", 1), ("cath", 1), ("vyre", 1),
            ("del", 1),
        ]
    }

    fn syllable_fname_count_m() -> &'static [(u8, usize)] {
        &[(2, 18), (3, 2)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_m() -> &'static [(&'static str, usize)] {
        &[
            ("Bil", 1), ("Da", 1), ("Go", 1), ("Haer'", 1), ("Her", 1), ("Ia", 1), ("Ka", 1),
            ("Kad", 1), ("Ma", 1), ("Mor", 1), ("No", 1), ("Pyn", 1), ("Ral", 1), ("Squid", 1),
            ("Va", 1), ("Vi", 1), ("Zev", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_m() -> &'static [(&'static str, usize)] {
        &[
            ("roth", 2), ("dai", 1), ("dly", 1), ("dos", 1), ("kos", 1), ("len", 1), ("lio", 1),
            ("lis", 1), ("lor", 1), ("mam", 1), ("mays", 1), ("non", 1), ("rim", 1), ("tuor", 1),
            ("chorn", 1), ("zgo", 1),
        ]
    }

    fn syllable_fname_count() -> &'static [(u8, usize)] {
        &[(2, 31), (3, 10), (4, 1)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Va", 2), ("Ka", 2), ("Bry", 1), ("Da", 1), ("Go", 1), ("Ha", 1), ("Haer'", 1),
            ("He", 1), ("Her", 1), ("Ia", 1), ("Jer", 1), ("Kad", 1), ("Lo", 1), ("Ma", 1),
            ("Mac", 1), ("Mor", 1), ("Nar", 1), ("Nee", 1), ("No", 1), ("Ny", 1), ("Ora", 1),
            ("Par", 1), ("Pyn", 1), ("Rae", 1), ("Ral", 1), ("Sfe", 1), ("Squid", 1), ("Syl", 1),
            ("Te", 1), ("Vi", 1), ("Xal", 1), ("Alfi", 1), ("Zev", 1), ("Bil", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last() -> &'static [(&'static str, usize)] {
        &[
            ("lis", 2), ("roth", 2), ("na", 2), ("ra", 2), ("del", 1), ("der", 1), ("dly", 1),
            ("dos", 1), ("fer", 1), ("kos", 1), ("la", 1), ("lar", 1), ("leen", 1), ("len", 1),
            ("lia", 1), ("lio", 1), ("lor", 1), ("mam", 1), ("mays", 1), ("non", 1), ("nosh", 1),
            ("rim", 1), ("sa", 1), ("seis", 1), ("shka", 1), ("tuor", 1), ("vyre", 1), ("bid", 1),
            ("zgo", 1), ("cath", 1), ("chorn", 1), ("dai", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("ra", 2), ("vi", 2), ("lia", 1), ("man", 1), ("mes", 1), ("Da", 1), ("phe", 1),
            ("ris", 1), ("ni", 1), ("dan", 1),
        ]
    }

    fn syllable_lname_count() -> &'static [(u8, usize)] {
        &[(2, 14), (3, 3)]
    }

    #[rustfmt::skip]
    fn syllable_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Aleg", 1), ("de", 1), ("Gul", 1), ("Imp", 1), ("Ka", 1), ("Rii", 1), ("Sa", 1),
            ("Sha", 1), ("Vroc", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("dow", 1), ("kis", 1), ("vi", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("breath", 1), ("kas", 1), ("kith", 1), ("kwing", 1), ("maern", 1), ("ni", 1),
            ("tos", 1), ("Vore", 1), ("zar", 1),
        ]
    }

    fn compound_word_probability() -> f64 {
        0.
    }

    fn word_lname_first() -> &'static [(&'static str, usize)] {
        &[]
    }

    fn word_lname_last() -> &'static [(&'static str, usize)] {
        &[]
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
        let adult = Age::Adult;
        let m = Gender::Masculine;
        let f = Gender::Feminine;
        let t = Gender::NonBinaryThey;

        assert_eq!(
            [
                "Vidandos Shatos",
                "Hevivyre Alegkwing",
                "Bilmays Katos",
                "Narra demaern",
                "Matuor Shazar",
                "Norislor Gulkwing",
                "Tefer Shavitos",
                "Orara deVore",
                "Zevlia Sazar",
                "Goroth Alegtos",
            ],
            [
                gen_name(&mut rng, &Age::Infant, &m),
                gen_name(&mut rng, &Age::Child, &f),
                gen_name(&mut rng, &Age::Adolescent, &m),
                gen_name(&mut rng, &Age::YoungAdult, &f),
                gen_name(&mut rng, &adult, &m),
                gen_name(&mut rng, &adult, &m),
                gen_name(&mut rng, &adult, &f),
                gen_name(&mut rng, &adult, &f),
                gen_name(&mut rng, &adult, &t),
                gen_name(&mut rng, &adult, &t),
            ],
        );
    }

    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        let mut npc = NpcData::default();
        npc.gender.replace(*gender);
        npc.age.replace(*age);
        npc.ethnicity.replace(Ethnicity::Tiefling);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
