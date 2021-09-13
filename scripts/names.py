import re
import math
import sys

def parse_input(search_str):
    return "".join(
            parse_word(part) if i % 2 == 0 else part
            for i, part in enumerate(re.split(r"(\W+)", search_str)))

VOWEL_SPLIT_EXP = re.compile(r"([aeiouy]+)")
SILENT_E_EXP = re.compile(r"_.e$")

def parse_word(input_word):
    if len(input_word) > 6:
        input_test = input_word.lower()
        for real_word in real_words:
            if input_test.startswith(real_word) and input_test[len(real_word):] in real_words:
                return parse_word(input_word[:len(real_word)]) + "__" + parse_word(input_word[len(real_word):])

    result = ""

    parts = VOWEL_SPLIT_EXP.split(input_word)
    for i, part in enumerate(parts):
        if i % 2 == 0 and i != 0 and i < len(parts) - 1:
            test_part = part.lower()
            sep = math.floor(len(part) / 2)

            if sep > 0 and test_part[sep - 1] in ["p", "t", "c", "d", "s", "g", "b"] and test_part[sep] == "h":
                if sep == 1:
                    sep = 0
                else:
                    sep += 1

            result += part[:sep] + "_" + part[sep:]
        else:
            result += part

    # belatedly handle silent e
    if SILENT_E_EXP.search(result):
        pos = result.rfind("_")
        result = result[:pos] + result[pos + 1:]

    return result

real_words = [word for word in open("words.csv").read().strip().splitlines() if len(word) > 2]
real_words.sort(key=len, reverse=True)

for name in sys.stdin:
    name = name.strip()

    if name[0] != '"' and not name.lower() in real_words:
        output = parse_input(name)
        if "__ing" in output:
            output = output.replace("__ing", "_ing")

        sql_name = name.replace("'", "''")
        sql_output = output.replace("'", "''")

        sys.stdout.write(f"UPDATE names SET syllables = '{sql_output}' WHERE name = '{sql_name}';\n")
