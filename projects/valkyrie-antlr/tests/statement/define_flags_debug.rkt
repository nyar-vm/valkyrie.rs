(enumerate Empty)
(flags Empty)
(enumerate OneEnum (flag A))
(flags OneFlag (flag A))
(enumerate
  ManyEnums
  (flag A 1)
  (flag B (≫ 2 0))
  (flag C (& (call/apply-dot Base (A)) (call/apply-dot Base (B))))
  (flag D (| (call/subscript1 flip (subscript1 Self∷A)) (call/subscript1 flip (subscript1 Self∷B)))))
(flags
  ManyFlags
  (flag A 1)
  (flag B (≫ 2 0))
  (flag C (& (call/apply-dot Base (A)) (call/apply-dot Base (B))))
  (flag D (| (call/subscript1 flip (subscript1 Self∷A)) (call/subscript1 flip (subscript1 Self∷B)))))
