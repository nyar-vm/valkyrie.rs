(define/class A (modifiers))
(define/class B (modifiers value))
(define/class Positive (modifiers refine))
(define/class A (modifiers) (class/field Base (modifiers inherit)) (class/field Base2 (modifiers inherit)))
(define/class B (modifiers) (class/field item (modifiers)))
(define/class
  A
  (modifiers)
  (class/field _a (modifiers))
  (class/method a (modifiers pub static) () (return/type Unit) (return/type Pure))
  (class/method a (modifiers get) () (return/type Unit) (return/type Pure))
  (class/method a (modifiers set) () (return/type Unit) (return/type Pure))
  (class/method into (modifiers) () (return/type Unit) (return/type Pure)))
(define/class A (modifiers))
(define/class Empty1 (modifiers))
(define/class Empty2 (modifiers))
(define/class Empty3 (modifiers public static dynamic_transaction_safe volatile final))
(define/class Inherit1 (modifiers open impart))
(define/class
  Inherit2
  (modifiers open impart)
  (class/field x_base (modifiers public virtual inherit))
  (class/field _y_base (modifiers private inline inherit))
  (class/method constructor (modifiers) () (return/type Unit) (return/type Pure)))
(define/class
  原神
  (modifiers)
  (class/method 启动 (modifiers) () (return/type Unit) (return/type Pure))
  (class/method value (modifiers get) () (return/type Unit) (return/type Pure))
  (class/method value (modifiers set) () (return/type Unit) (return/type Pure)))
(define/class
  ClassA
  (modifiers)
  (class/field type (modifiers public static))
  (class/field unknown_all (modifiers))
  (class/field default_value (modifiers))
  (class/field infer_type (modifiers))
  (class/method property (modifiers get) () (return/type Unit) (return/type Pure))
  (class/method property (modifiers set) () (return/type Unit) (return/type Pure))
  (class/method method (modifiers) () (return/type Unit) (return/type Pure))
  (class/method / (modifiers infix) () (return/type Unit) (return/type Pure))
  (class/method join (modifiers) () (return/type Unit) (return/type Pure)))
(define/class TestClass (modifiers))
(define/class
  Class
  (modifiers)
  (class/field field1 (modifiers readonly))
  (class/field field 2 (modifiers readonly))
  (class/field main (modifiers class static void))
  (class/field fields (modifiers mut))
  (class/method try_add (modifiers) () (return/type Unit) (return/type Pure)))
extends
