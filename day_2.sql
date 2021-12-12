CREATE TABLE IF NOT EXISTS day_2(inp text);
TRUNCATE TABLE day_2;
COPY day_2 from '/input/2.txt';

create temp table day_2_dims as select sort(string_to_array(inp, 'x')::integer[], 'asc') as dims from day_2;

-- part 1
select sum(3*dims[1]*dims[2] + 2*dims[1]*dims[3] + 2*dims[2]*dims[3]) from day_2_dims;

-- part 2
select sum(2*dims[1] + 2*dims[2] + dims[1]*dims[2]*dims[3]) from day_2_dims;
