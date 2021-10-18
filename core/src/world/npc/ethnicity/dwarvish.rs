use super::{Age, Gender, Generate, GenerateSimple};
use rand::prelude::*;

pub struct Ethnicity;

impl GenerateSimple for Ethnicity {
    fn syllable_fname_count_f() -> &'static [(u8, usize)] {
        &[(2, 46), (3, 17), (4, 1)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_f() -> &'static [(&'static str, usize)] {
        &[
            ("Kal", 2), ("Da", 2), ("Ba", 1), ("Bar", 1), ("Bel", 1), ("Dag", 1), ("Dor", 1),
            ("Era", 1), ("Eu", 1), ("Fav", 1), ("Glen", 1), ("Gren", 1), ("Gris", 1), ("Ha", 1),
            ("Hai", 1), ("Hel", 1), ("Hil", 1), ("Ja", 1), ("Ker", 1), ("Krys", 1), ("Le", 1),
            ("Lher", 1), ("Mag", 1), ("Mak", 1), ("Mi", 1), ("Min", 1), ("Mo", 1), ("Mor", 1),
            ("Net", 1), ("Nu", 1), ("Phos", 1), ("Ruu", 1), ("Sar", 1), ("Sin", 1), ("Su", 1),
            ("Tam", 1), ("Thea", 1), ("Then", 1), ("Ti", 1), ("Tor", 1), ("Ty", 1), ("Ulko", 1),
            ("Vei", 1), ("Vel", 1), ("Yan", 1), ("You", 1), ("Anag", 1), ("Ze", 1), ("Aug", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_f() -> &'static [(&'static str, usize)] {
        &[
            ("ra", 8), ("na", 5), ("da", 5), ("la", 4), ("ria", 2), ("ta", 2), ("ga", 1),
            ("jit", 1), ("lah", 1), ("leen", 1), ("lia", 1), ("lynn", 1), ("mans", 1), ("mas", 1),
            ("nee", 1), ("nel", 1), ("rek", 1), ("rys", 1), ("sey", 1), ("ther", 1), ("thmel", 1),
            ("ti", 1), ("tie", 1), ("tryd", 1), ("ba", 1), ("win", 1), ("bet", 1), ("bia", 1),
            ("der", 1), ("di", 1), ("dreth", 1),
        ]
    }

    fn syllable_fname_count_m() -> &'static [(u8, usize)] {
        &[(2, 274), (3, 36), (4, 3)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first_m() -> &'static [(&'static str, usize)] {
        &[
            ("Dor", 6), ("Ta", 5), ("Ha", 4), ("Thar", 4), ("Dur", 4), ("Kar", 3), ("Bo", 3),
            ("Tor", 3), ("Da", 3), ("Ba", 3), ("Dar", 3), ("Har", 3), ("Ga", 3), ("Gar", 3),
            ("Brue", 2), ("Mar", 2), ("Kur", 2), ("Ra", 2), ("Yon", 2), ("Tu", 2), ("Sa", 2),
            ("Ni", 2), ("Stone", 2), ("Mo", 2), ("Be", 2), ("Ma", 2), ("Dun", 2), ("Na", 2),
            ("Su", 2), ("Ka", 2), ("Hor", 2), ("Bol", 2), ("Gor", 2), ("Dag", 2), ("Grim", 2),
            ("Ko", 2), ("Bor", 2), ("Jer", 2), ("Cy", 1), ("Crom", 1), ("Crag", 1), ("Beh", 1),
            ("Ambe", 1), ("Cor", 1), ("Clan", 1), ("Fei", 1), ("Cin", 1), ("Esco", 1), ("Ele", 1),
            ("Car", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last_m() -> &'static [(&'static str, usize)] {
        &[
            ("gan", 4), ("gar", 4), ("den", 4), ("rin", 4), ("ryn", 3), ("lig", 3), ("nor", 3),
            ("kin", 2), ("nos", 2), ("la", 2), ("dar", 2), ("der", 2), ("dor", 2), ("rim", 2),
            ("zon", 2), ("ram", 2), ("rak", 2), ("ren", 2), ("lo", 2), ("wer", 2), ("go", 2),
            ("thar", 2), ("kum", 2), ("ly", 2), ("ri", 2), ("co", 1), ("dio", 1), ("din", 1),
            ("don", 1), ("cius", 1), ("bleth", 1), ("dik", 1), ("cil", 1), ("gle", 1), ("dek", 1),
            ("char", 1), ("bit", 1), ("bas", 1), ("darm", 1), ("ger", 1), ("darl", 1), ("car", 1),
            ("gen", 1), ("ghen", 1), ("ghor", 1), ("gin", 1), ("gis", 1), ("glak", 1), ("gel", 1),
            ("gaur", 1),
        ]
    }

    fn syllable_fname_count() -> &'static [(u8, usize)] {
        &[(2, 335), (3, 56), (4, 5)]
    }

    #[rustfmt::skip]
    fn syllable_fname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Dor", 7), ("Ba", 5), ("Ta", 5), ("Da", 5), ("Ha", 5), ("Thar", 4), ("Tor", 4),
            ("Dur", 4), ("Kar", 4), ("Stone", 3), ("Dag", 3), ("Dar", 3), ("Ga", 3), ("Mo", 3),
            ("Su", 3), ("Gar", 3), ("Har", 3), ("Bo", 3), ("Dun", 2), ("Ti", 2), ("Stro", 2),
            ("Brue", 2), ("Ree", 2), ("Ra", 2), ("Sa", 2), ("Bor", 2), ("Mar", 2), ("Na", 2),
            ("Tu", 2), ("Yon", 2), ("Kol", 2), ("Ka", 2), ("Kur", 2), ("Ni", 2), ("Hor", 2),
            ("Ja", 2), ("Gor", 2), ("Be", 2), ("Grim", 2), ("Kal", 2), ("Bol", 2), ("Ko", 2),
            ("Ma", 2), ("Jer", 2), ("Pry", 2), ("Mi", 2), ("Cor", 1), ("Clan", 1), ("Bar", 1),
            ("Cin", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_last() -> &'static [(&'static str, usize)] {
        &[
            ("ra", 8), ("na", 6), ("la", 6), ("da", 5), ("gan", 4), ("den", 4), ("gar", 4),
            ("rin", 4), ("ryn", 3), ("der", 3), ("lig", 3), ("nor", 3), ("ri", 3), ("nos", 2),
            ("ren", 2), ("ma", 2), ("dar", 2), ("zon", 2), ("rik", 2), ("ta", 2), ("rim", 2),
            ("thar", 2), ("ram", 2), ("rak", 2), ("ria", 2), ("vin", 2), ("dor", 2), ("ly", 2),
            ("nin", 2), ("kos", 2), ("go", 2), ("kin", 2), ("lah", 2), ("kum", 2), ("wer", 2),
            ("lo", 2), ("dek", 1), ("car", 1), ("darm", 1), ("darl", 1), ("bul", 1), ("bert", 1),
            ("dak", 1), ("bromm", 1), ("dai", 1), ("dagg", 1), ("bold", 1), ("beck", 1),
            ("garl", 1), ("bald", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_fname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("da", 4), ("de", 3), ("no", 3), ("ro", 2), ("bul", 2), ("mo", 2), ("ra", 2), ("re", 2),
            ("bel", 2), ("ri", 2), ("nab", 2), ("ham", 1), ("hed", 1), ("ke", 1), ("kil", 1),
            ("knug", 1), ("ko", 1), ("li", 1), ("mi", 1), ("nar", 1), ("nel", 1), ("ner", 1),
            ("pa", 1), ("ril", 1), ("rio", 1), ("ris", 1), ("rum", 1), ("run", 1), ("sen", 1),
            ("sha", 1), ("shar", 1), ("ta", 1), ("tha", 1), ("ti", 1), ("tok", 1), ("var", 1),
            ("wa", 1), ("wil", 1), ("zar", 1), ("zo", 1), ("zol", 1), ("bol", 1), ("bot", 1),
            ("den", 1), ("der", 1), ("du", 1), ("fa", 1), ("gal", 1), ("gi", 1), ("Ha", 1),
        ]
    }

    fn syllable_lname_count() -> &'static [(u8, usize)] {
        &[(2, 166), (3, 108), (4, 15)]
    }

    #[rustfmt::skip]
    fn syllable_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Iron", 14), ("Stone", 13), ("al-", 0), ("Ham", 7), ("Bat", 6), ("Steel", 6),
            ("Sil", 6), ("Bright", 4), ("Hill", 4), ("Storm", 3), ("Boul", 3), ("Thun", 3),
            ("Strong", 3), ("Forge", 3), ("Grey", 3), ("Round", 3), ("Brawn", 2), ("Xun", 2),
            ("Ur'", 2), ("Flame", 2), ("Fla", 2), ("Way", 2), ("Smoke", 2), ("Rock", 2),
            ("Split", 2), ("Troll", 2), ("Lud", 2), ("Bloo", 2), ("Ho", 2), ("Me", 2), ("Har", 2),
            ("Ha", 2), ("Hard", 2), ("Gold", 2), ("Brew", 2), ("Grim", 2), ("Black", 2),
            ("Gra", 2), ("Gar", 2), ("High", 2), ("Iro", 2), ("Sto", 2), ("Mi", 2), ("Ri", 2),
            ("Coo", 1), ("Coal", 1), ("Clan", 1), ("Chis", 1), ("Bra", 1), ("Ches", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_middle() -> &'static [(&'static str, usize)] {
        &[
            ("ham", 16), ("mer", 8), ("tle", 6), ("ver", 6), ("sha", 3), ("Ja", 3), ("Ha", 3),
            ("shoul", 3), ("ter", 3), ("car", 3), ("an", 3), ("der", 2), ("ka", 2), ("crus", 2),
            ("la", 2), ("per", 2), ("ea", 2), ("ton", 2), ("ra", 2), ("dy", 2), ("wa", 2),
            ("see", 2), ("kin", 1), ("lair", 1), ("len", 1), ("low", 1), ("mar", 1), ("mes", 1),
            ("mo", 1), ("ne", 1), ("nes", 1), ("nest", 1), ("nite", 1), ("no", 1), ("nug", 1),
            ("ped", 1), ("plit", 1), ("rel", 1), ("ron", 1), ("shat", 1), ("sin", 1), ("sing", 1),
            ("smel", 1), ("sprin", 1), ("stea", 1), ("sun", 1), ("thral", 1), ("thro", 1),
            ("Thu", 1), ("tles", 1),
        ]
    }

    #[rustfmt::skip]
    fn syllable_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("mer", 16), ("beard", 14), ("shield", 12), ("fist", 11), ("der", 7), ("ver", 5),
            ("helm", 5), ("hand", 5), ("ter", 5), ("ri", 4), ("ker", 4), ("bar", 4), ("stone", 3),
            ("skull", 3), ("vil", 3), ("fell", 2), ("dark", 2), ("gray", 2), ("wer", 2),
            ("dow", 2), ("stream", 2), ("tar", 2), ("steel", 2), ("zar", 2), ("blade", 2),
            ("dorn", 2), ("sonn", 2), ("naxe", 2), ("smith", 2), ("ler", 2), ("her", 2),
            ("lin", 2), ("den", 2), ("forge", 2), ("gue", 2), ("gar", 2), ("ma", 2), ("ger", 2),
            ("far", 1), ("daxe", 1), ("fall", 1), ("duum", 1), ("bold", 1), ("dson", 1),
            ("drun", 1), ("chasm", 1), ("grym", 1), ("grin", 1), ("grim", 1), ("cap", 1),
        ]
    }

    fn compound_word_probability() -> f64 {
        0.323943661971831
    }

    #[rustfmt::skip]
    fn word_lname_first() -> &'static [(&'static str, usize)] {
        &[
            ("Stone", 13), ("Hammer", 7), ("Iron", 7), ("Steel", 5), ("Grey", 3), ("Silver", 3),
            ("Strong", 3), ("Hill", 3), ("Forge", 3), ("Battle", 3), ("Hard", 2), ("Bright", 2),
            ("Boulder", 2), ("Grim", 2), ("Gold", 2), ("Troll", 2), ("Rock", 2), ("Split", 2),
            ("Flame", 2), ("Smoke", 2), ("Storm", 2), ("Flagon", 1), ("Flames", 1), ("Foe", 1),
            ("Fork", 1), ("Fray", 1), ("Frost", 1), ("Gar", 1), ("Gems", 1), ("Giant", 1),
            ("Glitter", 1), ("Gloom", 1), ("Granite", 1), ("Gravel", 1), ("Gray", 1),
            ("Grizzle", 1), ("High", 1), ("Hills", 1), ("Honest", 1), ("Ink", 1), ("Irons", 1),
            ("Mane", 1), ("Moon", 1), ("Muffin", 1), ("One", 1), ("Pebble", 1), ("Pure", 1),
            ("Red", 1), ("Rising", 1), ("Round", 1),
        ]
    }

    #[rustfmt::skip]
    fn word_lname_last() -> &'static [(&'static str, usize)] {
        &[
            ("beard", 13), ("hammer", 10), ("shield", 9), ("fist", 5), ("hand", 5), ("helm", 3),
            ("anvil", 2), ("seeker", 2), ("tar", 2), ("crus", 2), ("tongue", 2), ("shoulder", 2),
            ("fell", 2), ("skull", 2), ("dark", 2), ("smith", 2), ("her", 2), ("steel", 2),
            ("blade", 2), ("stone", 2), ("carver", 2), ("gold", 1), ("grin", 1), ("guard", 1),
            ("hall", 1), ("head", 1), ("heart", 1), ("hewer", 1), ("ira", 1), ("ire", 1),
            ("jaw", 1), ("killer", 1), ("kin", 1), ("layer", 1), ("less", 1), ("marrow", 1),
            ("mind", 1), ("mover", 1), ("night", 1), ("nugget", 1), ("one", 1), ("peddler", 1),
            ("rut", 1), ("scar", 1), ("shadow", 1), ("shatter", 1), ("shorn", 1), ("singer", 1),
            ("smelter", 1), ("springer", 1),
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
                "Gargel Xunterler",
                "Escokum Moonshield",
                "Tileen Ironhand",
                "Torketryd Muffinshield",
                "Nalig Flamehewer",
                "Turyn Stonesmelshield",
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
        npc.ethnicity.replace(Ethnicity::Dwarvish);
        regenerate(rng, &mut npc);
        format!("{}", npc.name)
    }
}
