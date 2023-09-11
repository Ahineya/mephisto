/*
stdlib.insert("buf_new".to_string(), "new Ringbuffer".to_string());
stdlib.insert("buf_read".to_string(), "Ringbuffer.read".to_string());
stdlib.insert("buf_push".to_string(), "Ringbuffer.push".to_string());
stdlib.insert("buf_pop".to_string(), "Ringbuffer.pop".to_string());
stdlib.insert("buf_length".to_string(), "Ringbuffer.length".to_string());
stdlib.insert("buf_clear".to_string(), "Ringbuffer.clear".to_string());
stdlib.insert("buf_put".to_string(), "Ringbuffer.put".to_string());
*/

const Rb = {
read: function (rb, index) {
return rb.get(index);
},

push: function (rb, element) {
rb.push(element);
},

pop: function (rb) {
return rb.pop();
},

length: function (rb) {
return rb.length;
},

clear: function (rb) {
rb.clear();
},

put: function (rb, index, value) {
rb.set(index, value);
},

setAll: function (rb, fn) {
rb.setAll(fn);
},

resize: function (rb, size) {
rb.resize(size);
}
}


class Ringbuffer {
constructor(size) {
this.elements = new Float64Array(size);
this.readIndex = 0;
this.writeIndex = 0;
}

push(element) {

// This is super slow. It should be ran in the realtime audio thread.
// for (let i = 0; i < this.elements.length - 1; i++) {
//     this.elements[i] = this.elements[i + 1];
// }
//
// this.elements[this.elements.length - 1] = element;

// This is faster, but it's not a ringbuffer.
// this.elements.push(element);

// This is the fastest
this.elements[this.writeIndex] = element;
this.writeIndex++;

if (this.writeIndex >= this.elements.length) {
this.writeIndex = 0;
}

if (this.writeIndex === this.readIndex) {
this.readIndex++;

if (this.readIndex >= this.elements.length) {
this.readIndex = 0;
}
}
}

pop() {
// This is super slow. It should be ran in the realtime audio thread.
// const element = this.elements[0];
//
// for (let i = 0; i < this.elements.length - 1; i++) {
//     this.elements[i] = this.elements[i + 1];
// }
//
// this.elements[this.elements.length - 1] = 0;
//
// return element;

// And here goes the proper implementation
const element = this.elements[this.readIndex];
this.readIndex++;

if (this.readIndex >= this.elements.length) {
this.readIndex = 0;
}

return element;
}

peek() {
// Peek into the current element
return this.elements[this.readIndex];
}

get(index) {
// Get the element at the given index, starting from the read index
return this.elements[(this.readIndex + index) % this.elements.length];
}

set(index, value) {
// Set the element at the given index, starting from the read index
this.elements[(this.readIndex + index) % this.elements.length] = value;
}

setAll(fn) {
for (let i = 0; i < this.elements.length; i++) {
Rb.push(this, fn(i));
}
}

resize(size) {
this.elements = new Float64Array(size);
this.readIndex = 0;
this.writeIndex = 0;
}

clear() {
for (let i = 0; i < this.elements.length; i++) {
this.elements[i] = 0;
}
}

get length() {
return this.elements.length;
}
}

const Std = {
if: function (condition, then) {
if (condition) {
return then();
} else {
return 0;
}
},

ifElse: function (condition, then, otherwise) {
if (condition) {
return then();
} else {
return otherwise();
}
}
}

const __m_inputs = new Float64Array(41);
const __m_outputs = new Float64Array(32);

const __inputNames = ["Osc#Phaser#frequency", "Osc#Smooth#inp", "Osc#frequency", "Osc#wave", "Osc#phase", "Osc2#Phaser#frequency", "Osc2#Smooth#inp", "Osc2#frequency", "Osc2#wave", "Osc2#phase", "Osc3#Phaser#frequency", "Osc3#Smooth#inp", "Osc3#frequency", "Osc3#wave", "Osc3#phase", "Noise#gain", "LFO#Phaser#frequency", "LFO#Smooth#inp", "LFO#frequency", "LFO#wave", "LFO#phase", "ADSR#gate", "LowPass#cutoffMod", "LowPass#audioIn", "Echo#audioIn", "Limiter#audioIn", "Freeverb#audioIn", "Mix#mix1", "Mix#mix2", "Attenuator#inp", "osc1", "osc2", "osc3", "noise", "echo", "adsr", "karplus", "frequencyMod", "osc1gainMod", "osc2gainMod", "noiseGainMod"];
const __outputNames = ["Osc#Phaser#phase", "Osc#Smooth#out", "Osc#out", "Osc#internal_freq", "Osc2#Phaser#phase", "Osc2#Smooth#out", "Osc2#out", "Osc2#internal_freq", "Osc3#Phaser#phase", "Osc3#Smooth#out", "Osc3#out", "Osc3#internal_freq", "Noise#out", "LFO#Phaser#phase", "LFO#Smooth#out", "LFO#out", "LFO#internal_freq", "OscVolume#osc1gainOut", "OscVolume#osc2gainOut", "OscVolume#osc3gainOut", "OscVolume#noiseGainOut", "ADSR#curve", "LowPass#audioOut", "Echo#audioOut", "Limiter#audioOut", "Freeverb#audioOut", "Mix#out", "Attenuator#out", "out", "osc1freq", "osc2freq", "osc3freq"];

let connections = [
    [16, 17],
[14, 16],
[13, 20],
[11, 11],
[9, 10],
[8, 14],
[7, 6],
[5, 5],
[4, 9],
[3, 1],
[1, 0],
[0, 4],
[2, 30],
[6, 31],
[10, 32],
[12, 33],
[29, 2],
[30, 7],
[31, 12],
[21, 35],
[28, 23],
[22, 24],
[23, 26],
[25, 25],



];

function __Osc__Lib__sinewave(phase) {
return Math.sin(((phase * 2) * Math.PI));
}

function __Osc__Lib__trianglewave(phase) {
return (1 - (4 * Math.abs((Math.round((phase - 0.25)) - (phase - 0.25)))));
}

function __Osc__Lib__sawwave(phase) {
return (2 * (phase - Math.round(phase)));
}

