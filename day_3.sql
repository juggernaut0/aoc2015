CREATE TABLE IF NOT EXISTS day_3(inp text);
TRUNCATE TABLE day_3;
COPY day_3 from '/input/3.txt';

drop schema if exists day_3 cascade;
create schema day_3;
set search_path to day_3,public;

create domain pos as integer[2];
create function pos_add(x pos, y pos) returns pos as $$
    select array[x[1] + y[1], x[2] + y[2]] as result;
$$ language sql;
create aggregate pos_sum(pos) (
    sfunc = pos_add,
    stype = pos,
    initcond = '{0, 0}'
);

create table d3_diffs as
with d3c as (select string_to_table(inp, NULL) as c from day_3)
select row_number() over (), case
    when c = '^' then array[ 0, -1]
    when c = '>' then array[ 1,  0]
    when c = 'v' then array[ 0,  1]
    when c = '<' then array[-1,  0]
end as d from d3c;


create table d3_pos as
select 0 as row_number, '{0, 0}' as pos_sum union
(select row_number, pos_sum(d) over (order by row_number rows between unbounded preceding and current row) from d3_diffs);

-- part 1
select count(distinct pos_sum) from d3_pos;

create table d3_pos_1 as
select '{0, 0}' as pos_sum union
(select pos_sum(d) over (order by row_number rows between unbounded preceding and current row) from d3_diffs where row_number % 2 = 0);
create table d3_pos_2 as
select '{0, 0}' as pos_sum union
(select pos_sum(d) over (order by row_number rows between unbounded preceding and current row) from d3_diffs where row_number % 2 = 1);

-- part 2
select count(distinct pos_sum) from (select * from d3_pos_1 union (select * from d3_pos_2)) as foo;
