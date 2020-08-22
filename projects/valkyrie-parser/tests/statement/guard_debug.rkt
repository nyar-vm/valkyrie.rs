(guard true else (body ()))
(return 0)
(guard false else (body ((return (- 1)))))
