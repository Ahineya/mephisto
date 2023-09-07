import {StoreSubject} from "@dgaa/store-subject";
import {synth} from "../audio-context.ts";



class SynthStore {
    public chart = new StoreSubject("");
    public inputs = new StoreSubject<string[]>([]);
    public outputs = new StoreSubject<string[]>([]);

    constructor() {}

    setChart(chart: string) {
        this.chart.next(chart);
    }

    setInputs(inputs: string[]) {
        this.inputs.next(inputs);
    }

    setOutputs(outputs: string[]) {
        this.outputs.next(outputs);
    }

    getNumericInput(name: string) {
        return this.inputs.getValue().findIndex((input) => input === name);
    }

    getNumericOutput(name: string) {
        return this.outputs.getValue().findIndex((output) => output === name);
    }

    connectLfoToFreqMod() {
        const lfo = this.getNumericOutput('LFO#out');
        const freqMod = this.getNumericInput('frequencyMod');

        if (lfo === -1 || freqMod === -1) {
            return;
        }

        console.log([lfo, freqMod]);

        synth.port.postMessage({
            command: 'addConnection',
            connection: [lfo, freqMod]
        });
    }

    disconnectLfoFromFreqMod() {
        const lfo = this.getNumericOutput('LFO#out');
        const freqMod = this.getNumericInput('frequencyMod');

        if (lfo === -1 || freqMod === -1) {
            return;
        }

        synth.port.postMessage({
            command: 'removeConnection',
            connection: [lfo, freqMod]
        });
    }
}

export const synthStore = new SynthStore();
