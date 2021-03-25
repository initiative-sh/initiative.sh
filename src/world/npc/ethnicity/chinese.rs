use rand::distributions::WeightedIndex;
use rand::prelude::*;

use super::{Age, Gender, Generate, Rng};

pub struct Ethnicity;

impl Ethnicity {
    #[rustfmt::skip]
    const FEMININE_NAMES: &'static [&'static str] = &[
        "Ai", "Anming", "Baozhai", "Bei", "Caixia", "Changchang", "Chen", "Chou", "Chunhua",
        "Daianna", "Daiyu", "Die", "Ehuang", "Fenfang", "Ge", "Hong", "Huan", "Huifang", "Jia",
        "Jiao", "Jiaying", "Jingfei", "Jinjing", "Lan", "Li", "Lihua", "Lin", "Ling", "Liu",
        "Meili", "Ning", "Qi", "Qiao", "Rong", "Shu", "Shuang", "Song", "Ting", "Wen", "Xia",
        "Xiaodan", "Xiaoli", "Xingjuan", "Xue", "Ya", "Yan", "Ying", "Yuan", "Yue", "Yun",
    ];

    #[rustfmt::skip]
    const MASCULINE_NAMES: &'static [&'static str] = &[
        "Bingwen", "Bo", "Bolin", "Chang", "Chao", "Chen", "Cheng", "Da", "Dingxia", "ng", "Fang",
        "Feng", "Fu", "Gang", "Guang", "Hai", "He", "ng", "Ho", "ng", "Huan", "Huang", "Huiliang",
        "Huizhong", "Jian", "Jiayi", "Junjie", "Kang", "Lei", "Liang", "Ling", "Liwei", "Meilin",
        "Niu", "Peizhi", "Peng", "Ping", "Qiang", "Qiu", "Quan", "Renshu", "Rong", "Ru", "Shan",
        "Shen", "Tengfei", "Wei", "Xiaobo", "Xiaoli", "Xin", "Yang", "Ying", "Zhong",
    ];
}

impl Generate for Ethnicity {
    fn gen_name(rng: &mut impl Rng, age: &Age, gender: &Gender) -> String {
        match gender {
            Gender::Masculine => {
                Self::MASCULINE_NAMES[rng.gen_range(0..Self::MASCULINE_NAMES.len())].to_string()
            }
            Gender::Feminine => {
                Self::FEMININE_NAMES[rng.gen_range(0..Self::FEMININE_NAMES.len())].to_string()
            }
            _ => {
                let dist =
                    WeightedIndex::new(&[Self::MASCULINE_NAMES.len(), Self::FEMININE_NAMES.len()])
                        .unwrap();
                if dist.sample(rng) == 0 {
                    Self::gen_name(rng, age, &Gender::Masculine)
                } else {
                    Self::gen_name(rng, age, &Gender::Feminine)
                }
            }
        }
    }
}

#[cfg(test)]
mod test_generate_for_ethnicity {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn gen_name_test() {
        let mut rng = StepRng::new(0, 0xDEADBEEF_DECAFBAD);
        let age = Age::Adult(0);
        let m = Gender::Masculine;
        let f = Gender::Feminine;
        let t = Gender::Trans;

        assert_eq!(
            ["Bingwen", "Wei", "Ning", "Huifang", "Chao", "Xiaoli"],
            [
                Ethnicity::gen_name(&mut rng, &age, &m),
                Ethnicity::gen_name(&mut rng, &age, &m),
                Ethnicity::gen_name(&mut rng, &age, &f),
                Ethnicity::gen_name(&mut rng, &age, &f),
                Ethnicity::gen_name(&mut rng, &age, &t),
                Ethnicity::gen_name(&mut rng, &age, &t),
            ]
        );
    }
}