function __Osc__Lib__squarewave(phase) {
return (((phase < 0.5 ? 1 : 0) * 2) - 1);
}

function __Osc__Lib__if_math(cond, a, b) {
return ((cond * a) + ((1 - cond) * b));
}

function __Osc__Lib__switch4(n, a, b, c, d) {
return __Osc__Lib__if_math((n == 0 ? 1 : 0), a, __Osc__Lib__if_math((n == 1 ? 1 : 0), b, __Osc__Lib__if_math((n == 2 ? 1 : 0), c, __Osc__Lib__if_math((n == 3 ? 1 : 0), d, 0))));
}

function __Osc__Lib__switch3(n, a, b, c) {
return __Osc__Lib__if_math((n == 0 ? 1 : 0), a, __Osc__Lib__if_math((n == 1 ? 1 : 0), b, __Osc__Lib__if_math((n == 2 ? 1 : 0), c, 0)));
}

function __Osc__Lib__clamp(x, a, b) {
return Math.min(Math.max(x, a), b);
}

function __Osc__Lib__lerp(a, b, t) {
return (a + ((b - a) * t));
}

__m_inputs[0] = 110;
__m_outputs[0] = 0;
let __Osc__Phaser__increment = 0;
__m_inputs[1] = 0;
__m_outputs[1] = 0;
let __Osc__Smooth__s = 0.7;
let __Osc__Smooth__y_prev = 0;
function __Osc__Smooth__smoo(signal) {
let __Osc__Smooth__y_curr = ((__Osc__Smooth__s * (__Osc__Smooth__y_prev - signal)) + signal);
__Osc__Smooth__y_prev = __Osc__Smooth__y_curr;
return __Osc__Smooth__y_curr;
}

let __Osc__Smooth____y_curr_2 = 0;
__m_inputs[2] = 110;
__m_inputs[3] = 0;
__m_outputs[2] = 0;
__m_inputs[4] = 0;
__m_outputs[3] = 0;
let __Osc__outwave = 0;
function __Osc2__Lib__sinewave(phase) {
return Math.sin(((phase * 2) * Math.PI));
}

function __Osc2__Lib__trianglewave(phase) {
return (1 - (4 * Math.abs((Math.round((phase - 0.25)) - (phase - 0.25)))));
}

function __Osc2__Lib__sawwave(phase) {
return (2 * (phase - Math.round(phase)));
}

function __Osc2__Lib__squarewave(phase) {
return (((phase < 0.5 ? 1 : 0) * 2) - 1);
}

function __Osc2__Lib__if_math(cond, a, b) {
return ((cond * a) + ((1 - cond) * b));
}

function __Osc2__Lib__switch4(n, a, b, c, d) {
return __Osc2__Lib__if_math((n == 0 ? 1 : 0), a, __Osc2__Lib__if_math((n == 1 ? 1 : 0), b, __Osc2__Lib__if_math((n == 2 ? 1 : 0), c, __Osc2__Lib__if_math((n == 3 ? 1 : 0), d, 0))));
}

function __Osc2__Lib__switch3(n, a, b, c) {
return __Osc2__Lib__if_math((n == 0 ? 1 : 0), a, __Osc2__Lib__if_math((n == 1 ? 1 : 0), b, __Osc2__Lib__if_math((n == 2 ? 1 : 0), c, 0)));
}

function __Osc2__Lib__clamp(x, a, b) {
return Math.min(Math.max(x, a), b);
}

function __Osc2__Lib__lerp(a, b, t) {
return (a + ((b - a) * t));
}

__m_inputs[5] = 110;
__m_outputs[4] = 0;
let __Osc2__Phaser__increment = 0;
__m_inputs[6] = 0;
__m_outputs[5] = 0;
let __Osc2__Smooth__s = 0.7;
let __Osc2__Smooth__y_prev = 0;
function __Osc2__Smooth__smoo(signal) {
let __Osc2__Smooth__y_curr = ((__Osc2__Smooth__s * (__Osc2__Smooth__y_prev - signal)) + signal);
__Osc2__Smooth__y_prev = __Osc2__Smooth__y_curr;
return __Osc2__Smooth__y_curr;
}

let __Osc2__Smooth____y_curr_2 = 0;
__m_inputs[7] = 110;
__m_inputs[8] = 0;
__m_outputs[6] = 0;
__m_inputs[9] = 0;
__m_outputs[7] = 0;
let __Osc2__outwave = 0;
function __Osc3__Lib__sinewave(phase) {
return Math.sin(((phase * 2) * Math.PI));
}

function __Osc3__Lib__trianglewave(phase) {
return (1 - (4 * Math.abs((Math.round((phase - 0.25)) - (phase - 0.25)))));
}

function __Osc3__Lib__sawwave(phase) {
return (2 * (phase - Math.round(phase)));
}

function __Osc3__Lib__squarewave(phase) {
return (((phase < 0.5 ? 1 : 0) * 2) - 1);
}

function __Osc3__Lib__if_math(cond, a, b) {
return ((cond * a) + ((1 - cond) * b));
}

function __Osc3__Lib__switch4(n, a, b, c, d) {
return __Osc3__Lib__if_math((n == 0 ? 1 : 0), a, __Osc3__Lib__if_math((n == 1 ? 1 : 0), b, __Osc3__Lib__if_math((n == 2 ? 1 : 0), c, __Osc3__Lib__if_math((n == 3 ? 1 : 0), d, 0))));
}

function __Osc3__Lib__switch3(n, a, b, c) {
return __Osc3__Lib__if_math((n == 0 ? 1 : 0), a, __Osc3__Lib__if_math((n == 1 ? 1 : 0), b, __Osc3__Lib__if_math((n == 2 ? 1 : 0), c, 0)));
}

function __Osc3__Lib__clamp(x, a, b) {
return Math.min(Math.max(x, a), b);
}

function __Osc3__Lib__lerp(a, b, t) {
return (a + ((b - a) * t));
}

__m_inputs[10] = 110;
__m_outputs[8] = 0;
let __Osc3__Phaser__increment = 0;
__m_inputs[11] = 0;
__m_outputs[9] = 0;
let __Osc3__Smooth__s = 0.7;
let __Osc3__Smooth__y_prev = 0;
function __Osc3__Smooth__smoo(signal) {
let __Osc3__Smooth__y_curr = ((__Osc3__Smooth__s * (__Osc3__Smooth__y_prev - signal)) + signal);
__Osc3__Smooth__y_prev = __Osc3__Smooth__y_curr;
return __Osc3__Smooth__y_curr;
}

