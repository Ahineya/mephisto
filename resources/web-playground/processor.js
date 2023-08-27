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

function __Lib__if(cond, a, b) {
return ((cond * a) + ((1 - cond) * b));
}

function __Lib__switch4(n, a, b, c, d) {
return __Lib__if((n == 0 ? 1 : 0), a, __Lib__if((n == 1 ? 1 : 0), b, __Lib__if((n == 2 ? 1 : 0), c, __Lib__if((n == 3 ? 1 : 0), d, 0))));
}

let __Phaser__frequency = 110;
let __Phaser__phase = 0;
let __Phaser__increment = 0;
let __AR__attackTime = 0.01;
let __AR__releaseTime = 0.1;
let __AR__trigger = 0;
let __AR__curve = 0;
let __AR__currentVal = 0;
let __AR__prevTrigger = 0;
let __AR__envelopeState = 0;
let __AR__attackInc = 0;
let __AR__releaseDec = 0;
let __AR__risingEdge = 0;
let __AR__increase = 0;
let __AR__decrease = 0;
let frequency = 110;
let gain = 0.7;
let wave = 0;
let envelope_trigger = 0;
let out = 0;
let freq = 0;
let trigger = 0;
let phase = 0;
let ar_curve = 0;
let sine = 0;
let square = 0;
let saw = 0;
let triangle = 0;
let outwave = 0;


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
            {name:'__AR__attackTime',min:0.01,max:10,step:0.01,initial:0.01}, {name:'__AR__releaseTime',min:0.01,max:10,step:0.01,initial:0.1}, {name:'frequency',initial:110}, {name:'gain',initial:0.7}, {name:'wave',initial:0,sine:0,square:1,saw:2,triangle:3}, {name:'envelope_trigger',initial:0}
        ];
    }

    process(inputs, outputs, parameters) {
        const output = outputs[0];

        const leftOutput = output[0];
        const rightOutput = output[1];

        for (let i = 0; i < this.scheduledParameterSetters.length; i++) {
            switch (this.scheduledParameterSetters[i].name) {
                case '__AR__attackTime': __AR__attackTime = this.scheduledParameterSetters[i].value; break;
case '__AR__releaseTime': __AR__releaseTime = this.scheduledParameterSetters[i].value; break;
case 'frequency': frequency = this.scheduledParameterSetters[i].value; break;
case 'gain': gain = this.scheduledParameterSetters[i].value; break;
case 'wave': wave = this.scheduledParameterSetters[i].value; break;
case 'envelope_trigger': envelope_trigger = this.scheduledParameterSetters[i].value; break;
            }
        }

        this.scheduledParameterSetters = [];

        /*console.trace('FIX ME, block');*/ {
__AR__attackInc = (1 / (sampleRate * __AR__attackTime));
__AR__releaseDec = (1 / (sampleRate * __AR__releaseTime));
__Phaser__increment = (__Phaser__frequency / sampleRate);
trigger = envelope_trigger;
}



        for (let i = 0; i < leftOutput.length; ++i) {
            // Advance each module
            __AR__risingEdge = (__AR__trigger * (1 - __AR__prevTrigger));
__AR__envelopeState = (__AR__envelopeState + (__AR__risingEdge * (1 - __AR__envelopeState)));
__AR__increase = (__AR__attackInc * (__AR__envelopeState == 1 ? 1 : 0));
__AR__decrease = (__AR__releaseDec * (__AR__envelopeState == 2 ? 1 : 0));
__AR__currentVal = ((__AR__currentVal + __AR__increase) - __AR__decrease);
__AR__envelopeState = (__AR__envelopeState + ((__AR__currentVal >= 1 ? 1 : 0) * (__AR__envelopeState == 1 ? 1 : 0)));
__AR__currentVal = (__AR__currentVal * (__AR__currentVal > 0 ? 1 : 0));
__AR__envelopeState = (__AR__envelopeState * (__AR__currentVal > 0 ? 1 : 0));
__AR__curve = __AR__currentVal;
__AR__prevTrigger = __AR__trigger;
__Phaser__phase = (__Phaser__increment + (__Phaser__phase - Math.floor((__Phaser__increment + __Phaser__phase))));
freq = (frequency * ar_curve);
sine = __Lib__sinewave(phase);
square = __Lib__squarewave(phase);
saw = __Lib__sawwave(phase);
triangle = __Lib__trianglewave(phase);
outwave = __Lib__switch4(wave, sine, square, saw, triangle);
out = ((outwave * gain) * ar_curve);

            leftOutput[i] = out;
rightOutput && (rightOutput[i] = out);
__Phaser__frequency = freq;
__AR__trigger = trigger;
phase = __Phaser__phase;
ar_curve = __AR__curve;



        }

        return true;
    }
}

console.log('SAMPLE RATE', sampleRate);

registerProcessor('mephisto-generator', MephistoGenerator);