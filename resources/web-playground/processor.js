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

function __Osc__Lib__if(cond, a, b) {
return ((cond * a) + ((1 - cond) * b));
}

function __Osc__Lib__switch4(n, a, b, c, d) {
return __Osc__Lib__if((n == 0 ? 1 : 0), a, __Osc__Lib__if((n == 1 ? 1 : 0), b, __Osc__Lib__if((n == 2 ? 1 : 0), c, __Osc__Lib__if((n == 3 ? 1 : 0), d, 0))));
}

let __Osc__Phaser__frequency = 110;
let __Osc__Phaser__phase = 0;
let __Osc__Phaser__increment = 0;
let __Osc__frequency = 110;
let __Osc__gain = 0.7;
let __Osc__wave = 0;
let __Osc__out = 0;
let __Osc__phase = 0;
let __Osc__freq = 0;
let __Osc__sine = 0;
let __Osc__square = 0;
let __Osc__saw = 0;
let __Osc__triangle = 0;
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

function __Osc2__Lib__if(cond, a, b) {
return ((cond * a) + ((1 - cond) * b));
}

function __Osc2__Lib__switch4(n, a, b, c, d) {
return __Osc2__Lib__if((n == 0 ? 1 : 0), a, __Osc2__Lib__if((n == 1 ? 1 : 0), b, __Osc2__Lib__if((n == 2 ? 1 : 0), c, __Osc2__Lib__if((n == 3 ? 1 : 0), d, 0))));
}

let __Osc2__Phaser__frequency = 110;
let __Osc2__Phaser__phase = 0;
let __Osc2__Phaser__increment = 0;
let __Osc2__frequency = 110;
let __Osc2__gain = 0.7;
let __Osc2__wave = 0;
let __Osc2__out = 0;
let __Osc2__phase = 0;
let __Osc2__freq = 0;
let __Osc2__sine = 0;
let __Osc2__square = 0;
let __Osc2__saw = 0;
let __Osc2__triangle = 0;
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

function __Osc3__Lib__if(cond, a, b) {
return ((cond * a) + ((1 - cond) * b));
}

function __Osc3__Lib__switch4(n, a, b, c, d) {
return __Osc3__Lib__if((n == 0 ? 1 : 0), a, __Osc3__Lib__if((n == 1 ? 1 : 0), b, __Osc3__Lib__if((n == 2 ? 1 : 0), c, __Osc3__Lib__if((n == 3 ? 1 : 0), d, 0))));
}

let __Osc3__Phaser__frequency = 110;
let __Osc3__Phaser__phase = 0;
let __Osc3__Phaser__increment = 0;
let __Osc3__frequency = 110;
let __Osc3__gain = 0.7;
let __Osc3__wave = 0;
let __Osc3__out = 0;
let __Osc3__phase = 0;
let __Osc3__freq = 0;
let __Osc3__sine = 0;
let __Osc3__square = 0;
let __Osc3__saw = 0;
let __Osc3__triangle = 0;
let __Osc3__outwave = 0;
function __Drum__Lib__sinewave(phase) {
return Math.sin(((phase * 2) * Math.PI));
}

function __Drum__Lib__trianglewave(phase) {
return (1 - (4 * Math.abs((Math.round((phase - 0.25)) - (phase - 0.25)))));
}

function __Drum__Lib__sawwave(phase) {
return (2 * (phase - Math.round(phase)));
}

function __Drum__Lib__squarewave(phase) {
return (((phase < 0.5 ? 1 : 0) * 2) - 1);
}

function __Drum__Lib__if(cond, a, b) {
return ((cond * a) + ((1 - cond) * b));
}

function __Drum__Lib__switch4(n, a, b, c, d) {
return __Drum__Lib__if((n == 0 ? 1 : 0), a, __Drum__Lib__if((n == 1 ? 1 : 0), b, __Drum__Lib__if((n == 2 ? 1 : 0), c, __Drum__Lib__if((n == 3 ? 1 : 0), d, 0))));
}

