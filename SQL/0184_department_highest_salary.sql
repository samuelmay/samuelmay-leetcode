select d.name, e.Employee, e.Salary
 from Department d inner join
 (select name as Employee,
	salary as Salary,
	departmentId,
	rank() over (partition by departmentId order by salary desc) as SalaryRank
    from Employee) e
 on d.id = e.departmentId where SalaryRank = 1;
