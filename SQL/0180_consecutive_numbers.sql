# joining the table to itself using primary id key
select distinct l.num as ConsecutiveNums from Logs l
  inner join Logs lshift1 on l.id = lshift1.id+1
  inner join Logs lshift2 on l.id = lshift2.id+2
  where l.num = lshift1.num and l.num = lshift2.num;
  
# fancy version using lead and lag window functions.
select distinct num as ConsecutiveNums from
  (select num, lead(num) over (order by id) as nextnum, lag(num) over (order by id) as prevnum from Logs) tmp
  where num = nextnum and num = prevnum;