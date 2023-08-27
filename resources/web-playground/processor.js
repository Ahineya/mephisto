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
            {name:'__Osc__frequency',initial:110,type:0,min:55,max:880,step:0.01}, {name:'__Osc__gain',initial:0.7,type:0,min:0,max:1,step:0.01}, {name:'__Osc__wave',initial:0,type:0,min:0,max:3,step:1,sine:0,square:1,saw:2,triangle:3}
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
            }
        }

        this.scheduledParameterSetters = [];

        /*console.trace('FIX ME, block');*/ {
__Osc__Phaser__increment = (__Osc__Phaser__frequency / sampleRate);
__Osc__freq = __Osc__frequency;
}



        for (let i = 0; i < leftOutput.length; ++i) {
            // Advance each module
            __Osc__Phaser__phase = (__Osc__Phaser__increment + (__Osc__Phaser__phase - Math.floor((__Osc__Phaser__increment + __Osc__Phaser__phase))));
__Osc__sine = __Osc__Lib__sinewave(__Osc__phase);
__Osc__square = __Osc__Lib__squarewave(__Osc__phase);
__Osc__saw = __Osc__Lib__sawwave(__Osc__phase);
__Osc__triangle = __Osc__Lib__trianglewave(__Osc__phase);
__Osc__outwave = __Osc__Lib__switch4(__Osc__wave, __Osc__sine, __Osc__square, __Osc__saw, __Osc__triangle);
__Osc__out = (__Osc__outwave * __Osc__gain);

            __Osc__Phaser__frequency = __Osc__freq;
__Osc__phase = __Osc__Phaser__phase;
leftOutput[i] = __Osc__out;
rightOutput && (rightOutput[i] = __Osc__out);



        }

        return true;
    }
}

console.log('SAMPLE RATE', sampleRate);

registerProcessor('mephisto-generator', MephistoGenerator);