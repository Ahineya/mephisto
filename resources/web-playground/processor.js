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
console.log(this.elements);
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

let pluckTrigger = 0;
let frequency = 440;
let out = 0;
let ksBuffer = new Ringbuffer(110);
let justPlucked = 0;
let lastSample = 0;
let lastPluckState = 0;
let decayFactor = 0.995;
let oldFrequency = 440;
function resize_buf() {
let tmp = Rb.resize(ksBuffer, (sampleRate / frequency));
return 0;
}

let firstSample = 0;
let ksSample = 0;
let impulse = 0;
let newSample = 0;
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
            {name:'pluckTrigger',initial:0,type:0}, {name:'frequency',initial:440,type:1,min:20,max:2000,step:1}
        ];
    }

    process(inputs, outputs, parameters) {
        const output = outputs[0];

        const leftOutput = output[0];
        const rightOutput = output[1];

        for (let i = 0; i < this.scheduledParameterSetters.length; i++) {
            switch (this.scheduledParameterSetters[i].name) {
                case 'pluckTrigger': pluckTrigger = this.scheduledParameterSetters[i].value; break;
case 'frequency': frequency = this.scheduledParameterSetters[i].value; break;
            }
        }

        this.scheduledParameterSetters = [];

        /*console.trace('FIX ME, block');*/ {
justPlucked = ((1 - lastPluckState) * pluckTrigger);
lastPluckState = pluckTrigger;
let tmp = Std.if((1 - (frequency == oldFrequency ? 1 : 0)), resize_buf);
oldFrequency = frequency;
}



        for (let i = 0; i < leftOutput.length; i++) {
            // Advance each module
            firstSample = Rb.read(ksBuffer, 0);
ksSample = ((firstSample + lastSample) * 0.5);
impulse = ((justPlucked * 0.5) + ((justPlucked * Math.random()) * 0.5));
newSample = ((justPlucked * impulse) + ((1 - justPlucked) * ksSample));
newSample = (newSample * decayFactor);
tmp = Rb.push(ksBuffer, newSample);
lastSample = newSample;
justPlucked = 0;
out = newSample;

            leftOutput[i] = out;
rightOutput && (rightOutput[i] = out);



        }

        return true;
    }
}

console.log('SAMPLE RATE', sampleRate);

registerProcessor('mephisto-generator', MephistoGenerator);