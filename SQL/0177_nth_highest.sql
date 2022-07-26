delimiter //
create function getNthHighestSalary(N int) returns int
begin
  declare salary_offset int;
  set salary_offset = N - 1;
  return (select
    (select salary
     from Employee
     group by salary
     order by salary desc
     limit 1 offset salary_offset) from dual);
end//
delimiter ;