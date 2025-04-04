use super::{Age, Gender, Generate, GenerateSimple};
use rand::prelude::*;

pub struct Ethnicity;

impl GenerateSimple for Ethnicity {
    fn syllable_fname_count_f() -> &'static [(u8, usize)] {
        &[(2, 19), (3, 12), (4, 1), (7, 1)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_f() -> &'static [(&'static str, usize)] {
        &[
            ("Se", 2), ("Wa", 2), ("Blin", 1), ("Da", 1), ("Del", 1), ("Gel", 1), ("Gin", 1),
            ("Ha", 1), ("Hen", 1), ("Jal", 1), ("Jam", 1), ("Joyel", 1), ("Lan", 1), ("Ma", 1),
            ("Mav", 1), ("Mrel", 1), ("Nai", 1), ("No", 1), ("Ruk", 1), ("Sen", 1), ("Sto", 1),
            ("Tap", 1), ("Toh", 1), ("Vil", 1), ("Ya", 1), ("Yan", 1), ("Yo", 1), ("Angha", 1),
            ("Za", 1), ("Aria", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_f() -> &'static [(&'static str, usize)] {
        &[
            ("la", 6), ("na", 3), ("ni", 2), ("ki", 2), ("kkal", 1), ("leed", 1), ("less", 1),
            ("lie", 1), ("mi", 1), ("moth", 1), ("nah", 1), ("net", 1), ("nia", 1), ("py", 1),
            ("ra", 1), ("ran", 1), ("tha", 1), ("thee", 1), ("a", 1), ("tow", 1), ("del", 1),
            ("drin", 1), ("isle", 1),
        ]
    }

    fn syllable_fname_count_m() -> &'static [(u8, usize)] {
        &[(2, 71), (3, 19), (4, 2)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_m() -> &'static [(&'static str, usize)] {
        &[
            ("Fil", 2), ("Gur", 2), ("Dor", 2), ("Pe", 2), ("Ha", 2), ("Bar", 1), ("Ben", 1),
            ("Bils", 1), ("Bod", 1), ("Bof", 1), ("Bran", 1), ("Bric", 1), ("Bu", 1), ("Da", 1),
            ("Dir", 1), ("Elis", 1), ("Fal", 1), ("Fit", 1), ("Fla", 1), ("Gim", 1), ("Gly", 1),
            ("Grab", 1), ("Grob", 1), ("Gus", 1), ("Hei", 1), ("Ho", 1), ("Jim", 1), ("Jou", 1),
            ("Ka", 1), ("Kar", 1), ("Kor", 1), ("Krie", 1), ("Lur", 1), ("Ma", 1), ("Mak", 1),
            ("Mig", 1), ("Mu", 1), ("Nan", 1), ("Nor", 1), ("Peeb", 1), ("Pin", 1), ("Prit", 1),
            ("Pur", 1), ("Ra", 1), ("Ril", 1), ("Ror", 1), ("Rug", 1), ("Rund", 1), ("San", 1),
            ("Skee", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_m() -> &'static [(&'static str, usize)] {
        &[
            ("ka", 2), ("den", 2), ("rinn", 2), ("lin", 2), ("ry", 2), ("cere", 1), ("chard", 1),
            ("cus", 1), ("darn", 1), ("das", 1), ("des", 1), ("do", 1), ("dor", 1), ("dyn", 1),
            ("fal", 1), ("fbin", 1), ("fiz", 1), ("fo", 1), ("foodle", 1), ("garth", 1),
            ("ger", 1), ("gien", 1), ("gor", 1), ("grog", 1), ("gtu", 1), ("gus", 1), ("har", 1),
            ("jar", 1), ("kers", 1), ("kik", 1), ("krist", 1), ("kyl", 1), ("lan", 1), ("les", 1),
            ("lian", 1), ("liyun", 1), ("lo", 1), ("lob", 1), ("mas", 1), ("min", 1), ("mir", 1),
            ("mo", 1), ("nar", 1), ("net", 1), ("nik", 1), ("no", 1), ("nock", 1), ("phic", 1),
            ("pin", 1), ("roo", 1),
        ]
    }

    fn syllable_fname_count() -> &'static [(u8, usize)] {
        &[(2, 96), (3, 33), (4, 3), (7, 1)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Ha", 3), ("Ma", 3), ("Wa", 2), ("Dor", 2), ("Pe", 2), ("Ka", 2), ("Fil", 2),
            ("Gur", 2), ("Ben", 2), ("Se", 2), ("Da", 2), ("Bod", 1), ("Bof", 1), ("Brae", 1),
            ("Bran", 1), ("Bric", 1), ("Bu", 1), ("Del", 1), ("Dir", 1), ("Elis", 1), ("Fal", 1),
            ("Fan", 1), ("Fit", 1), ("Fla", 1), ("Gel", 1), ("Gim", 1), ("Gin", 1), ("Gly", 1),
            ("Grab", 1), ("Grob", 1), ("Gus", 1), ("Hei", 1), ("Hen", 1), ("Ho", 1), ("Jal", 1),
            ("Jam", 1), ("Jim", 1), ("Jou", 1), ("Joyel", 1), ("Kar", 1), ("Kor", 1), ("Krie", 1),
            ("Lan", 1), ("Lur", 1), ("Mak", 1), ("Mav", 1), ("Mig", 1), ("Mrel", 1), ("Mu", 1),
            ("Nai", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last() -> &'static [(&'static str, usize)] {
        &[
            ("la", 6), ("lin", 3), ("na", 3), ("rinn", 2), ("ka", 2), ("ry", 2), ("ni", 2),
            ("net", 2), ("den", 2), ("ki", 2), ("daer", 1), ("darn", 1), ("das", 1), ("del", 1),
            ("der", 1), ("des", 1), ("do", 1), ("dor", 1), ("drin", 1), ("dyn", 1), ("fal", 1),
            ("fbin", 1), ("fiz", 1), ("fo", 1), ("foodle", 1), ("garth", 1), ("ger", 1),
            ("gien", 1), ("gor", 1), ("grog", 1), ("gtu", 1), ("gus", 1), ("har", 1), ("isle", 1),
            ("jar", 1), ("kers", 1), ("kik", 1), ("kkal", 1), ("krist", 1), ("kyl", 1), ("lah", 1),
            ("lan", 1), ("leed", 1), ("les", 1), ("less", 1), ("lian", 1), ("lie", 1),
            ("liyun", 1), ("lo", 1), ("lob", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("na", 3), ("ra", 3), ("la", 2), ("de", 2), ("ri", 2), ("ble", 2), ("he", 1),
            ("hel", 1), ("i", 1), ("jus", 1), ("ka", 1), ("ko", 1), ("krom", 1), ("lak", 1),
            ("li", 1), ("lis", 1), ("lo", 1), ("lus", 1), ("ly", 1), ("mad", 1), ("mi", 1),
            ("ner", 1), ("nol", 1), ("sul", 1), ("ta", 1), ("tad", 1), ("tian", 1), ("to", 1),
            ("ven", 1), ("ban", 1), ("zmil", 1), ("da", 1), ("di'", 1), ("dyk", 1), ("ga", 1),
            ("ge", 1),
        ]
    }

    fn syllable_lname_count() -> &'static [(u8, usize)] {
        &[(2, 43), (3, 45), (4, 10), (5, 1), (6, 1)]
    }

    #[rustfmt::skip]
    fn syllable_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Mir", 3), ("Fid", 2), ("Burr", 2), ("Lead", 2), ("Dig", 2), ("Gar", 2), ("Bit", 1),
            ("Bright", 1), ("Bul", 1), ("Bus", 1), ("Cher", 1), ("Clam", 1), ("Coax", 1),
            ("Cos", 1), ("Crac", 1), ("Daer", 1), ("Dis", 1), ("Doog", 1), ("Dream", 1),
            ("Drip", 1), ("Faya", 1), ("Fi", 1), ("Flint", 1), ("Foam", 1), ("Gleam", 1),
            ("Glinc", 1), ("Glit", 1), ("Gnar", 1), ("Gnome", 1), ("Gra", 1), ("Grea", 1),
            ("Haerl", 1), ("Har", 1), ("Het", 1), ("ibn-", 1), ("Iron", 1), ("Jas", 1),
            ("Knob", 1), ("Kov", 1), ("Kres", 1), ("Lar", 1), ("Le", 1), ("Me", 1), ("Min", 1),
            ("Morn", 1), ("Muc", 1), ("Nat", 1), ("Nog", 1), ("Path", 1), ("Pe", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("der", 3), ("ror", 3), ("ben", 2), ("dle", 2), ("war", 2), ("mat", 2), ("stop", 2),
            ("dner", 2), ("bar", 2), ("ger", 2), ("hun", 1), ("ing", 1), ("je", 1), ("kan", 1),
            ("ke", 1), ("ke", 1), ("ked", 1), ("ket", 1), ("kled", 1), ("knap", 1), ("knoc", 1),
            ("ktap", 1), ("kun", 1), ("li", 1), ("lwub", 1), ("ly", 1), ("man", 1), ("mer", 1),
            ("mi", 1), ("na", 1), ("per", 1), ("re", 1), ("ros", 1), ("row", 1), ("sen", 1),
            ("shut", 1), ("sil", 1), ("sin", 1), ("skil", 1), ("sten", 1), ("tel", 1), ("ten", 1),
            ("ters", 1), ("tlef", 1), ("vel", 1), ("ven", 1), ("wea", 1), ("wil", 1), ("woc", 1),
            ("ber", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("son", 5), ("per", 4), ("tle", 3), ("stone", 3), ("tock", 2), ("lo", 2), ("der", 2),
            ("ver", 2), ("rock", 2), ("ter", 2), ("den", 2), ("shade", 2), ("light", 2),
            ("ger", 2), ("grove", 1), ("gulp", 1), ("hand", 1), ("hands", 1), ("ka", 1),
            ("ker", 1), ("ket", 1), ("klar", 1), ("kle", 1), ("la", 1), ("lane", 1), ("let", 1),
            ("li", 1), ("liamne", 1), ("lice", 1), ("ling", 1), ("lutz", 1), ("mi", 1),
            ("monk", 1), ("nez", 1), ("piece", 1), ("quartz", 1), ("ram", 1), ("rel", 1),
            ("ri", 1), ("rick", 1), ("rim", 1), ("ritt", 1), ("rooj", 1), ("sham", 1),
            ("shine", 1), ("skeel", 1), ("song", 1), ("strap", 1), ("tat", 1), ("ten", 1),
        ]
    }

    fn compound_word_probability() -> f64 {
        0.2483221476510067
    }

    #[rustfmt::skip]
    fn word_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Mirror", 2), ("Bright", 1), ("Burr", 1), ("Coax", 1), ("Cos", 1), ("Cracked", 1),
            ("Digger", 1), ("Dream", 1), ("Drip", 1), ("Fiddle", 1), ("Foam", 1), ("Gleam", 1),
            ("Glitters", 1), ("Gnome", 1), ("Gravel", 1), ("Harrow", 1), ("Iron", 1),
            ("Jasper", 1), ("Lead", 1), ("Lena", 1), ("Min", 1), ("Morning", 1), ("Path", 1),
            ("Pick", 1), ("Randy", 1), ("Scrape", 1), ("Shadow", 1), ("Shive", 1), ("Spider", 1),
            ("Stone", 1), ("Tap", 1), ("Thunder", 1), ("True", 1), ("Weird", 1), ("Auld", 1),
            ("Wood", 1),
        ]
    }

    #[rustfmt::skip]
    fn word_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("rock", 2), ("blood", 1), ("bonk", 1), ("bottle", 1), ("castle", 1), ("dust", 1),
            ("finger", 1), ("gate", 1), ("grove", 1), ("hand", 1), ("hands", 1), ("hunter", 1),
            ("jewel", 1), ("kettle", 1), ("lice", 1), ("light", 1), ("ling", 1), ("mattock", 1),
            ("monk", 1), ("per", 1), ("quartz", 1), ("shade", 1), ("shine", 1), ("shut", 1),
            ("silver", 1), ("singer", 1), ("skillet", 1), ("song", 1), ("stone", 1), ("stop", 1),
            ("strap", 1), ("ter", 1), ("toe", 1), ("tone", 1), ("tor", 1), ("warden", 1),
            ("bender", 1), ("weaver", 1),
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
    use Ethnicity::Gnomish;
    use Gender::{Feminine, Masculine, NonBinaryThey};

    #[test]
    fn gen_name_test() {
        let mut rng = SmallRng::seed_from_u64(0);

        assert_eq!(
            [
                "Gimger Glitkedjerel",
                "Sankrist Weirdgrove",
                "Ginzmilki Mirskilkerick",
                "Jalni Harshade",
                "Jimna Cosgershade",
                "Malian Cosstone",
            ],
            [
                test::gen_name(&mut rng, Gnomish, Adult, Masculine),
                test::gen_name(&mut rng, Gnomish, Adult, Masculine),
                test::gen_name(&mut rng, Gnomish, Adult, Feminine),
                test::gen_name(&mut rng, Gnomish, Adult, Feminine),
                test::gen_name(&mut rng, Gnomish, Adult, NonBinaryThey),
                test::gen_name(&mut rng, Gnomish, Adult, NonBinaryThey),
            ],
        );
    }
}