let __Drum__Phaser__frequency = 110;
let __Drum__Phaser__phase = 0;
let __Drum__Phaser__increment = 0;
let __Drum__AR__attackTime = 0.01;
let __Drum__AR__releaseTime = 0.1;
let __Drum__AR__trigger = 0;
let __Drum__AR__curve = 0;
let __Drum__AR__currentVal = 0;
let __Drum__AR__prevTrigger = 0;
let __Drum__AR__envelopeState = 0;
let __Drum__AR__attackInc = 0;
let __Drum__AR__releaseDec = 0;
let __Drum__AR__risingEdge = 0;
let __Drum__AR__increase = 0;
let __Drum__AR__decrease = 0;
let __Drum__frequency = 110;
let __Drum__gain = 0.7;
let __Drum__wave = 0;
let __Drum__drum_trigger = 0;
let __Drum__out = 0;
let __Drum__freq = 0;
let __Drum__trigger = 0;
let __Drum__phase = 0;
let __Drum__ar_curve = 0;
let __Drum__sine = 0;
let __Drum__square = 0;
let __Drum__saw = 0;
let __Drum__triangle = 0;
let __Drum__outwave = 0;
let osc2enabled = 0;
let osc3enabled = 0;
let out = 0;
let osc1 = 0;
let osc2 = 0;
let osc3 = 0;
let drum = 0;


class MephistoGenerator extends AudioWorkletProcessor {

    constructor() {
        super();

        this.port.onmessage = (e) => {
            console.log(e.data);

            if (e.data.command === 'init') {
                this.port.postMessage({
                    command: 'init',
                    parameters: this.parameterDescriptors()
                });
            }

            if (e.data.command === 'addModule') {
                //this.registerModule(e.data.module);
            }

            if (e.data.command === 'setParameter') {
                this.scheduleSetParameter(e.data.setter);
            }

            if (e.data.command === 'addModulesConnection') {
                //this.scheduleAddModulesConnection(e.data.connection);
            }

            if (e.data.command === 'addOutputConnection') {
                //this.scheduleAddOutputConnection(e.data.connection);
            }

            if (e.data.command === 'removeModuleConnections') {
                //this.scheduleRemoveModuleConnections(e.data.module);
            }
        }
    }

    scheduledParameterSetters = [];

    /**
    * @param {function} setter
    */
    scheduleSetParameter(setter) {
        this.scheduledParameterSetters.push(setter);
    }

    parameterDescriptors() {
        return [
            {name:'__Osc__frequency',initial:110,type:1,min:55,max:880,step:0.01}, {name:'__Osc__gain',initial:0.7,type:1,min:0,max:1,step:0.01}, {name:'__Osc__wave',initial:0,type:1,min:0,max:3,step:1,sine:0,square:1,saw:2,triangle:3}, {name:'__Osc2__frequency',initial:110,type:1,min:55,max:880,step:0.01}, {name:'__Osc2__gain',initial:0.7,type:1,min:0,max:1,step:0.01}, {name:'__Osc2__wave',initial:0,type:1,min:0,max:3,step:1,sine:0,square:1,saw:2,triangle:3}, {name:'__Osc3__frequency',initial:110,type:1,min:55,max:880,step:0.01}, {name:'__Osc3__gain',initial:0.7,type:1,min:0,max:1,step:0.01}, {name:'__Osc3__wave',initial:0,type:1,min:0,max:3,step:1,sine:0,square:1,saw:2,triangle:3}, {name:'__Drum__AR__attackTime',min:0.01,max:10,step:0.01,initial:0.01}, {name:'__Drum__AR__releaseTime',min:0.01,max:10,step:0.01,initial:0.1}, {name:'__Drum__frequency',initial:110,type:1,min:0,max:1000,step:0.01}, {name:'__Drum__gain',initial:0.7,type:1,min:0,max:1,step:0.01}, {name:'__Drum__wave',initial:0,sine:0,square:1,saw:2,triangle:3,type:1,min:0,max:3,step:1}, {name:'__Drum__drum_trigger',initial:0,type:0}, {name:'osc2enabled',initial:0,type:1,min:0,max:1,step:1}, {name:'osc3enabled',initial:0,type:1,min:0,max:1,step:1}
        ];
    }

