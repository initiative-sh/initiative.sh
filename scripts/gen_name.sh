#!/bin/bash
set -euo pipefail

race="$1"

function gen_where() {
  race="$1"

  if [[ $# -ge 2 ]]; then
    gender="$2"
  else
    gender="t"
  fi

  if [[ $# -ge 3 ]]; then
    name_type="$3"
  else
    name_type=""
  fi

  gender_upper="$(echo "$gender" | tr '[:lower:]' '[:upper:]')"

  case "$race" in
    Elf)
      echo -n "p.race IN('Elf', 'Half-elf')"
      ;;
    Orc)
      echo -n "p.race IN('Orc', 'Half-orc')"
      ;;
    Dragonborn)
      echo -n "p.race IN('Dragonborn', 'Dragon')"
      ;;
    *)
      echo -n "p.race = '$race'"
  esac

  if [[ "$gender" != "t" ]]; then
    echo -n " AND p.normalized_gender = '$gender_upper'"
  fi

  case "$name_type" in
    first)
      echo -n " AND n.is_first_name = true"
      ;;
    last)
      echo -n " AND n.is_first_name = false"
      ;;
  esac
}

function run_query() {
  query="$1"

  # Eleven spaces!
  acc="           "
  echo -n "$acc"
  echo "$query" \
    | docker-compose exec -T db psql -Upostgres --csv \
    | sed -E '1d;s/^([^,]+),(.*)$/("\1", \2),/' \
    | while read -r record
  do
    test_acc="$acc $record"
    if [[ ${#test_acc} -gt 100 ]]; then
      echo -ne "\n           "
      acc="            "
    fi

    echo -n " $record"
    acc="$acc $record"
  done

  echo
}

function fn_block_start() {
  fn_name="$1"

  echo
  echo "    #[rustfmt::skip]"
  echo "    fn ${fn_name}() -> &'static [(&'static str, usize)] {"
  echo "        &["
}

function fn_block_end() {
  echo "        ]"
  echo "    }"
}

function syllable_count() {
  race="$1"
  gender="$2"
  name_type="$3"

  fn_name="syllable"
  fn_name="${fn_name}_${name_type:0:1}name"
  fn_name="${fn_name}_count"
  if [[ "$gender" != "t" ]]; then
    fn_name="${fn_name}_${gender}"
  fi

  query="
    SELECT counts.syllable_count, COUNT(*)
    FROM (
      SELECT CASE WHEN COUNT(*) > 2 THEN COUNT(*) ELSE 2 END AS syllable_count
      FROM syllables s, names n, persons p
      WHERE s.name_id = n.name_id
      AND n.person_id = p.person_id
      AND $(gen_where "$race" "$gender" "$name_type")
      GROUP BY s.name_id
    ) AS counts
    GROUP BY counts.syllable_count
    ORDER BY counts.syllable_count ASC
  "

  echo
  echo "    fn ${fn_name}() -> &'static [(u8, usize)] {"
  echo -n "        &["
  echo "$query" \
    | docker-compose exec -T db psql -Upostgres --csv \
    | sed -E '1d;s/^([^,]+),(.*)$/(\1, \2),/' | tr '\n' ' '
  echo "]"
  echo "    }"
}

function syllables() {
  race="$1"
  gender="$2"
  name_type="$3"
  position="$4"

  if [[ "$position" == "first" ]]; then
    is_first="true"
  else
    is_first="false"
  fi

  if [[ "$position" == "last" ]]; then
    is_last="true"
  else
    is_last="false"
  fi

  fn_name="syllable"
  fn_name="${fn_name}_${name_type:0:1}name"
  fn_name="${fn_name}_${position}"
  if [[ "$gender" != "t" ]]; then
    fn_name="${fn_name}_${gender}"
  fi

  fn_block_start "$fn_name"
  run_query "
    SELECT counts.syllable, counts.num
    FROM (
      SELECT syllable, COUNT(*) AS num
      FROM syllables s, names n, persons p
      WHERE s.name_id = n.name_id
      AND n.person_id = p.person_id
      AND $(gen_where "$race" "$gender" "$name_type")
      AND s.is_first = $is_first
      AND s.is_last = $is_last
      GROUP BY syllable
    ) counts
    ORDER BY counts.num DESC
    LIMIT 50
  "
  fn_block_end
}

function compound_word_probability() {
  race="$1"

  query="
    SELECT (
      SELECT COUNT(DISTINCT n.person_id)
      FROM words w, names n, persons p
      WHERE w.name_id = n.name_id
      AND n.person_id = p.person_id
      AND $(gen_where "$race")
    )::float / (
      SELECT COUNT(*)
      FROM persons p
      WHERE $(gen_where "$race")
    )::float;
  "

  echo
  echo "    fn compound_word_probability() -> f64 {"
  echo "$query" \
    | docker-compose exec -T db psql -Upostgres --csv \
    | sed '1d;s/.*/        \0/'
  echo "    }"
}

function compound_words()  {
  race="$1"
  position="$2"

  if [[ "$position" == "first" ]]; then
    is_first="true"
  else
    is_first="false"
  fi

  fn_name="word_lname"
  fn_name="${fn_name}_${position}"

  fn_block_start "$fn_name"
  run_query "
    SELECT w.word, COUNT(*) AS num
    FROM words w, names n, persons p
    WHERE w.name_id = n.name_id
    AND n.person_id = p.person_id
    AND w.is_first = $is_first
    AND $(gen_where "$race")
    GROUP BY w.word
    ORDER BY num DESC
    LIMIT 50
  "
  fn_block_end
}

echo -n "impl GenerateSimple for Ethnicity {"

for gender in f m t; do
  syllable_count "$race" "$gender" first

  for position in first last; do
    syllables "$race" "$gender" first "$position"
  done

  if [[ "$gender" == "t" ]]; then
    syllables "$race" "$gender" first middle
  fi
done

syllable_count "$race" t last
for position in first middle last; do
  syllables "$race" t last "$position"
done

compound_word_probability "$race"
compound_words "$race" first
compound_words "$race" last

cat <<'EOF'
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

EOF
