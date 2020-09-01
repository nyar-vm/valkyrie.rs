(flags Empty)
(flags OneFlag (flag A))
(flags
  TwoFlags
  (flag A 1)
  (flag B (≫ 2 0))
  (flag C (& (call/apply-dot Base (A)) (call/apply-dot Base (B))))
  (flag C (| (call/subscript1 flip (subscript1 Self∷A)) (call/subscript1 flip (subscript1 Self∷B)))))