let __Osc3__Smooth____y_curr_2 = 0;
__m_inputs[12] = 110;
__m_inputs[13] = 0;
__m_outputs[10] = 0;
__m_inputs[14] = 0;
__m_outputs[11] = 0;
let __Osc3__outwave = 0;
__m_inputs[15] = 1;
__m_outputs[12] = 0;
let __Noise__randomValue = 0;
function __LFO__Lib__sinewave(phase) {
return Math.sin(((phase * 2) * Math.PI));
}

function __LFO__Lib__trianglewave(phase) {
return (1 - (4 * Math.abs((Math.round((phase - 0.25)) - (phase - 0.25)))));
}

function __LFO__Lib__sawwave(phase) {
return (2 * (phase - Math.round(phase)));
}

function __LFO__Lib__squarewave(phase) {
return (((phase < 0.5 ? 1 : 0) * 2) - 1);
}

function __LFO__Lib__if_math(cond, a, b) {
return ((cond * a) + ((1 - cond) * b));
}

function __LFO__Lib__switch4(n, a, b, c, d) {
return __LFO__Lib__if_math((n == 0 ? 1 : 0), a, __LFO__Lib__if_math((n == 1 ? 1 : 0), b, __LFO__Lib__if_math((n == 2 ? 1 : 0), c, __LFO__Lib__if_math((n == 3 ? 1 : 0), d, 0))));
}

function __LFO__Lib__switch3(n, a, b, c) {
return __LFO__Lib__if_math((n == 0 ? 1 : 0), a, __LFO__Lib__if_math((n == 1 ? 1 : 0), b, __LFO__Lib__if_math((n == 2 ? 1 : 0), c, 0)));
}

function __LFO__Lib__clamp(x, a, b) {
return Math.min(Math.max(x, a), b);
}

function __LFO__Lib__lerp(a, b, t) {
return (a + ((b - a) * t));
}

__m_inputs[16] = 110;
__m_outputs[13] = 0;
let __LFO__Phaser__increment = 0;
__m_inputs[17] = 0;
__m_outputs[14] = 0;
let __LFO__Smooth__s = 0.7;
let __LFO__Smooth__y_prev = 0;
function __LFO__Smooth__smoo(signal) {
let __LFO__Smooth__y_curr = ((__LFO__Smooth__s * (__LFO__Smooth__y_prev - signal)) + signal);
__LFO__Smooth__y_prev = __LFO__Smooth__y_curr;
return __LFO__Smooth__y_curr;
}

let __LFO__Smooth____y_curr_2 = 0;
__m_inputs[18] = 110;
__m_inputs[19] = 0;
__m_outputs[15] = 0;
__m_inputs[20] = 0;
__m_outputs[16] = 0;
let __LFO__outwave = 0;
let __OscVolume__osc1gain = 0.33;
let __OscVolume__osc2gain = 0;
let __OscVolume__osc3gain = 0;
let __OscVolume__noiseGain = 0;
__m_outputs[17] = 0;
__m_outputs[18] = 0;
__m_outputs[19] = 0;
__m_outputs[20] = 0;
let __ADSR__attackTime = 0.01;
let __ADSR__decayTime = 0.1;
let __ADSR__sustainLevel = 0.7;
let __ADSR__releaseTime = 0.1;
__m_inputs[21] = 0;
__m_outputs[21] = 0;
let __ADSR__currentVal = 0;
let __ADSR__prevGate = 0;
let __ADSR__envelopeState = 0;
let __ADSR__attackInc = 0;
let __ADSR__decayDec = 0;
let __ADSR__releaseDec = 0;
let __ADSR__risingEdge = 0;
let __ADSR__fallingEdge = 0;
function __LowPass__Lib__sinewave(phase) {
return Math.sin(((phase * 2) * Math.PI));
}

function __LowPass__Lib__trianglewave(phase) {
return (1 - (4 * Math.abs((Math.round((phase - 0.25)) - (phase - 0.25)))));
}

function __LowPass__Lib__sawwave(phase) {
return (2 * (phase - Math.round(phase)));
}

function __LowPass__Lib__squarewave(phase) {
return (((phase < 0.5 ? 1 : 0) * 2) - 1);
}

function __LowPass__Lib__if_math(cond, a, b) {
return ((cond * a) + ((1 - cond) * b));
}

function __LowPass__Lib__switch4(n, a, b, c, d) {
return __LowPass__Lib__if_math((n == 0 ? 1 : 0), a, __LowPass__Lib__if_math((n == 1 ? 1 : 0), b, __LowPass__Lib__if_math((n == 2 ? 1 : 0), c, __LowPass__Lib__if_math((n == 3 ? 1 : 0), d, 0))));
}

function __LowPass__Lib__switch3(n, a, b, c) {
return __LowPass__Lib__if_math((n == 0 ? 1 : 0), a, __LowPass__Lib__if_math((n == 1 ? 1 : 0), b, __LowPass__Lib__if_math((n == 2 ? 1 : 0), c, 0)));
}

function __LowPass__Lib__clamp(x, a, b) {
return Math.min(Math.max(x, a), b);
}

function __LowPass__Lib__lerp(a, b, t) {
return (a + ((b - a) * t));
}

