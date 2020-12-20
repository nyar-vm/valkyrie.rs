(switch)
(switch ((match/when true) 1) (match/else 2))
(switch ((match/case A) a) ((match/type B) b) ((match/when (> c 0)) c) (match/else d))
