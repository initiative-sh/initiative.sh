pub fn wrap(input: &str, line_len: usize) -> String {
    let mut result = String::with_capacity(input.len());
    let mut cur_line_len = 0;

    input.split_inclusive(char::is_whitespace).for_each(|word| {
        let word_len = word.trim_end().len();

        if word_len + cur_line_len <= line_len {
            result.push_str(word);
            if word.ends_with('\n') {
                cur_line_len = 0;
            } else {
                cur_line_len += word.len();
            }
        } else {
            // Trim trailing whitespace from the previous line.
            while let Some(c) = result.pop() {
                if !c.is_whitespace() {
                    result.push(c);
                    break;
                }
            }

            result.push('\n');

            cur_line_len = if word_len > line_len {
                word.chars().enumerate().for_each(|(i, c)| {
                    result.push(c);
                    if i % line_len == line_len - 1 && !c.is_whitespace() {
                        result.push('\n');
                    }
                });

                word.len() % line_len
            } else {
                result.push_str(word);
                word.len()
            };
        }
    });

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn wrap_short_test() {
        assert_eq!(
            "A word\n\
             wrappe\n\
             d\n\
             senten\n\
             ce\n\
             with\n\
             a\n\
             line\n\
             break.",
            wrap("A word wrapped sentence with\na\nline break.", 6).as_str()
        );
    }

    #[test]
    fn wrap_long_test() {
        let input = "\
CHAPTER 1
Loomings

Call me Ishmael. Some years ago- never mind how long precisely- having little or no money in my purse, and nothing particular to interest me on shore, I thought I would sail about a little and see the watery part of the world. It is a way I have of driving off the spleen and regulating the circulation. Whenever I find myself growing grim about the mouth; whenever it is a damp, drizzly November in my soul; whenever I find myself involuntarily pausing before coffin warehouses, and bringing up the rear of every funeral I meet; and especially whenever my hypos get such an upper hand of me, that it requires a strong moral principle to prevent me from deliberately stepping into the street, and methodically knocking people's hats off- then, I account it high time to get to sea as soon as I can. This is my substitute for pistol and ball. With a philosophical flourish Cato throws himself upon his sword; I quietly take to the ship. There is nothing surprising in this. If they but knew it, almost all men in their degree, some time or other, cherish very nearly the same feelings towards the ocean with me.

There now is your insular city of the Manhattoes, belted round by wharves as Indian isles by coral reefs- commerce surrounds it with her surf. Right and left, the streets take you waterward. Its extreme downtown is the battery, where that noble mole is washed by waves, and cooled by breezes, which a few hours previous were out of sight of land. Look at the crowds of water-gazers there.

Circumambulate the city of a dreamy Sabbath afternoon. Go from Corlears Hook to Coenties Slip, and from thence, by Whitehall, northward. What do you see?- Posted like silent sentinels all around the town, stand thousands upon thousands of mortal men fixed in ocean reveries. Some leaning against the spiles; some seated upon the pier-heads; some looking over the bulwarks of ships from China; some high aloft in the rigging, as if striving to get a still better seaward peep. But these are all landsmen; of week days pent up in lath and plaster- tied to counters, nailed to benches, clinched to desks. How then is this? Are the green fields gone? What do they here?";

        let output_expected = "\
CHAPTER 1
Loomings

Call me Ishmael. Some years ago- never mind how long precisely- having little or
no money in my purse, and nothing particular to interest me on shore, I thought
I would sail about a little and see the watery part of the world. It is a way I
have of driving off the spleen and regulating the circulation. Whenever I find
myself growing grim about the mouth; whenever it is a damp, drizzly November in
my soul; whenever I find myself involuntarily pausing before coffin warehouses,
and bringing up the rear of every funeral I meet; and especially whenever my
hypos get such an upper hand of me, that it requires a strong moral principle to
prevent me from deliberately stepping into the street, and methodically knocking
people's hats off- then, I account it high time to get to sea as soon as I can.
This is my substitute for pistol and ball. With a philosophical flourish Cato
throws himself upon his sword; I quietly take to the ship. There is nothing
surprising in this. If they but knew it, almost all men in their degree, some
time or other, cherish very nearly the same feelings towards the ocean with me.

There now is your insular city of the Manhattoes, belted round by wharves as
Indian isles by coral reefs- commerce surrounds it with her surf. Right and
left, the streets take you waterward. Its extreme downtown is the battery, where
that noble mole is washed by waves, and cooled by breezes, which a few hours
previous were out of sight of land. Look at the crowds of water-gazers there.

Circumambulate the city of a dreamy Sabbath afternoon. Go from Corlears Hook to
Coenties Slip, and from thence, by Whitehall, northward. What do you see?-
Posted like silent sentinels all around the town, stand thousands upon thousands
of mortal men fixed in ocean reveries. Some leaning against the spiles; some
seated upon the pier-heads; some looking over the bulwarks of ships from China;
some high aloft in the rigging, as if striving to get a still better seaward
peep. But these are all landsmen; of week days pent up in lath and plaster- tied
to counters, nailed to benches, clinched to desks. How then is this? Are the
green fields gone? What do they here?";

        let output_actual = wrap(input, 80);

        assert_eq!(output_expected, output_actual.as_str());
        assert!(output_actual.lines().all(|line| line.len() <= 80));
        assert!(output_actual.lines().any(|line| line.len() == 80));
    }
}
