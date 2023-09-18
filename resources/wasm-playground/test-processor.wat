;; WebAssembly WAT audio processor

(module
    (global $SR (mut f64) (f64.const 48000))

    (func $set_SR (param $new_SR f64)
        ;; set the global variable "SR" to the value of the parameter
        (local.get $new_SR)
        (global.set $SR)
    )

    (func $process (result f64)
        (f64.const 0.0)
    )

    (export "process" (func $process))
    (export "set_SR" (func $set_SR))
)