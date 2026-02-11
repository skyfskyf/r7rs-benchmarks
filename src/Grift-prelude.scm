;;; Grift prelude for r7rs benchmarks

;; Make import a no-op since grift has all R7RS builtins available
(define-syntax import
  (syntax-rules ()
    ((import . rest) (begin))))

;; Implementation name (update version when upgrading grift dependency)
(define (this-scheme-implementation-name) "grift-1.4.0")

;; Make values a first-class procedure (grift treats values as a special form)
(define values-proc (lambda args
  (if (= (length args) 1)
      (car args)
      (apply values args))))
