/*
stdlib.insert("buf_new".to_string(), "new Ringbuffer".to_string());
stdlib.insert("buf_read".to_string(), "Ringbuffer.read".to_string());
stdlib.insert("buf_push".to_string(), "Ringbuffer.push".to_string());
stdlib.insert("buf_pop".to_string(), "Ringbuffer.pop".to_string());
stdlib.insert("buf_length".to_string(), "Ringbuffer.length".to_string());
stdlib.insert("buf_clear".to_string(), "Ringbuffer.clear".to_string());
stdlib.insert("buf_put".to_string(), "Ringbuffer.put".to_string());
*/

class Ringbuffer {
constructor(size) {
this.elements = new Float64Array(size);
console.log(this.elements);
}

push(element) {
for (let i = 0; i < this.elements.length - 1; i++) {
this.elements[i] = this.elements[i + 1];
}

this.elements[this.elements.length - 1] = element;
}

pop() {
const element = this.elements[0];

for (let i = 0; i < this.elements.length - 1; i++) {
this.elements[i] = this.elements[i + 1];
}

this.elements[this.elements.length - 1] = 0;

return element;
}

peek() {
return this.elements[this.elements.length - 1];
}

get(index) {
return this.elements[index] || 0;
}

set(index, value) {
this.elements[index] = value;
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
}
}



let delayTime = 0.5;
let feedback = 0.5;
let dryWet = 0.5;
let audioIn = 0;
let audioOut = 0;
let delayBuffer = new Ringbuffer(48000);
let delaySamples = 0;
let bufLen = 0;
let readIndex = 0;
let delayedSignal = 0;
let toPush = 0;
let tmp = 0;


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
            {name:'delayTime',initial:0.5,min:0,max:1,step:0.01,type:1}, {name:'feedback',initial:0.5,min:0,max:1,step:0.01,type:1}, {name:'dryWet',initial:0.5,min:0,max:1,step:0.01,type:1}
        ];
    }

    process(inputs, outputs, parameters) {
        const output = outputs[0];

        const leftOutput = output[0];
        const rightOutput = output[1];

        for (let i = 0; i < this.scheduledParameterSetters.length; i++) {
            switch (this.scheduledParameterSetters[i].name) {
                case 'delayTime': delayTime = this.scheduledParameterSetters[i].value; break;
case 'feedback': feedback = this.scheduledParameterSetters[i].value; break;
case 'dryWet': dryWet = this.scheduledParameterSetters[i].value; break;
            }
        }

        this.scheduledParameterSetters = [];

        

        for (let i = 0; i < leftOutput.length; ++i) {
            // Advance each module
            delaySamples = (delayTime * 48000);
bufLen = Rb.length(delayBuffer);
readIndex = (bufLen - delaySamples);
readIndex = Math.max(0, Math.min(readIndex, (bufLen - 1)));
delayedSignal = Rb.read(delayBuffer, readIndex);
toPush = (audioIn + (delayedSignal * feedback));
tmp = Rb.push(delayBuffer, toPush);
audioOut = ((audioIn * (1 - dryWet)) + (delayedSignal * dryWet));

            
        }

        return true;
    }
}

console.log('SAMPLE RATE', sampleRate);

registerProcessor('mephisto-generator', MephistoGenerator);