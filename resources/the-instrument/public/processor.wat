;; WebAssembly WAT audio processor

(module
    (global $SR (mut f64) (f64.const 48000))

    (func $set_SR (param $new_SR f64)
        ;; set the global variable "SR" to the value of the parameter
        (global.set $SR (local.get $new_SR))
    )

    (global $__inputs_0 (mut f64) (f64.const 110))
(global $__outputs_0 (mut f64) (f64.const 0))
(global $increment (mut f64) (f64.const 0))


    (func $process (result f64)
        {
(global.set (global.get $increment)(f64.div(global.get $__inputs_0)sampleRate))}


        (global.set $__outputs_0(f64.add(global.get $increment)(f64.sub(global.get $__outputs_0)(f64.floor(f64.add(global.get $increment)(global.get $__outputs_0))))))
    )

    (export "process" (func $process))
    (export "set_SR" (func $set_SR))
)