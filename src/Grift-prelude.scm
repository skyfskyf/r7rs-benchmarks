;;; Grift prelude for r7rs benchmarks

;; Make import a no-op since grift has all R7RS builtins available
(define-syntax import
  (syntax-rules ()
    ((import . rest) (begin))))