    process(inputs, outputs, parameters) {
        const output = outputs[0];

        const leftOutput = output[0];
        const rightOutput = output[1];

        for (let i = 0; i < this.scheduledParameterSetters.length; i++) {
            switch (this.scheduledParameterSetters[i].name) {
                case '__Osc__frequency': __Osc__frequency = this.scheduledParameterSetters[i].value; break;
case '__Osc__gain': __Osc__gain = this.scheduledParameterSetters[i].value; break;
case '__Osc__wave': __Osc__wave = this.scheduledParameterSetters[i].value; break;
case '__Osc2__frequency': __Osc2__frequency = this.scheduledParameterSetters[i].value; break;
case '__Osc2__gain': __Osc2__gain = this.scheduledParameterSetters[i].value; break;
case '__Osc2__wave': __Osc2__wave = this.scheduledParameterSetters[i].value; break;
case '__Osc3__frequency': __Osc3__frequency = this.scheduledParameterSetters[i].value; break;
case '__Osc3__gain': __Osc3__gain = this.scheduledParameterSetters[i].value; break;
case '__Osc3__wave': __Osc3__wave = this.scheduledParameterSetters[i].value; break;
case '__Drum__AR__attackTime': __Drum__AR__attackTime = this.scheduledParameterSetters[i].value; break;
case '__Drum__AR__releaseTime': __Drum__AR__releaseTime = this.scheduledParameterSetters[i].value; break;
case '__Drum__frequency': __Drum__frequency = this.scheduledParameterSetters[i].value; break;
case '__Drum__gain': __Drum__gain = this.scheduledParameterSetters[i].value; break;
case '__Drum__wave': __Drum__wave = this.scheduledParameterSetters[i].value; break;
case '__Drum__drum_trigger': __Drum__drum_trigger = this.scheduledParameterSetters[i].value; break;
case 'osc2enabled': osc2enabled = this.scheduledParameterSetters[i].value; break;
case 'osc3enabled': osc3enabled = this.scheduledParameterSetters[i].value; break;
            }
        }

        this.scheduledParameterSetters = [];

        /*console.trace('FIX ME, block');*/ {
__Drum__AR__attackInc = (1 / (sampleRate * __Drum__AR__attackTime));
__Drum__AR__releaseDec = (1 / (sampleRate * __Drum__AR__releaseTime));
__Drum__Phaser__increment = (__Drum__Phaser__frequency / sampleRate);
__Drum__trigger = __Drum__drum_trigger;
__Osc3__Phaser__increment = (__Osc3__Phaser__frequency / sampleRate);
__Osc3__freq = __Osc3__frequency;
__Osc2__Phaser__increment = (__Osc2__Phaser__frequency / sampleRate);
__Osc2__freq = __Osc2__frequency;
__Osc__Phaser__increment = (__Osc__Phaser__frequency / sampleRate);
__Osc__freq = __Osc__frequency;
}



        for (let i = 0; i < leftOutput.length; ++i) {
            // Advance each module
            __Drum__AR__risingEdge = (__Drum__AR__trigger * (1 - __Drum__AR__prevTrigger));
__Drum__AR__envelopeState = (__Drum__AR__envelopeState + (__Drum__AR__risingEdge * (1 - __Drum__AR__envelopeState)));
__Drum__AR__increase = (__Drum__AR__attackInc * (__Drum__AR__envelopeState == 1 ? 1 : 0));
__Drum__AR__decrease = (__Drum__AR__releaseDec * (__Drum__AR__envelopeState == 2 ? 1 : 0));
__Drum__AR__currentVal = ((__Drum__AR__currentVal + __Drum__AR__increase) - __Drum__AR__decrease);
__Drum__AR__envelopeState = (__Drum__AR__envelopeState + ((__Drum__AR__currentVal >= 1 ? 1 : 0) * (__Drum__AR__envelopeState == 1 ? 1 : 0)));
__Drum__AR__currentVal = (__Drum__AR__currentVal * (__Drum__AR__currentVal > 0 ? 1 : 0));
__Drum__AR__envelopeState = (__Drum__AR__envelopeState * (__Drum__AR__currentVal > 0 ? 1 : 0));
__Drum__AR__prevTrigger = __Drum__AR__trigger;
__Drum__AR__curve = __Drum__AR__currentVal;
__Drum__Phaser__phase = (__Drum__Phaser__increment + (__Drum__Phaser__phase - Math.floor((__Drum__Phaser__increment + __Drum__Phaser__phase))));
__Drum__freq = (__Drum__frequency * __Drum__ar_curve);
__Drum__sine = __Drum__Lib__sinewave(__Drum__phase);
__Drum__square = __Drum__Lib__squarewave(__Drum__phase);
__Drum__saw = __Drum__Lib__sawwave(__Drum__phase);
__Drum__triangle = __Drum__Lib__trianglewave(__Drum__phase);
__Drum__outwave = __Drum__Lib__switch4(__Drum__wave, __Drum__sine, __Drum__square, __Drum__saw, __Drum__triangle);
__Drum__out = ((__Drum__outwave * __Drum__gain) * __Drum__ar_curve);
__Osc3__Phaser__phase = (__Osc3__Phaser__increment + (__Osc3__Phaser__phase - Math.floor((__Osc3__Phaser__increment + __Osc3__Phaser__phase))));
__Osc3__sine = __Osc3__Lib__sinewave(__Osc3__phase);
__Osc3__square = __Osc3__Lib__squarewave(__Osc3__phase);
__Osc3__saw = __Osc3__Lib__sawwave(__Osc3__phase);
__Osc3__triangle = __Osc3__Lib__trianglewave(__Osc3__phase);
__Osc3__outwave = __Osc3__Lib__switch4(__Osc3__wave, __Osc3__sine, __Osc3__square, __Osc3__saw, __Osc3__triangle);
__Osc3__out = (__Osc3__outwave * __Osc3__gain);
__Osc2__Phaser__phase = (__Osc2__Phaser__increment + (__Osc2__Phaser__phase - Math.floor((__Osc2__Phaser__increment + __Osc2__Phaser__phase))));
__Osc2__sine = __Osc2__Lib__sinewave(__Osc2__phase);
__Osc2__square = __Osc2__Lib__squarewave(__Osc2__phase);
__Osc2__saw = __Osc2__Lib__sawwave(__Osc2__phase);
__Osc2__triangle = __Osc2__Lib__trianglewave(__Osc2__phase);
__Osc2__outwave = __Osc2__Lib__switch4(__Osc2__wave, __Osc2__sine, __Osc2__square, __Osc2__saw, __Osc2__triangle);
__Osc2__out = (__Osc2__outwave * __Osc2__gain);
__Osc__Phaser__phase = (__Osc__Phaser__increment + (__Osc__Phaser__phase - Math.floor((__Osc__Phaser__increment + __Osc__Phaser__phase))));
__Osc__sine = __Osc__Lib__sinewave(__Osc__phase);
__Osc__square = __Osc__Lib__squarewave(__Osc__phase);
__Osc__saw = __Osc__Lib__sawwave(__Osc__phase);
__Osc__triangle = __Osc__Lib__trianglewave(__Osc__phase);
__Osc__outwave = __Osc__Lib__switch4(__Osc__wave, __Osc__sine, __Osc__square, __Osc__saw, __Osc__triangle);
__Osc__out = (__Osc__outwave * __Osc__gain);
out = (((osc1 + drum) + (osc2 * osc2enabled)) + (osc3 * osc3enabled));

            leftOutput[i] = __Drum__out;
rightOutput && (rightOutput[i] = __Drum__out);
__Drum__Phaser__frequency = __Drum__freq;
__Drum__phase = __Drum__Phaser__phase;
__Drum__AR__trigger = __Drum__trigger;
__Drum__ar_curve = __Drum__AR__curve;
__Osc3__Phaser__frequency = __Osc3__freq;
__Osc3__phase = __Osc3__Phaser__phase;
__Osc2__Phaser__frequency = __Osc2__freq;
__Osc2__phase = __Osc2__Phaser__phase;
__Osc__Phaser__frequency = __Osc__freq;
__Osc__phase = __Osc__Phaser__phase;
osc1 = __Osc__out;
osc2 = __Osc2__out;
osc3 = __Osc3__out;
drum = __Drum__out;
leftOutput[i] = out;
rightOutput && (rightOutput[i] = out);



        }

        return true;
    }
}

console.log('SAMPLE RATE', sampleRate);

registerProcessor('mephisto-generator', MephistoGenerator);