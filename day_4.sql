select n from generate_series(0, 1000000) as n where md5('ckczppom' || n) like '00000%' limit 1;
select n from generate_series(0, 10000000) as n where md5('ckczppom' || n) like '000000%' limit 1;
-- select md5('abcdef' || 609043);
