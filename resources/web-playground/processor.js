let frequency = 110;
let gain = 0.2;
let wave = 0;
let phase = 0;
let increment = 0;
let out = 0;
let sine = 0;
let square = 0;
let saw = 0;


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
            {name:'frequency',initial:110}, {name:'gain',initial:0.2}, {name:'wave',initial:0,sine:0,square:1,saw:2}
        ];
    }

    process(inputs, outputs, parameters) {
        const output = outputs[0];

        const leftOutput = output[0];
        const rightOutput = output[1];

        for (let i = 0; i < this.scheduledParameterSetters.length; i++) {
            switch (this.scheduledParameterSetters[i].name) {
                case 'frequency': frequency = this.scheduledParameterSetters[i].value; break;
                case 'gain': gain = this.scheduledParameterSetters[i].value; break;
                case 'wave': wave = this.scheduledParameterSetters[i].value; break;
            }
        }

        this.scheduledParameterSetters = [];

        /*console.trace('FIX ME, block');*/ {
            increment = (frequency / sampleRate);
        }


        for (let i = 0; i < leftOutput.length; ++i) {
            // Advance each module
            phase = (increment + (phase - Math.floor((increment + phase))));
            sine = ((wave == 0 ? 1 : 0) * Math.sin(((phase * 2) * Math.PI)));
            square = ((wave == 1 ? 1 : 0) * (((phase < 0.5 ? 1 : 0) * 2) - 1));
            saw = ((wave == 2 ? 1 : 0) * ((phase * 2) - 1));
            out = (((sine + square) + saw) * gain);

            leftOutput[i] = out;
            rightOutput && (rightOutput[i] = out);



        }

        return true;
    }
}

console.log('SAMPLE RATE', sampleRate);

registerProcessor('mephisto-generator', MephistoGenerator);