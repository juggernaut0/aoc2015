CREATE TABLE IF NOT EXISTS day_5(inp text);
TRUNCATE TABLE day_5;
COPY day_5 from '/input/5.txt';

create or replace function contains_duplicate_char(string text) returns boolean as $$
with foo as (select row_number() over (), c from string_to_table(string, NULL) as c),
     winds as (select string_agg(c, '') over (order by row_number rows between 1 preceding and current row) as wind from foo),
     dups as (select * from winds where char_length(wind) = 2 and substring(wind from 1 for 1) = substring(wind from 2 for 1))
select count(*) > 0 as result from dups;
$$ language sql;

create or replace function contains_3_vowels(string text) returns boolean as $$
select count(*) >= 3 as result from string_to_table(string, NULL) as c where c = 'a' or c = 'e' or c = 'i' or c = 'o' or c = 'u';
$$ language sql;

create or replace function contains_bad_substring(string text) returns boolean as $$
select position('ab' in string) > 0 or position('cd' in string) > 0 or position('pq' in string) > 0 or position('xy' in string) > 0 as result;
$$ language sql;

select count(inp) as part_1 from day_5 where contains_3_vowels(inp) and contains_duplicate_char(inp) and not contains_bad_substring(inp);

create or replace function contains_nonoverlapping_pair(string text) returns boolean as $$
with with_index as (select row_number() over (), c from string_to_table(string, NULL) as c),
     winds as (select row_number, string_agg(c, '') over (order by row_number rows between 1 preceding and current row) as wind from with_index),
     pairs as (select wind, min(row_number), max(row_number) from winds where char_length(wind) = 2 group by wind)
select count(*) > 0 as result from pairs where "max" - "min" > 1;
$$ language sql;

create or replace function contains_sandwich(string text) returns boolean as $$
with with_index as (select row_number() over (), c from string_to_table(string, NULL) as c),
     winds as (select row_number, string_agg(c, '') over (order by row_number rows between 2 preceding and current row) as wind from with_index),
     sands as (select wind from winds where char_length(wind) = 3 and substring(wind from 1 for 1) = substring(wind from 3 for 1))
select count(*) > 0 as result from sands;
$$ language sql;

select count(inp) as part_2 from day_5 where contains_nonoverlapping_pair(inp) and contains_sandwich(inp);
