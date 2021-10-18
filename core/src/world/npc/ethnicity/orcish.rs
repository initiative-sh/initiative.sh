use super::{Age, Gender, Generate, GenerateSimple};
use rand::prelude::*;

pub struct Ethnicity;

impl GenerateSimple for Ethnicity {
    fn syllable_fname_count_f() -> &'static [(u8, usize)] {
        &[(2, 21), (3, 1)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_f() -> &'static [(&'static str, usize)] {
        &[
            ("Baer", 1), ("Boo", 1), ("Cha", 1), ("Grib", 1), ("Jo", 1), ("Kreo", 1), ("Lar", 1),
            ("Lo", 1), ("Mut", 1), ("Nag", 1), ("Oo", 1), ("Quet", 1), ("Rai", 1), ("Rath", 1),
            ("Ta", 1), ("Thu", 1), ("Yag", 1), ("Zo", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_f() -> &'static [(&'static str, usize)] {
        &[
            ("ga", 2), ("da", 1), ("do", 1), ("ene", 1), ("ka", 1), ("lik", 1), ("mith", 1),
            ("ra", 1), ("ri", 1), ("riya", 1), ("rog", 1), ("ruuch", 1), ("ta", 1), ("ti", 1),
            ("war", 1), ("bla", 1), ("zel", 1),
        ]
    }

    fn syllable_fname_count_m() -> &'static [(u8, usize)] {
        &[(2, 88), (3, 9)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_m() -> &'static [(&'static str, usize)] {
        &[
            ("Ha", 3), ("Ko", 2), ("Mu", 2), ("Bi", 1), ("Bru", 1), ("Bry", 1), ("Bu", 1),
            ("Buh", 1), ("Dae", 1), ("Dul", 1), ("Glom", 1), ("Glor", 1), ("Gool", 1), ("Gre", 1),
            ("Gri", 1), ("Grim", 1), ("Grom", 1), ("Grum'", 1), ("Gur", 1), ("Har", 1), ("Ho", 1),
            ("Hrab", 1), ("Hul", 1), ("Inna", 1), ("Jo", 1), ("Khru", 1), ("Lef", 1), ("Log", 1),
            ("Mak", 1), ("Mar", 1), ("Mo", 1), ("Nau", 1), ("Nor", 1), ("Nyun", 1), ("Rag", 1),
            ("Raz", 1), ("Se", 1), ("Smae", 1), ("Thra", 1), ("Uryu", 1), ("Vag", 1), ("Ve", 1),
            ("Vhaz", 1), ("Wur", 1), ("Xa", 1), ("Ala", 1), ("Zi", 1), ("Ashkab", 1), ("Ban", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_m() -> &'static [(&'static str, usize)] {
        &[
            ("red", 2), ("baz", 1), ("bul", 1), ("chan", 1), ("dagh", 1), ("dru", 1), ("fang", 1),
            ("ga", 1), ("gan", 1), ("gen", 1), ("ghor", 1), ("gog", 1), ("gy", 1), ("hey", 1),
            ("ka", 1), ("kan", 1), ("kul", 1), ("lan", 1), ("larkh", 1), ("lis", 1), ("low", 1),
            ("luv", 1), ("moel", 1), ("neire", 1), ("nig", 1), ("nir", 1), ("nos", 1), ("raj", 1),
            ("rak", 1), ("rakt", 1), ("ram", 1), ("rash", 1), ("rath", 1), ("rell", 1), ("rim", 1),
            ("rin", 1), ("ris", 1), ("rock", 1), ("rog", 1), ("rok", 1), ("ron", 1), ("ront", 1),
            ("ror", 1), ("shar", 1), ("shnak", 1), ("thtur", 1), ("tusk", 1), ("ty", 1),
            ("vak", 1), ("vark", 1),
        ]
    }

    fn syllable_fname_count() -> &'static [(u8, usize)] {
        &[(2, 116), (3, 10)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Ha", 3), ("Glor", 2), ("Ko", 2), ("Mu", 2), ("Jo", 2), ("Boo", 1), ("Bru", 1),
            ("Bry", 1), ("Bu", 1), ("Buh", 1), ("Cha", 1), ("Dae", 1), ("Dah", 1), ("Dul", 1),
            ("Glom", 1), ("Gool", 1), ("Gre", 1), ("Gri", 1), ("Grib", 1), ("Grim", 1),
            ("Grom", 1), ("Grum'", 1), ("Gur", 1), ("Har", 1), ("Hes", 1), ("Ho", 1), ("Hrab", 1),
            ("Hul", 1), ("Inna", 1), ("Khru", 1), ("Kreo", 1), ("Lar", 1), ("Lef", 1), ("Lo", 1),
            ("Log", 1), ("Mak", 1), ("Mar", 1), ("Mo", 1), ("Mut", 1), ("Nag", 1), ("Nau", 1),
            ("Nor", 1), ("Nyun", 1), ("Oo", 1), ("Quet", 1), ("Rag", 1), ("Rai", 1), ("Rath", 1),
            ("Raz", 1), ("Ri", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last() -> &'static [(&'static str, usize)] {
        &[
            ("ga", 3), ("red", 2), ("rog", 2), ("ka", 2), ("chan", 1), ("da", 1), ("dagh", 1),
            ("do", 1), ("dru", 1), ("ene", 1), ("fang", 1), ("gan", 1), ("gen", 1), ("ghor", 1),
            ("gog", 1), ("gy", 1), ("hen", 1), ("hey", 1), ("kan", 1), ("kul", 1), ("lan", 1),
            ("larkh", 1), ("lik", 1), ("lis", 1), ("low", 1), ("luv", 1), ("ming", 1), ("mith", 1),
            ("moel", 1), ("neire", 1), ("nig", 1), ("nir", 1), ("nos", 1), ("pal", 1), ("ra", 1),
            ("raj", 1), ("rak", 1), ("rakt", 1), ("ram", 1), ("rash", 1), ("rath", 1), ("rell", 1),
            ("ren", 1), ("rgash", 1), ("ri", 1), ("rim", 1), ("rin", 1), ("ris", 1), ("riya", 1),
            ("rock", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("da", 1), ("fal", 1), ("ha", 1), ("la", 1), ("lit", 1), ("mer", 1), ("ra", 1),
            ("ro", 1), ("ry", 1), ("zi", 1),
        ]
    }

    fn syllable_lname_count() -> &'static [(u8, usize)] {
        &[(2, 34), (3, 8), (4, 1)]
    }

    #[rustfmt::skip]
    fn syllable_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Proud", 3), ("Death", 2), ("Il-", 2), ("Bes", 1), ("Black", 1), ("Blood", 1),
            ("Crack", 1), ("Elven", 1), ("Eye", 1), ("Fay", 1), ("Gnarl", 1), ("Iron", 1),
            ("Iso", 1), ("Krin", 1), ("Ma", 1), ("Mes", 1), ("Nev", 1), ("Ni", 1), ("Orc", 1),
            ("Rar", 1), ("Ren", 1), ("Stone", 1), ("The", 1), ("Three", 1), ("Thun", 1),
            ("Troll", 1), ("Xerk", 1), ("Zhu", 1), ("Anga", 1), ("Axe-", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("bac", 1), ("Bi", 1), ("da", 1), ("der", 1), ("fin", 1), ("gou", 1), ("ka", 1),
            ("ny-", 1), ("sar", 1), ("sli", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("fist", 5), ("claw", 2), ("khan", 2), ("cort", 1), ("ger", 1), ("gers", 1),
            ("hand", 1), ("head", 1), ("kat", 1), ("kel", 1), ("ker", 1), ("len", 1), ("lev", 1),
            ("lin", 1), ("mos", 1), ("ri", 1), ("rim", 1), ("roth", 1), ("sblood", 1),
            ("shale", 1), ("spear", 1), ("stil", 1), ("ta", 1), ("ter", 1), ("the", 1),
            ("Arrows", 1), ("ver", 1), ("bane", 1), ("blade", 1),
        ]
    }

    fn compound_word_probability() -> f64 {
        0.08333333333333333
    }

    #[rustfmt::skip]
    fn word_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Black", 1), ("Blood", 1), ("Crack", 1), ("Death", 1), ("Gnarl", 1), ("Iron", 1),
            ("Orc", 1), ("Proud", 1), ("Stone", 1), ("Three", 1), ("Thunder", 1),
        ]
    }

    #[rustfmt::skip]
    fn word_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("fist", 3), ("blade", 1), ("claw", 1), ("fingers", 1), ("hand", 1), ("head", 1),
            ("sliver", 1), ("backer", 1), ("spear", 1),
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
                "Harmoel Ironsarkhan",
                "Modatusk Il-the",
                "Yagzel Threefist",
                "Kreolik Ironsblood",
                "Dahkan Deathfist",
                "Jofalrak Krinri",
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
        npc.ethnicity.replace(Ethnicity::Orcish);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