let __LowPass__cutoffFrequency = 1000;
__m_inputs[22] = 0;
let __LowPass__resonance = 0.5;
__m_inputs[23] = 0;
__m_outputs[22] = 0;
let __LowPass__dt = (1 / sampleRate);
let __LowPass__previousOutput1 = 0;
let __LowPass__previousOutput2 = 0;
let __LowPass__previousOutput3 = 0;
let __LowPass__previousOutput4 = 0;
let __LowPass__cutoffFreq = 0;
let __LowPass__RC = 0;
let __LowPass__alpha = 0;
let __LowPass__buffer1 = 0;
let __LowPass__buffer2 = 0;
let __LowPass__buffer3 = 0;
let __LowPass__buffer4 = 0;
let __Echo__delayTime = 0.5;
let __Echo__feedback = 0.5;
let __Echo__dryWet = 0;
__m_inputs[24] = 0;
__m_outputs[23] = 0;
const __Echo__$delayBuffer = new Ringbuffer(sampleRate);
let __Echo__delaySamples = 0;
let __Echo__bufLen = 0;
let __Echo__readIndex = 0;
let __Echo__delayedSignal = 0;
let __Echo__toPush = 0;
let __Limiter__threshold = 0.8;
let __Limiter__recoveryRate = 0.0001;
__m_inputs[25] = 0;
__m_outputs[24] = 0;
let __Limiter__gain = 1;
let __Limiter__signalMagnitude = 0;
let __Limiter__exceed = 0;
let __Limiter__reductionFactor = 0;
function __Freeverb__Lib__sinewave(phase) {
return Math.sin(((phase * 2) * Math.PI));
}

function __Freeverb__Lib__trianglewave(phase) {
return (1 - (4 * Math.abs((Math.round((phase - 0.25)) - (phase - 0.25)))));
}

function __Freeverb__Lib__sawwave(phase) {
return (2 * (phase - Math.round(phase)));
}

function __Freeverb__Lib__squarewave(phase) {
return (((phase < 0.5 ? 1 : 0) * 2) - 1);
}

function __Freeverb__Lib__if_math(cond, a, b) {
return ((cond * a) + ((1 - cond) * b));
}

function __Freeverb__Lib__switch4(n, a, b, c, d) {
return __Freeverb__Lib__if_math((n == 0 ? 1 : 0), a, __Freeverb__Lib__if_math((n == 1 ? 1 : 0), b, __Freeverb__Lib__if_math((n == 2 ? 1 : 0), c, __Freeverb__Lib__if_math((n == 3 ? 1 : 0), d, 0))));
}

function __Freeverb__Lib__switch3(n, a, b, c) {
return __Freeverb__Lib__if_math((n == 0 ? 1 : 0), a, __Freeverb__Lib__if_math((n == 1 ? 1 : 0), b, __Freeverb__Lib__if_math((n == 2 ? 1 : 0), c, 0)));
}

function __Freeverb__Lib__clamp(x, a, b) {
return Math.min(Math.max(x, a), b);
}

function __Freeverb__Lib__lerp(a, b, t) {
return (a + ((b - a) * t));
}

let __Freeverb__dryWet = 0.5;
let __Freeverb__roomSize = 0.5;
let __Freeverb__damp = 0.5;
__m_inputs[26] = 0;
__m_outputs[25] = 0;
let __Freeverb__$combBuffer1 = new Ringbuffer(1557);
let __Freeverb__$combBuffer2 = new Ringbuffer(1617);
let __Freeverb__$combBuffer3 = new Ringbuffer(1491);
let __Freeverb__$allpassBuffer1 = new Ringbuffer(225);
let __Freeverb__$allpassBuffer2 = new Ringbuffer(556);
let __Freeverb__inputSample = 0;
let __Freeverb__combOut1 = 0;
let __Freeverb__combOut2 = 0;
let __Freeverb__combOut3 = 0;
let __Freeverb__combSum = 0;
let __Freeverb__allpassOut1 = 0;
let __Freeverb__allpassOut2 = 0;
let __Freeverb__wetSignal = 0;
function __Lib__sinewave(phase) {
return Math.sin(((phase * 2) * Math.PI));
}

function __Lib__trianglewave(phase) {
return (1 - (4 * Math.abs((Math.round((phase - 0.25)) - (phase - 0.25)))));
}

function __Lib__sawwave(phase) {
return (2 * (phase - Math.round(phase)));
}

function __Lib__squarewave(phase) {
return (((phase < 0.5 ? 1 : 0) * 2) - 1);
}

function __Lib__if_math(cond, a, b) {
return ((cond * a) + ((1 - cond) * b));
}

function __Lib__switch4(n, a, b, c, d) {
return __Lib__if_math((n == 0 ? 1 : 0), a, __Lib__if_math((n == 1 ? 1 : 0), b, __Lib__if_math((n == 2 ? 1 : 0), c, __Lib__if_math((n == 3 ? 1 : 0), d, 0))));
}

function __Lib__switch3(n, a, b, c) {
return __Lib__if_math((n == 0 ? 1 : 0), a, __Lib__if_math((n == 1 ? 1 : 0), b, __Lib__if_math((n == 2 ? 1 : 0), c, 0)));
}

function __Lib__clamp(x, a, b) {
return Math.min(Math.max(x, a), b);
}

function __Lib__lerp(a, b, t) {
return (a + ((b - a) * t));
}

const __Freq__SEMI = Math.pow(2, (1 / 12));
function __Freq__semiOffset(freq, semi) {
return (freq * Math.pow(__Freq__SEMI, semi));
}

__m_inputs[27] = 0;
__m_inputs[28] = 0;
let __Mix__balance = 0;
__m_outputs[26] = 0;
let __Mix__bal = 0;
__m_inputs[29] = 0;
let __Attenuator__balance = 0;
__m_outputs[27] = 0;
let osc1waveform = 0;
let osc2waveform = 0;
let osc3waveform = 0;
let lfowaveform = 0;
let lfoFrequency = 1;
let osc2octaveoffset = 2;
let osc2semioffset = 0;
let osc2detune = 0;
let osc3octaveoffset = 2;
let osc3semioffset = 0;
let osc3detune = 0;
let trigger = 0;
let frequency = 440;
__m_outputs[28] = 0;
__m_inputs[30] = 0;
__m_inputs[31] = 0;
__m_inputs[32] = 0;
__m_inputs[33] = 0;
__m_inputs[34] = 0;
__m_inputs[35] = 0;
__m_inputs[36] = 0;
__m_outputs[29] = 440;
__m_outputs[30] = 440;
__m_outputs[31] = 440;
__m_inputs[37] = 0;
let frequencyModAmount = 0;
let globalgate = 0;
__m_inputs[38] = 0;
__m_inputs[39] = 0;
__m_inputs[40] = 0;
let freq = 0;
let osc2detuned = 0;
let osc3detuned = 0;


class MephistoGenerator extends AudioWorkletProcessor {

