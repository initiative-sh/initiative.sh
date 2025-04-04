use super::{Age, Gender, Generate, GenerateSimple};
use rand::prelude::*;

pub struct Ethnicity;

impl Ethnicity {
    const SYLLABLE_FNAME_COUNT_CHILD: &'static [(u8, usize)] = &[(2, 20), (3, 1)];
}

impl GenerateSimple for Ethnicity {
    fn syllable_fname_count_f() -> &'static [(u8, usize)] {
        &[(2, 194), (3, 119), (4, 14), (5, 2)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_f() -> &'static [(&'static str, usize)] {
        &[
            ("Ha", 10), ("Ta", 8), ("Sha", 5), ("Me", 4), ("La", 4), ("Da", 4), ("Ki", 4),
            ("Ja", 4), ("Re", 4), ("Nu", 4), ("Ka", 4), ("Ca", 3), ("Tu", 3), ("Na", 3), ("De", 3),
            ("My", 3), ("Mi", 3), ("Li", 3), ("Va", 3), ("Ama", 2), ("Za", 2), ("Xa", 2),
            ("Tha", 2), ("Yas", 2), ("Du", 2), ("Zin", 2), ("Sa", 2), ("Aza", 2), ("Aya", 2),
            ("Mal", 2), ("Si", 2), ("Aun", 2), ("Ky", 2), ("Ly", 2), ("Erel", 2), ("Ela", 2),
            ("Fe", 2), ("Le", 2), ("Kes", 2), ("Mae", 2), ("Sin", 2), ("Ce", 1), ("Cas", 1),
            ("Asba", 1), ("Car", 1), ("Arian", 1), ("Bry", 1), ("Bres", 1), ("Aria", 1),
            ("Edher", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_f() -> &'static [(&'static str, usize)] {
        &[
            ("ra", 41), ("na", 23), ("la", 14), ("dra", 6), ("tha", 6), ("da", 6), ("sa", 5),
            ("ma", 5), ("tra", 4), ("lia", 4), ("ri", 4), ("a", 4), ("rae", 4), ("ria", 4),
            ("via", 3), ("va", 3), ("ril", 3), ("riel", 3), ("lene", 2), ("ka", 2), ("ta", 2),
            ("za", 2), ("xa", 2), ("sin", 2), ("rell", 2), ("tel", 2), ("sha", 2), ("sis", 2),
            ("rien", 2), ("rie", 2), ("ther", 2), ("rene", 2), ("fa", 2), ("dyl", 2), ("ni", 2),
            ("mi", 2), ("cia", 2), ("kal", 1), ("kah", 1), ("dria", 1), ("jin", 1), ("dar", 1),
            ("heart", 1), ("ha", 1), ("dle", 1), ("light", 1), ("gwais", 1), ("liath", 1),
            ("liane", 1), ("ghtal", 1),
        ]
    }

    fn syllable_fname_count_m() -> &'static [(u8, usize)] {
        &[(2, 375), (3, 104), (4, 7), (5, 2)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_m() -> &'static [(&'static str, usize)] {
        &[
            ("Ta", 7), ("Ma", 6), ("Ce", 6), ("Ka", 6), ("Da", 6), ("Ha", 5), ("Me", 5), ("Ra", 4),
            ("Del", 4), ("Te", 4), ("Jo", 4), ("To", 3), ("Be", 3), ("Ky", 3), ("Jan", 3),
            ("Ga", 3), ("Kha", 3), ("Na", 3), ("De", 3), ("Har", 3), ("Ke", 3), ("Ty", 3),
            ("Tan", 2), ("Si", 2), ("So", 2), ("Pae", 2), ("Ny", 2), ("Myr", 2), ("Ara", 2),
            ("Dar", 2), ("Mi", 2), ("Ri", 2), ("The", 2), ("Ki", 2), ("Kes", 2), ("Jar", 2),
            ("Ili", 2), ("Ali", 2), ("Ja", 2), ("Mal", 2), ("Gar", 2), ("Ca", 2), ("Fer", 2),
            ("Hal", 2), ("Ea", 2), ("Brin", 2), ("Elor", 2), ("Aa", 2), ("My", 2), ("Far", 2),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_m() -> &'static [(&'static str, usize)] {
        &[
            ("ran", 9), ("dar", 7), ("ril", 6), ("lar", 6), ("rian", 5), ("lis", 5), ("lin", 5),
            ("rin", 5), ("lan", 4), ("gar", 4), ("tar", 4), ("sin", 4), ("nar", 4), ("din", 4),
            ("rom", 3), ("ro", 3), ("nell", 3), ("ras", 3), ("man", 3), ("fein", 3), ("dor", 3),
            ("dan", 3), ("rath", 3), ("ten", 3), ("ros", 2), ("rol", 2), ("ryl", 2), ("rion", 2),
            ("ron", 2), ("dis", 2), ("rith", 2), ("rien", 2), ("rel", 2), ("rim", 2), ("ryn", 2),
            ("ral", 2), ("nil", 2), ("mir", 2), ("mar", 2), ("mon", 2), ("drach", 2), ("mi", 2),
            ("dorr", 2), ("lid", 2), ("phys", 2), ("rak", 2), ("lor", 2), ("don", 2), ("gan", 2),
            ("har", 2),
        ]
    }

    fn syllable_fname_count() -> &'static [(u8, usize)] {
        &[(2, 583), (3, 232), (4, 23), (5, 4)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Ta", 17), ("Ha", 16), ("Da", 10), ("Ka", 10), ("Me", 9), ("Ma", 7), ("Ce", 7),
            ("Re", 6), ("Ca", 6), ("De", 6), ("Ja", 6), ("Ki", 6), ("Sha", 6), ("Na", 6),
            ("Nu", 5), ("Va", 5), ("Mi", 5), ("Del", 5), ("My", 5), ("Te", 5), ("Ky", 5),
            ("Li", 4), ("La", 4), ("Ke", 4), ("Mal", 4), ("Ra", 4), ("Si", 4), ("Jo", 4),
            ("Kes", 4), ("Xa", 4), ("So", 4), ("Ga", 4), ("Ara", 3), ("Har", 3), ("Ty", 3),
            ("Fe", 3), ("Za", 3), ("Ny", 3), ("Be", 3), ("Ela", 3), ("Ali", 3), ("Tu", 3),
            ("Mae", 3), ("Du", 3), ("Sa", 3), ("Ly", 3), ("Hal", 3), ("Dar", 3), ("Mar", 3),
            ("To", 3),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last() -> &'static [(&'static str, usize)] {
        &[
            ("ra", 43), ("na", 24), ("la", 16), ("dar", 9), ("ran", 9), ("ril", 9), ("tha", 8),
            ("lar", 7), ("da", 7), ("ri", 6), ("dra", 6), ("sa", 6), ("sin", 6), ("ma", 6),
            ("lis", 6), ("tar", 5), ("rae", 5), ("rin", 5), ("ria", 5), ("lin", 5), ("rian", 5),
            ("tra", 5), ("lan", 4), ("lia", 4), ("nar", 4), ("a", 4), ("gar", 4), ("rien", 4),
            ("din", 4), ("mi", 4), ("rak", 3), ("ryn", 3), ("dyl", 3), ("ro", 3), ("li", 3),
            ("ten", 3), ("ni", 3), ("dor", 3), ("thir", 3), ("via", 3), ("va", 3), ("dan", 3),
            ("rith", 3), ("rom", 3), ("dis", 3), ("sha", 3), ("nell", 3), ("ras", 3), ("ryth", 3),
            ("man", 3),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("la", 17), ("li", 12), ("ri", 9), ("ra", 8), ("na", 7), ("re", 6), ("tha", 6),
            ("lan", 6), ("da", 6), ("va", 5), ("de", 5), ("le", 5), ("lae", 5), ("ma", 4),
            ("sa", 4), ("lu", 3), ("te", 3), ("rin", 3), ("vi", 3), ("ha", 3), ("rae", 3),
            ("ryn", 3), ("ve", 3), ("ry", 2), ("ram", 2), ("lag", 2), ("mi", 2), ("laa", 2),
            ("ka", 2), ("than", 2), ("ria", 2), ("thar", 2), ("lyn", 2), ("fe", 2), ("ver", 2),
            ("sha", 2), ("fa", 2), ("gar", 2), ("lei", 2), ("a", 2), ("do", 2), ("ca", 2),
            ("ro", 2), ("ni", 2), ("mal", 2), ("se", 2), ("di", 2), ("ran", 2), ("thoe", 1),
            ("via", 1),
        ]
    }

    fn syllable_lname_count() -> &'static [(u8, usize)] {
        &[(2, 348), (3, 225), (4, 17)]
    }

    #[rustfmt::skip]
    fn syllable_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("al-", 0), ("Sil", 9), ("Moon", 9), ("Du", 8), ("Sta", 7), ("Iri", 7), ("Ta", 6),
            ("Ca", 6), ("Star", 6), ("Ma", 5), ("Miz", 5), ("Ha", 5), ("Eva", 5), ("Hal", 5),
            ("Me", 5), ("Dus", 5), ("Ilda", 5), ("Win", 4), ("Wind", 4), ("Gol", 4), ("Hawk", 4),
            ("Ae", 4), ("Alas", 4), ("Sym", 4), ("Snow", 4), ("Zo", 3), ("Le", 3), ("Van", 3),
            ("Ko", 3), ("Le'", 3), ("T'", 3), ("Sun", 3), ("Black", 3), ("Tlab", 3), ("Shin", 3),
            ("Se", 3), ("Sha", 3), ("H'", 3), ("Oblod", 3), ("Green", 3), ("Ri", 3), ("Far", 3),
            ("Fire", 3), ("Ni", 3), ("Flo", 3), ("Mi", 3), ("Night", 3), ("Lu", 2), ("Bright", 2),
            ("Kha", 2),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("ver", 11), ("la", 8), ("ro", 8), ("na", 7), ("ra", 7), ("wa", 5), ("Ka", 5),
            ("den", 4), ("mas", 4), ("ter", 4), ("me", 4), ("trar", 3), ("ta", 3), ("ti", 3),
            ("ryn", 3), ("ryv", 3), ("ri", 3), ("rian", 3), ("Quel", 3), ("laud", 3), ("ma", 3),
            ("wal", 3), ("va", 2), ("a", 2), ("Ta", 2), ("sar", 2), ("le", 2), ("Bran", 2),
            ("Sa", 2), ("ven", 2), ("si", 2), ("lar", 2), ("lan", 2), ("ree", 2), ("nor'", 2),
            ("blos", 2), ("dra", 2), ("Na", 2), ("mer", 2), ("mel", 2), ("ni", 2), ("re", 2),
            ("ing", 2), ("ar", 2), ("do", 2), ("man", 2), ("no", 2), ("sa", 2), ("sad", 2),
            ("win", 2),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("ra", 21), ("thil", 12), ("ter", 11), ("tar", 9), ("song", 9), ("bow", 8), ("thyl", 8),
            ("rym", 7), ("leaf", 6), ("cer", 6), ("spear", 6), ("dar", 6), ("bar", 5), ("kryn", 5),
            ("zrym", 5), ("lond", 4), ("rim", 4), ("vin", 4), ("star", 4), ("ker", 4), ("reth", 3),
            ("sar", 3), ("cloak", 3), ("lis", 3), ("dree", 3), ("larn", 3), ("la", 3), ("shin", 3),
            ("rin", 3), ("ryn", 3), ("som", 3), ("did", 2), ("breeze", 2), ("rel", 2), ("ren", 2),
            ("ri", 2), ("phiir", 2), ("branch", 2), ("nuath", 2), ("ner", 2), ("niv", 2),
            ("orgh", 2), ("ran", 2), ("long", 2), ("lin", 2), ("len", 2), ("liom", 2), ("lor", 2),
            ("kin", 2), ("math", 2),
        ]
    }

    fn compound_word_probability() -> f64 {
        0.14977477477477477
    }

    #[rustfmt::skip]
    fn word_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Moon", 8), ("Star", 6), ("Winter", 4), ("Wind", 4), ("Silver", 4), ("Snow", 4),
            ("Golden", 4), ("Night", 3), ("Fire", 3), ("Black", 3), ("Green", 3), ("Far", 2),
            ("Tree", 2), ("Bay", 2), ("True", 2), ("Dusk", 2), ("Mist", 2), ("Blues", 2),
            ("Haven", 2), ("White", 2), ("Sun", 2), ("Bright", 2), ("Winds", 2), ("Hawk", 2),
            ("River", 2), ("Mane", 1), ("Mel", 1), ("Minstrel", 1), ("Miri", 1), ("Morning", 1),
            ("Nights", 1), ("Oak", 1), ("Pars", 1), ("Quiver", 1), ("Rally", 1), ("Red", 1),
            ("Reef", 1), ("Rune", 1), ("Sea", 1), ("Shade", 1), ("Shadow", 1), ("Shin", 1),
            ("Shivers", 1), ("Spell", 1), ("Spring", 1), ("Still", 1), ("Storm", 1), ("Strong", 1),
            ("Sure", 1), ("Sweet", 1),
        ]
    }

    #[rustfmt::skip]
    fn word_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("song", 7), ("bow", 5), ("leaf", 5), ("star", 4), ("tar", 4), ("spear", 4),
            ("water", 4), ("walker", 3), ("master", 3), ("cloak", 3), ("spoon", 2), ("blossom", 2),
            ("down", 2), ("moon", 2), ("hand", 2), ("mantle", 2), ("bars", 2), ("wind", 2),
            ("winter", 2), ("breeze", 2), ("word", 2), ("harp", 1), ("heart", 1), ("helm", 1),
            ("horn", 1), ("hound", 1), ("ira", 1), ("kin", 1), ("leap", 1), ("lock", 1),
            ("long", 1), ("lost", 1), ("maine", 1), ("mane", 1), ("meadow", 1), ("melt", 1),
            ("mer", 1), ("methyl", 1), ("mis", 1), ("moor", 1), ("pear", 1), ("river", 1),
            ("root", 1), ("rose", 1), ("runner", 1), ("ruth", 1), ("seal", 1), ("seed", 1),
            ("shadow", 1), ("sheaf", 1),
        ]
    }
}

