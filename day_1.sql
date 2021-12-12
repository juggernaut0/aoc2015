CREATE TABLE IF NOT EXISTS day_1(inp text);
TRUNCATE TABLE day_1;
COPY day_1 from '/input/1.txt';

create table day_1_cumsum as
with d1c as (SELECT string_to_table(inp, NULL) as c from day_1),
     d1n as (SELECT row_number() over (), CASE WHEN c = '(' then 1 else -1 end as n from d1c)
select row_number, sum(n) over (order by row_number rows between unbounded preceding and current row) from d1n order by row_number;

-- part 1
select sum from day_1_cumsum where row_number = 7000;

-- part 2
select min(row_number) from day_1_cumsum where sum < 0;