    constructor() {
        super();

        this.port.onmessage = (e) => {
            console.log(e.data);

            if (e.data.command === 'init') {
                this.port.postMessage({
                    command: 'init',
                    parameters: this.parameterDescriptors(),
                    connections,
                    inputNames: __inputNames,
                    outputNames: __outputNames,
                });
            }

            if (e.data.command === 'addModule') {
                //this.registerModule(e.data.module);
            }

            if (e.data.command === 'setParameter') {
                this.scheduleSetParameter(e.data.setter);
            }

            if (e.data.command === 'addConnection') {
                this.scheduleAddConnection(e.data.connection);
            }

            if (e.data.command === 'addOutputConnection') {
                //this.scheduleAddOutputConnection(e.data.connection);
            }

            if (e.data.command === 'removeConnection') {
                this.scheduleRemoveConnection(e.data.connection);
            }
        }
    }

    scheduledParameterSetters = [];

    scheduleSetParameter(setter) {
        this.scheduledParameterSetters.push(setter);
    }

    scheduledConnections = [];

    scheduleAddConnection(connection) {
        this.scheduledConnections.push(connection); // [out, inp]
    }

    scheduledRemoveConnections = [];

    scheduleRemoveConnection(connection) {
        this.scheduledRemoveConnections.push(connection);
    }

    parameterDescriptors() {
        return [
            {name:'__OscVolume__osc1gain',initial:0.33,type:1,min:0,max:1,step:0.01}, {name:'__OscVolume__osc2gain',initial:0,type:1,min:0,max:1,step:0.01}, {name:'__OscVolume__osc3gain',initial:0,type:1,min:0,max:1,step:0.01}, {name:'__OscVolume__noiseGain',initial:0,type:1,min:0,max:1,step:0.01}, {name:'__ADSR__attackTime',min:0.01,max:10,step:0.01,initial:0.01,type:1}, {name:'__ADSR__decayTime',min:0.01,max:10,step:0.01,initial:0.1,type:1}, {name:'__ADSR__sustainLevel',min:0,max:1,step:0.01,initial:0.7,type:1}, {name:'__ADSR__releaseTime',min:0.01,max:10,step:0.01,initial:0.1,type:1}, {name:'__LowPass__cutoffFrequency',initial:1000,min:20,max:20000,step:10,type:1}, {name:'__LowPass__resonance',initial:0.5,min:0,max:4,step:0.01,type:1}, {name:'__Echo__delayTime',initial:0.5,min:0,max:1,step:0.01,type:1}, {name:'__Echo__feedback',initial:0.5,min:0,max:1,step:0.01,type:1}, {name:'__Echo__dryWet',initial:0,min:0,max:1,step:0.01,type:1}, {name:'__Limiter__threshold',min:0,max:1,step:0.01,initial:0.8,type:1}, {name:'__Limiter__recoveryRate',min:0.01,max:1,step:0.01,initial:0.0001,type:1}, {name:'__Freeverb__dryWet',initial:0.5,type:1,min:0,max:1,step:0.01}, {name:'__Freeverb__roomSize',initial:0.5,type:1,min:0,max:1,step:0.01}, {name:'__Freeverb__damp',initial:0.5,type:1,min:0,max:1,step:0.01}, {name:'__Mix__balance',initial:0,min:-1,max:1,step:0.01,type:1}, {name:'__Attenuator__balance',initial:0,min:-1,max:1,step:0.01,type:1}, {name:'osc1waveform',initial:0,type:1,min:0,max:3,step:1,sine:0,square:1,saw:2,triangle:3}, {name:'osc2waveform',initial:0,type:1,min:0,max:3,step:1,sine:0,square:1,saw:2,triangle:3}, {name:'osc3waveform',initial:0,type:1,min:0,max:3,step:1,sine:0,square:1,saw:2,triangle:3}, {name:'lfowaveform',initial:0,type:1,min:0,max:3,step:1,sine:0,square:1,saw:2,triangle:3}, {name:'lfoFrequency',initial:1,type:1,min:0,max:20,step:0.01}, {name:'osc2octaveoffset',initial:2,type:1,min:0,max:4,step:1}, {name:'osc2semioffset',initial:0,type:1,min:-12,max:12,step:1}, {name:'osc2detune',initial:0,type:1,min:-0.1,max:0.1,step:0.001}, {name:'osc3octaveoffset',initial:2,type:1,min:0,max:4,step:1}, {name:'osc3semioffset',initial:0,type:1,min:-12,max:12,step:1}, {name:'osc3detune',initial:0,type:1,min:-0.1,max:0.1,step:0.001}, {name:'trigger',initial:0,type:0}, {name:'frequency',initial:440}, {name:'frequencyModAmount',initial:0,type:1,min:0,max:1,step:0.01}, {name:'globalgate',initial:0,type:1,min:0,max:1,step:1}
        ];
    }