impl Generate for Ethnicity {
    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        format!(
            "{} {}",
            match age {
                Age::Infant | Age::Child | Age::Adolescent => {
                    super::gen_name(
                        rng,
                        Self::SYLLABLE_FNAME_COUNT_CHILD,
                        Self::syllable_fname_first(),
                        Self::syllable_fname_middle(),
                        Self::syllable_fname_last(),
                    )
                }
                _ => Self::gen_fname_simple(rng, gender),
            },
            Self::gen_lname_simple(rng),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::world::npc::ethnicity::{test_utils as test, Ethnicity};

    use Age::{Adolescent, Adult, Child, Infant};
    use Ethnicity::Elvish;
    use Gender::{Feminine, Masculine, NonBinaryThey};

    #[test]
    fn gen_name_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            [
                "Sharyth Mitasom",
                "Marlin Shiverstar",
                "Arani Luversom",
                "Tagar Evasong",
                "Taros Dusthyl",
                "Catel Irirolarn",
                "Meleri Duleaf",
                "Taten Shinnomath",
                "Kina Riwareth",
            ],
            [
                test::gen_name(&mut rng, Elvish, Infant, Masculine),
                test::gen_name(&mut rng, Elvish, Child, Feminine),
                test::gen_name(&mut rng, Elvish, Adolescent, NonBinaryThey),
                test::gen_name(&mut rng, Elvish, Adult, Masculine),
                test::gen_name(&mut rng, Elvish, Adult, Masculine),
                test::gen_name(&mut rng, Elvish, Adult, Feminine),
                test::gen_name(&mut rng, Elvish, Adult, Feminine),
                test::gen_name(&mut rng, Elvish, Adult, NonBinaryThey),
                test::gen_name(&mut rng, Elvish, Adult, NonBinaryThey),
            ],
        );
    }
}
