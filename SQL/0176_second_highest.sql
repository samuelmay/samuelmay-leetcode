select b.salary as SecondHighestSalary from
  (select 2 as rank from dual) a left join
  (select salary, ROW_NUMBER() over (order by salary desc) as rank from Employee group by salary) b
  on a.rank = b.rank;