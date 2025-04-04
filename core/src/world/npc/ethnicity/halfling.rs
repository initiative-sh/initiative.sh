use super::{Age, Gender, Generate, GenerateSimple};
use rand::prelude::*;

pub struct Ethnicity;

impl GenerateSimple for Ethnicity {
    fn syllable_fname_count_f() -> &'static [(u8, usize)] {
        &[(2, 34), (3, 15)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_f() -> &'static [(&'static str, usize)] {
        &[
            ("No", 2), ("Ali", 1), ("Alo", 1), ("Ama", 1), ("Bah", 1), ("Blai", 1), ("Brin", 1),
            ("Cas", 1), ("Cel", 1), ("Cyn", 1), ("Da", 1), ("Dhar", 1), ("Don", 1), ("Dwah", 1),
            ("Eli", 1), ("Fa", 1), ("Gel", 1), ("Gret", 1), ("Grin", 1), ("Indee", 1), ("Ja", 1),
            ("Jar", 1), ("Ka", 1), ("Mac", 1), ("Mane", 1), ("Maz", 1), ("Mer", 1), ("Mil", 1),
            ("Mit", 1), ("Naa", 1), ("Nur", 1), ("Pe", 1), ("Qel", 1), ("Se", 1), ("Shan", 1),
            ("Adi", 1), ("Si", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_f() -> &'static [(&'static str, usize)] {
        &[
            ("ra", 3), ("ma", 3), ("la", 3), ("li", 2), ("sa", 2), ("da", 2), ("lek", 1),
            ("line", 1), ("man", 1), ("miyah", 1), ("na", 1), ("nas", 1), ("ni", 1), ("ri", 1),
            ("sha", 1), ("sra", 1), ("su", 1), ("ter", 1), ("va", 1), ("vel", 1), ("wile", 1),
            ("zette", 1), ("ca", 1), ("zy", 1), ("chen", 1), ("dell", 1), ("die", 1), ("dine", 1),
            ("lea", 1),
        ]
    }

    fn syllable_fname_count_m() -> &'static [(u8, usize)] {
        &[(2, 101), (3, 32), (4, 3)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_m() -> &'static [(&'static str, usize)] {
        &[
            ("Jo", 3), ("Ra", 2), ("Me", 2), ("Fo", 2), ("Da", 2), ("Wig", 2), ("Bur", 2),
            ("Pi", 2), ("Fa", 2), ("Stan", 2), ("Not", 2), ("Los", 2), ("Brox", 1), ("Buc", 1),
            ("Buck", 1), ("Bul", 1), ("Bun", 1), ("Ca", 1), ("Cal", 1), ("Ce", 1), ("Cor", 1),
            ("Dal", 1), ("Dar", 1), ("De", 1), ("Do", 1), ("Dun", 1), ("Ei", 1), ("Este", 1),
            ("Fal", 1), ("Far", 1), ("Fla", 1), ("Fre", 1), ("Ful", 1), ("Gha", 1), ("Gil", 1),
            ("Gon", 1), ("Gor", 1), ("Gris", 1), ("Grit", 1), ("Guy", 1), ("Ha", 1), ("Hae", 1),
            ("Ham", 1), ("Ho", 1), ("Hu", 1), ("Hud", 1), ("Jar", 1), ("Jen", 1), ("Kel", 1),
            ("Ki", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_m() -> &'static [(&'static str, usize)] {
        &[
            ("mur", 2), ("my", 2), ("ser", 2), ("ran", 2), ("do", 2), ("ner", 2), ("lon", 2),
            ("der", 2), ("kin", 2), ("ron", 2), ("man", 2), ("lo", 2), ("ryl", 2), ("fry", 1),
            ("gan", 1), ("gar", 1), ("gas", 1), ("gast", 1), ("gers", 1), ("go", 1), ("gope", 1),
            ("gord", 1), ("ha", 1), ("har", 1), ("hock", 1), ("id", 1), ("ka", 1), ("kal", 1),
            ("klal", 1), ("kus", 1), ("kuth", 1), ("las", 1), ("ley", 1), ("liam", 1), ("lian", 1),
            ("lias", 1), ("lij", 1), ("loe", 1), ("long", 1), ("mal", 1), ("max", 1), ("mi", 1),
            ("mien", 1), ("mir", 1), ("mon", 1), ("muck", 1), ("muel", 1), ("nas", 1), ("no", 1),
            ("noc", 1),
        ]
    }

    fn syllable_fname_count() -> &'static [(u8, usize)] {
        &[(2, 139), (3, 48), (4, 4)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Fa", 3), ("Jo", 3), ("Da", 3), ("Ra", 2), ("Bur", 2), ("Wig", 2), ("Pi", 2),
            ("Fo", 2), ("Stan", 2), ("Pe", 2), ("Jar", 2), ("Me", 2), ("Si", 2), ("Los", 2),
            ("Not", 2), ("No", 2), ("Blai", 1), ("Bhe", 1), ("Avim", 1), ("Ber", 1), ("Bun", 1),
            ("Bel", 1), ("Ather", 1), ("Alme", 1), ("Ca", 1), ("Cal", 1), ("Cas", 1), ("Ce", 1),
            ("Cel", 1), ("Chen", 1), ("Bul", 1), ("Buck", 1), ("Be", 1), ("Buc", 1), ("Brox", 1),
            ("Ban", 1), ("Ama", 1), ("Do", 1), ("Don", 1), ("Dun", 1), ("Dwah", 1), ("Ei", 1),
            ("Eli", 1), ("Este", 1), ("Dhar", 1), ("Fal", 1), ("Far", 1), ("Fla", 1), ("Brin", 1),
            ("De", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last() -> &'static [(&'static str, usize)] {
        &[
            ("la", 4), ("ra", 4), ("ma", 3), ("man", 3), ("nas", 2), ("do", 2), ("ryl", 2),
            ("na", 2), ("ter", 2), ("sa", 2), ("ri", 2), ("kin", 2), ("lon", 2), ("lo", 2),
            ("my", 2), ("mur", 2), ("da", 2), ("ni", 2), ("ron", 2), ("ser", 2), ("li", 2),
            ("der", 2), ("ran", 2), ("ner", 2), ("dine", 1), ("gope", 1), ("go", 1), ("dil", 1),
            ("gord", 1), ("ha", 1), ("chet", 1), ("gers", 1), ("gast", 1), ("die", 1), ("gas", 1),
            ("lian", 1), ("gar", 1), ("chen", 1), ("bas", 1), ("gan", 1), ("ley", 1), ("lek", 1),
            ("fry", 1), ("dell", 1), ("lea", 1), ("liam", 1), ("las", 1), ("lias", 1), ("lij", 1),
            ("fred", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("li", 4), ("la", 3), ("de", 2), ("do", 2), ("ra", 2), ("ri", 2), ("ta", 2), ("fal", 1),
            ("fin", 1), ("gar", 1), ("gle", 1), ("hoo", 1), ("i", 1), ("kes", 1), ("kit", 1),
            ("lab", 1), ("lan", 1), ("le", 1), ("lon", 1), ("me", 1), ("mo", 1), ("na", 1),
            ("no", 1), ("ral", 1), ("ran", 1), ("re", 1), ("rim", 1), ("rin", 1), ("roel", 1),
            ("ry", 1), ("stan", 1), ("sze", 1), ("tha", 1), ("ti", 1), ("tle", 1), ("va", 1),
            ("ver", 1), ("wea", 1), ("bi", 1), ("xi", 1), ("ci", 1), ("cit", 1), ("co", 1),
            ("da", 1), ("di", 1), ("dri", 1),
        ]
    }

    fn syllable_lname_count() -> &'static [(u8, usize)] {
        &[(2, 53), (3, 70), (4, 7)]
    }

    #[rustfmt::skip]
    fn syllable_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Net", 6), ("al-", 0), ("Mins", 5), ("To", 3), ("Long", 3), ("Bright", 2), ("Fen", 2),
            ("Under", 2), ("Grim", 2), ("Hard", 2), ("Wan", 2), ("Pa", 2), ("Hill", 2),
            ("Alder", 2), ("Mi", 2), ("Three-", 2), ("Ber", 2), ("Fal", 2), ("Din", 1),
            ("Ember", 1), ("Fa", 1), ("Fair", 1), ("Fat", 1), ("Fer", 1), ("Fir", 1), ("Flame", 1),
            ("Fle", 1), ("Free", 1), ("Gled", 1), ("High", 1), ("Jal", 1), ("Ji", 1), ("Keen", 1),
            ("Khay", 1), ("Kos", 1), ("Light", 1), ("List", 1), ("Lum", 1), ("Ma", 1), ("Mir", 1),
            ("Mis", 1), ("Moa", 1), ("Moon", 1), ("Nes", 1), ("Noa", 1), ("One", 1), ("Par", 1),
            ("Peb", 1), ("Proud", 1), ("Rai", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("tle", 8), ("trel", 5), ("li", 4), ("der", 3), ("ble", 3), ("ry", 2), ("top", 2),
            ("ra", 2), ("shac", 2), ("wil", 2), ("na", 2), ("min", 2), ("ter", 2), ("po", 2),
            ("gir", 1), ("ing", 1), ("kal", 1), ("ket", 1), ("la", 1), ("las", 1), ("low", 1),
            ("ma", 1), ("mer", 1), ("Mu", 1), ("nes", 1), ("ni", 1), ("no", 1), ("o", 1),
            ("Ba", 1), ("ram", 1), ("Ri", 1), ("ring", 1), ("ro", 1), ("rul", 1), ("sa", 1),
            ("slee", 1), ("stock", 1), ("tan", 1), ("tar", 1), ("tel", 1), ("tfif", 1),
            ("thyst", 1), ("whis", 1), ("rab", 1), ("bo", 1), ("bot", 1), ("ca", 1), ("daw", 1),
            ("del", 1), ("do", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("foot", 6), ("bee", 6), ("buck", 6), ("wish", 5), ("tle", 3), ("kle", 2), ("hair", 2),
            ("ra", 2), ("ple", 2), ("leaf", 2), ("ter", 2), ("low", 2), ("bell", 2), ("si", 2),
            ("no", 2), ("pipe", 2), ("pip", 2), ("dyn", 1), ("ear", 1), ("fin", 1), ("fir", 1),
            ("fle", 1), ("fowe", 1), ("garth", 1), ("gers", 1), ("glar", 1), ("glow", 1),
            ("gost", 1), ("green", 1), ("heart", 1), ("heel", 1), ("huck", 1), ("ing", 1),
            ("jen", 1), ("kard", 1), ("ker", 1), ("kes", 1), ("klav", 1), ("knife", 1), ("kyn", 1),
            ("lies", 1), ("lows", 1), ("luck", 1), ("ma", 1), ("mar", 1), ("moth", 1), ("muck", 1),
            ("na", 1), ("nes", 1), ("nin", 1),
        ]
    }

    fn compound_word_probability() -> f64 {
        0.28846153846153844
    }

    #[rustfmt::skip]
    fn word_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Long", 3), ("Under", 2), ("Hill", 2), ("Apple", 1), ("Berry", 1), ("Bold", 1),
            ("Brace", 1), ("Bramble", 1), ("Brandy", 1), ("Bright", 1), ("Brown", 1), ("Card", 1),
            ("Clay", 1), ("Din", 1), ("Ember", 1), ("Fair", 1), ("Fat", 1), ("Flame", 1),
            ("Free", 1), ("Grim", 1), ("Hard", 1), ("Harding", 1), ("High", 1), ("Keen", 1),
            ("Light", 1), ("List", 1), ("Minstrel", 1), ("Moon", 1), ("Nestle", 1), ("Nettle", 1),
            ("One", 1), ("Para", 1), ("Pebble", 1), ("Proud", 1), ("Rich", 1), ("Rumble", 1),
            ("Scat", 1), ("Short", 1), ("Sore", 1), ("Strong", 1), ("Summer", 1), ("Tall", 1),
            ("Tar", 1), ("Tell", 1), ("Thistle", 1), ("Thunder", 1), ("Tin", 1), ("Trees", 1),
            ("Vermin", 1), ("Wander", 1),
        ]
    }

    #[rustfmt::skip]
    fn word_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("foot", 5), ("hair", 2), ("pipe", 2), ("topple", 2), ("bell", 1), ("bones", 1),
            ("bough", 1), ("brook", 1), ("buck", 1), ("bug", 1), ("burp", 1), ("cheese", 1),
            ("dale", 1), ("dower", 1), ("ear", 1), ("fellow", 1), ("fingers", 1), ("fir", 1),
            ("fitter", 1), ("gallows", 1), ("garth", 1), ("girdle", 1), ("glow", 1), ("green", 1),
            ("heart", 1), ("heel", 1), ("huck", 1), ("kettle", 1), ("knife", 1), ("leaf", 1),
            ("luck", 1), ("nose", 1), ("over", 1), ("pip", 1), ("pole", 1), ("rabbit", 1),
            ("rot", 1), ("shackle", 1), ("sleeves", 1), ("spur", 1), ("stocking", 1),
            ("tankard", 1), ("ter", 1), ("thumb", 1), ("toe", 1), ("toes", 1), ("tree", 1),
            ("tump", 1), ("wine", 1), ("wink", 1),
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
mod test {
    use super::*;
    use crate::world::npc::ethnicity::{test_utils as test, Ethnicity};

    use Age::Adult;
    use Ethnicity::Halfling;
    use Gender::{Feminine, Masculine, NonBinaryThey};

    #[test]
    fn gen_name_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            [
                "Buncicohock Listnes",
                "Hudder Claypole",
                "Brindell Netwiling",
                "Gelter Listmabee",
                "Wigdeha Appletopple",
                "Sirimra Panaknife",
            ],
            [
                test::gen_name(&mut rng, Halfling, Adult, Masculine),
                test::gen_name(&mut rng, Halfling, Adult, Masculine),
                test::gen_name(&mut rng, Halfling, Adult, Feminine),
                test::gen_name(&mut rng, Halfling, Adult, Feminine),
                test::gen_name(&mut rng, Halfling, Adult, NonBinaryThey),
                test::gen_name(&mut rng, Halfling, Adult, NonBinaryThey),
            ],
        );
    }
}