    process(inputs, outputs, parameters) {
        const output = outputs[0];

        const leftOutput = output[0];
        const rightOutput = output[1];

        for (let i = 0; i < this.scheduledParameterSetters.length; i++) {
            switch (this.scheduledParameterSetters[i].name) {
                case '__OscVolume__osc1gain': __OscVolume__osc1gain = this.scheduledParameterSetters[i].value; break;
case '__OscVolume__osc2gain': __OscVolume__osc2gain = this.scheduledParameterSetters[i].value; break;
case '__OscVolume__osc3gain': __OscVolume__osc3gain = this.scheduledParameterSetters[i].value; break;
case '__OscVolume__noiseGain': __OscVolume__noiseGain = this.scheduledParameterSetters[i].value; break;
case '__ADSR__attackTime': __ADSR__attackTime = this.scheduledParameterSetters[i].value; break;
case '__ADSR__decayTime': __ADSR__decayTime = this.scheduledParameterSetters[i].value; break;
case '__ADSR__sustainLevel': __ADSR__sustainLevel = this.scheduledParameterSetters[i].value; break;
case '__ADSR__releaseTime': __ADSR__releaseTime = this.scheduledParameterSetters[i].value; break;
case '__LowPass__cutoffFrequency': __LowPass__cutoffFrequency = this.scheduledParameterSetters[i].value; break;
case '__LowPass__resonance': __LowPass__resonance = this.scheduledParameterSetters[i].value; break;
case '__Echo__delayTime': __Echo__delayTime = this.scheduledParameterSetters[i].value; break;
case '__Echo__feedback': __Echo__feedback = this.scheduledParameterSetters[i].value; break;
case '__Echo__dryWet': __Echo__dryWet = this.scheduledParameterSetters[i].value; break;
case '__Limiter__threshold': __Limiter__threshold = this.scheduledParameterSetters[i].value; break;
case '__Limiter__recoveryRate': __Limiter__recoveryRate = this.scheduledParameterSetters[i].value; break;
case '__Freeverb__dryWet': __Freeverb__dryWet = this.scheduledParameterSetters[i].value; break;
case '__Freeverb__roomSize': __Freeverb__roomSize = this.scheduledParameterSetters[i].value; break;
case '__Freeverb__damp': __Freeverb__damp = this.scheduledParameterSetters[i].value; break;
case '__Mix__balance': __Mix__balance = this.scheduledParameterSetters[i].value; break;
case '__Attenuator__balance': __Attenuator__balance = this.scheduledParameterSetters[i].value; break;
case 'osc1waveform': osc1waveform = this.scheduledParameterSetters[i].value; break;
case 'osc2waveform': osc2waveform = this.scheduledParameterSetters[i].value; break;
case 'osc3waveform': osc3waveform = this.scheduledParameterSetters[i].value; break;
case 'lfowaveform': lfowaveform = this.scheduledParameterSetters[i].value; break;
case 'lfoFrequency': lfoFrequency = this.scheduledParameterSetters[i].value; break;
case 'osc2octaveoffset': osc2octaveoffset = this.scheduledParameterSetters[i].value; break;
case 'osc2semioffset': osc2semioffset = this.scheduledParameterSetters[i].value; break;
case 'osc2detune': osc2detune = this.scheduledParameterSetters[i].value; break;
case 'osc3octaveoffset': osc3octaveoffset = this.scheduledParameterSetters[i].value; break;
case 'osc3semioffset': osc3semioffset = this.scheduledParameterSetters[i].value; break;
case 'osc3detune': osc3detune = this.scheduledParameterSetters[i].value; break;
case 'trigger': trigger = this.scheduledParameterSetters[i].value; break;
case 'frequency': frequency = this.scheduledParameterSetters[i].value; break;
case 'frequencyModAmount': frequencyModAmount = this.scheduledParameterSetters[i].value; break;
case 'globalgate': globalgate = this.scheduledParameterSetters[i].value; break;
            }
        }

        this.scheduledConnections.forEach(([out, inp]) => {
            connections.push([out, inp]);
        });

        connections = connections.filter(([out, inp]) => {
            for (let i = 0; i < this.scheduledRemoveConnections.length; i++) {
                if (out === this.scheduledRemoveConnections[i][0] && inp === this.scheduledRemoveConnections[i][1]) {
                    __m_inputs[inp] = 0;
                    return false;
                }
            }

            return true;
        });

        if (this.scheduledConnections.length > 0 || this.scheduledRemoveConnections.length > 0) {
            this.port.postMessage({
                command: 'connectionsChanged',
                connections,
                inputNames: __inputNames,
                outputNames: __outputNames,
            });
        }

        this.scheduledParameterSetters = [];
        this.scheduledConnections = [];
        this.scheduledRemoveConnections = [];

        {
__Limiter__signalMagnitude = Math.abs(__m_inputs[25]);
__ADSR__attackInc = (1 / (sampleRate * __ADSR__attackTime));
__ADSR__decayDec = ((1 - __ADSR__sustainLevel) / (sampleRate * __ADSR__decayTime));
__ADSR__releaseDec = (__ADSR__sustainLevel / (sampleRate * __ADSR__releaseTime));
__m_outputs[17] = __OscVolume__osc1gain;
__m_outputs[18] = __OscVolume__osc2gain;
__m_outputs[19] = __OscVolume__osc3gain;
__m_outputs[20] = __OscVolume__noiseGain;
__LFO__Phaser__increment = (__m_inputs[16] / sampleRate);
__m_outputs[16] = __m_inputs[18];
__Osc3__Phaser__increment = (__m_inputs[10] / sampleRate);
__m_outputs[11] = __m_inputs[12];
__Osc2__Phaser__increment = (__m_inputs[5] / sampleRate);
__m_outputs[7] = __m_inputs[7];
__Osc__Phaser__increment = (__m_inputs[0] / sampleRate);
__m_outputs[3] = __m_inputs[2];
}



        for (let i = 0; i < leftOutput.length; i++) {
            // Advance each module
            __m_outputs[27] = (__m_inputs[29] * __Attenuator__balance);
__Mix__bal = ((__Mix__balance * 0.5) + 0.5);
__m_outputs[26] = (((__m_inputs[27] * (1 - __Mix__bal)) + (__m_inputs[28] * __Mix__bal)) / 2);
__Freeverb__inputSample = __m_inputs[26];
__Freeverb__combOut1 = ((Rb.read(__Freeverb__$combBuffer1, 0) * __Freeverb__roomSize) + __Freeverb__inputSample);
__Freeverb__combOut2 = ((Rb.read(__Freeverb__$combBuffer2, 0) * __Freeverb__roomSize) + __Freeverb__inputSample);
__Freeverb__combOut3 = ((Rb.read(__Freeverb__$combBuffer3, 0) * __Freeverb__roomSize) + __Freeverb__inputSample);
Rb.push(__Freeverb__$combBuffer1, ((__Freeverb__combOut1 * (1 - __Freeverb__damp)) + (Rb.read(__Freeverb__$combBuffer1, 1) * __Freeverb__damp)));
Rb.push(__Freeverb__$combBuffer2, ((__Freeverb__combOut2 * (1 - __Freeverb__damp)) + (Rb.read(__Freeverb__$combBuffer2, 1) * __Freeverb__damp)));
Rb.push(__Freeverb__$combBuffer3, ((__Freeverb__combOut3 * (1 - __Freeverb__damp)) + (Rb.read(__Freeverb__$combBuffer3, 1) * __Freeverb__damp)));
__Freeverb__combSum = (((__Freeverb__combOut1 + __Freeverb__combOut2) + __Freeverb__combOut3) / 3);
__Freeverb__allpassOut1 = -(__Freeverb__combSum + Rb.read(__Freeverb__$allpassBuffer1, 0));
Rb.push(__Freeverb__$allpassBuffer1, __Freeverb__combSum);
__Freeverb__allpassOut2 = -(__Freeverb__allpassOut1 + Rb.read(__Freeverb__$allpassBuffer2, 0));
Rb.push(__Freeverb__$allpassBuffer2, __Freeverb__allpassOut1);
__Freeverb__wetSignal = __Freeverb__allpassOut2;
__m_outputs[25] = ((__Freeverb__inputSample * (1 - __Freeverb__dryWet)) + (__Freeverb__wetSignal * __Freeverb__dryWet));
__Limiter__exceed = (__Limiter__signalMagnitude - __Limiter__threshold);
__Limiter__reductionFactor = Math.exp(-(__Limiter__exceed * __Limiter__recoveryRate));
__Limiter__gain = __Limiter__reductionFactor;
__m_outputs[24] = (__m_inputs[25] * __Limiter__gain);
__Echo__delaySamples = (__Echo__delayTime * sampleRate);
__Echo__bufLen = Rb.length(__Echo__$delayBuffer);
__Echo__readIndex = (__Echo__bufLen - __Echo__delaySamples);
__Echo__readIndex = Math.max(0, Math.min(__Echo__readIndex, (__Echo__bufLen - 1)));
__Echo__delayedSignal = Rb.read(__Echo__$delayBuffer, __Echo__readIndex);
__Echo__toPush = (__m_inputs[24] + (__Echo__delayedSignal * __Echo__feedback));
Rb.push(__Echo__$delayBuffer, __Echo__toPush);
__m_outputs[23] = ((__m_inputs[24] * (1 - __Echo__dryWet)) + (__Echo__delayedSignal * __Echo__dryWet));
__LowPass__cutoffFreq = __LowPass__Lib__clamp((__LowPass__cutoffFrequency + (__m_inputs[22] * 10000)), 20, 20000);
__LowPass__RC = (1 / ((2 * Math.PI) * __LowPass__cutoffFreq));
__LowPass__alpha = (__LowPass__dt / (__LowPass__RC + __LowPass__dt));
__LowPass__buffer1 = ((__LowPass__alpha * (__m_inputs[23] - (__LowPass__resonance * __LowPass__previousOutput4))) + ((1 - __LowPass__alpha) * __LowPass__previousOutput1));
__LowPass__previousOutput1 = __LowPass__buffer1;
__LowPass__buffer2 = ((__LowPass__alpha * __LowPass__buffer1) + ((1 - __LowPass__alpha) * __LowPass__previousOutput2));
__LowPass__previousOutput2 = __LowPass__buffer2;
__LowPass__buffer3 = ((__LowPass__alpha * __LowPass__buffer2) + ((1 - __LowPass__alpha) * __LowPass__previousOutput3));
__LowPass__previousOutput3 = __LowPass__buffer3;
__LowPass__buffer4 = ((__LowPass__alpha * __LowPass__buffer3) + ((1 - __LowPass__alpha) * __LowPass__previousOutput4));
__LowPass__previousOutput4 = __LowPass__buffer4;
__m_outputs[22] = __LowPass__buffer4;
__ADSR__risingEdge = (__m_inputs[21] * (1 - __ADSR__prevGate));
__ADSR__fallingEdge = (__ADSR__prevGate * (1 - __m_inputs[21]));
__ADSR__envelopeState = ((__ADSR__envelopeState * (1 - __ADSR__risingEdge)) + (1 * __ADSR__risingEdge));
__ADSR__envelopeState = ((__ADSR__envelopeState * (1 - ((__ADSR__currentVal >= 1 ? 1 : 0) * (__ADSR__envelopeState == 1 ? 1 : 0)))) + ((2 * (__ADSR__currentVal >= 1 ? 1 : 0)) * (__ADSR__envelopeState == 1 ? 1 : 0)));
__ADSR__envelopeState = ((__ADSR__envelopeState * (1 - ((__ADSR__currentVal <= __ADSR__sustainLevel ? 1 : 0) * (__ADSR__envelopeState == 2 ? 1 : 0)))) + ((3 * (__ADSR__currentVal <= __ADSR__sustainLevel ? 1 : 0)) * (__ADSR__envelopeState == 2 ? 1 : 0)));
__ADSR__envelopeState = ((__ADSR__envelopeState * (1 - (__ADSR__fallingEdge * (((__ADSR__envelopeState == 1 ? 1 : 0) + (__ADSR__envelopeState == 2 ? 1 : 0)) + (__ADSR__envelopeState == 3 ? 1 : 0))))) + ((4 * __ADSR__fallingEdge) * (((__ADSR__envelopeState == 1 ? 1 : 0) + (__ADSR__envelopeState == 2 ? 1 : 0)) + (__ADSR__envelopeState == 3 ? 1 : 0))));
__ADSR__currentVal = (((__ADSR__currentVal + (__ADSR__attackInc * (__ADSR__envelopeState == 1 ? 1 : 0))) - (__ADSR__decayDec * (__ADSR__envelopeState == 2 ? 1 : 0))) - (__ADSR__releaseDec * (__ADSR__envelopeState == 4 ? 1 : 0)));
__ADSR__currentVal = ((__ADSR__currentVal * (__ADSR__currentVal >= 0 ? 1 : 0)) + (0 * (__ADSR__currentVal < 0 ? 1 : 0)));
__ADSR__currentVal = ((__ADSR__currentVal * (__ADSR__currentVal <= 1 ? 1 : 0)) + (1 * (__ADSR__currentVal > 1 ? 1 : 0)));
__ADSR__prevGate = __m_inputs[21];
__m_outputs[21] = __ADSR__currentVal;
__LFO__Smooth____y_curr_2 = ((__LFO__Smooth__s * (__LFO__Smooth__y_prev - __m_inputs[17])) + __m_inputs[17]);
__LFO__Smooth__y_prev = __LFO__Smooth____y_curr_2;
__m_outputs[14] = __LFO__Smooth____y_curr_2;
__m_outputs[13] = (__LFO__Phaser__increment + (__m_outputs[13] - Math.floor((__LFO__Phaser__increment + __m_outputs[13]))));
__LFO__outwave = 0;
if ((__m_inputs[19] == 0 ? 1 : 0)) {
__LFO__outwave = __LFO__Lib__sinewave(__m_inputs[20]);
}  else if ((__m_inputs[19] == 1 ? 1 : 0)) {
__LFO__outwave = __LFO__Lib__squarewave(__m_inputs[20]);
}  else if ((__m_inputs[19] == 2 ? 1 : 0)) {
__LFO__outwave = __LFO__Lib__sawwave(__m_inputs[20]);
}  else if ((__m_inputs[19] == 3 ? 1 : 0)) {
__LFO__outwave = __LFO__Lib__trianglewave(__m_inputs[20]);
} 



;
__m_outputs[15] = __LFO__outwave;
__Noise__randomValue = Math.random();
__m_outputs[12] = ((__Noise__randomValue * 2) - 1);
__m_outputs[12] = (__m_outputs[12] * __m_inputs[15]);
__Osc3__Smooth____y_curr_2 = ((__Osc3__Smooth__s * (__Osc3__Smooth__y_prev - __m_inputs[11])) + __m_inputs[11]);
__Osc3__Smooth__y_prev = __Osc3__Smooth____y_curr_2;
__m_outputs[9] = __Osc3__Smooth____y_curr_2;
__m_outputs[8] = (__Osc3__Phaser__increment + (__m_outputs[8] - Math.floor((__Osc3__Phaser__increment + __m_outputs[8]))));
__Osc3__outwave = 0;
if ((__m_inputs[13] == 0 ? 1 : 0)) {
__Osc3__outwave = __Osc3__Lib__sinewave(__m_inputs[14]);
}  else if ((__m_inputs[13] == 1 ? 1 : 0)) {
__Osc3__outwave = __Osc3__Lib__squarewave(__m_inputs[14]);
}  else if ((__m_inputs[13] == 2 ? 1 : 0)) {
__Osc3__outwave = __Osc3__Lib__sawwave(__m_inputs[14]);
}  else if ((__m_inputs[13] == 3 ? 1 : 0)) {
__Osc3__outwave = __Osc3__Lib__trianglewave(__m_inputs[14]);
} 



;
__m_outputs[10] = __Osc3__outwave;
__Osc2__Smooth____y_curr_2 = ((__Osc2__Smooth__s * (__Osc2__Smooth__y_prev - __m_inputs[6])) + __m_inputs[6]);
__Osc2__Smooth__y_prev = __Osc2__Smooth____y_curr_2;
__m_outputs[5] = __Osc2__Smooth____y_curr_2;
__m_outputs[4] = (__Osc2__Phaser__increment + (__m_outputs[4] - Math.floor((__Osc2__Phaser__increment + __m_outputs[4]))));
__Osc2__outwave = 0;
if ((__m_inputs[8] == 0 ? 1 : 0)) {
__Osc2__outwave = __Osc2__Lib__sinewave(__m_inputs[9]);
}  else if ((__m_inputs[8] == 1 ? 1 : 0)) {
__Osc2__outwave = __Osc2__Lib__squarewave(__m_inputs[9]);
}  else if ((__m_inputs[8] == 2 ? 1 : 0)) {
__Osc2__outwave = __Osc2__Lib__sawwave(__m_inputs[9]);
}  else if ((__m_inputs[8] == 3 ? 1 : 0)) {
__Osc2__outwave = __Osc2__Lib__trianglewave(__m_inputs[9]);
} 



;
__m_outputs[6] = __Osc2__outwave;
__Osc__Smooth____y_curr_2 = ((__Osc__Smooth__s * (__Osc__Smooth__y_prev - __m_inputs[1])) + __m_inputs[1]);
__Osc__Smooth__y_prev = __Osc__Smooth____y_curr_2;
__m_outputs[1] = __Osc__Smooth____y_curr_2;
__m_outputs[0] = (__Osc__Phaser__increment + (__m_outputs[0] - Math.floor((__Osc__Phaser__increment + __m_outputs[0]))));
__Osc__outwave = 0;
if ((__m_inputs[3] == 0 ? 1 : 0)) {
__Osc__outwave = __Osc__Lib__sinewave(__m_inputs[4]);
}  else if ((__m_inputs[3] == 1 ? 1 : 0)) {
__Osc__outwave = __Osc__Lib__squarewave(__m_inputs[4]);
}  else if ((__m_inputs[3] == 2 ? 1 : 0)) {
__Osc__outwave = __Osc__Lib__sawwave(__m_inputs[4]);
}  else if ((__m_inputs[3] == 3 ? 1 : 0)) {
__Osc__outwave = __Osc__Lib__trianglewave(__m_inputs[4]);
} 



;
__m_outputs[2] = __Osc__outwave;
freq = (frequency + ((frequency * __m_inputs[37]) * frequencyModAmount));
__m_outputs[29] = freq;
osc2detuned = (freq * (1 + osc2detune));
osc3detuned = (freq * (1 + osc3detune));
__m_outputs[30] = __Freq__semiOffset((osc2detuned * osc2octaveoffset), osc2semioffset);
__m_outputs[31] = __Freq__semiOffset((osc3detuned * osc3octaveoffset), osc3semioffset);
__m_outputs[28] = ((((((__m_inputs[30] * __m_outputs[17]) + (__m_inputs[31] * __m_outputs[18])) + (__m_inputs[32] * __m_outputs[19])) + (__m_inputs[33] * __m_outputs[20])) * __m_inputs[35]) * globalgate);


            connections.forEach(([out, inp]) => {
                __m_inputs[inp] = __m_outputs[out];
            });

            __m_inputs[3] = osc1waveform;
__m_inputs[8] = osc2waveform;
__m_inputs[13] = osc3waveform;
__m_inputs[19] = lfowaveform;
__m_inputs[18] = lfoFrequency;
__m_inputs[21] = trigger;
leftOutput[i] = __m_outputs[24];
rightOutput && (rightOutput[i] = __m_outputs[24]);

        }

        return true;
    }
}

console.log('SAMPLE RATE', sampleRate);

registerProcessor('mephisto-generator', MephistoGenerator);