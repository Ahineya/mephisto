(module
    (memory 1) ;; declare memory with 1 page
    ;; 1 page is 64kb
    ;; Set the memory to be shared between wasm and js
    (export "memory" (memory 0))
    ;; Add "frequency" to the memory
    (data (i32.const 0) "frequency")
    ;; declare "phase" as a global variable
    (global $phase (mut f64) (f64.const 0))
    ;; declare "SR" as a global variable
    (global $SR (mut f64) (f64.const 48000))

    (func $set_SR (param $new_SR f64)
        ;; set the global variable "SR" to the value of the parameter
        local.get $new_SR
        global.set $SR
    )

    (func $set_frequency (param $new_frequency f64)
        ;; set the frequency in memory to the value of the parameter
        i32.const 0
        local.get $new_frequency
        f64.store
    )

  (func $process (result f64) (local $increment f64)
    ;; get the frequency from memory
    i32.const 0
    f64.load
    ;; get the SR from memory
    global.get $SR
    ;; divide the frequency by the SR
    f64.div
    ;; store the result in the local variable "increment"
    local.set $increment
    ;; get the phase from the global variable "phase"
    global.get $phase
    global.get $phase
    local.get $increment
    f64.add
    ;; floor the result
    f64.floor
    ;; subtract the result from the phase
    f64.sub
    ;; add the result to the increment
    local.get $increment
    f64.add
    ;; store the result in the global variable "phase"
    global.set $phase
    ;; return the result
    local.get $increment
  )

    (export "process" (func $process))
    (export "set_SR" (func $set_SR))
    (export "set_frequency" (func $set_frequency))
)