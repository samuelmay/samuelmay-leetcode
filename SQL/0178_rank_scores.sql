select s.score, r.position as 'rank'
  from Scores s inner join
  (select row_number() over (order by score desc) as position, score
    from Scores group by score) r
  on s.score = r.score
  order by r.position;