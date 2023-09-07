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

    connect(output: string, input: string) {
        const outputIndex = this.getNumericOutput(output);
        const inputIndex = this.getNumericInput(input);

        if (outputIndex === -1 || inputIndex === -1) {
            return;
        }

        synth.port.postMessage({
            command: 'addConnection',
            connection: [outputIndex, inputIndex]
        });
    }

    disconnect(output: string, input: string) {
        const outputIndex = this.getNumericOutput(output);
        const inputIndex = this.getNumericInput(input);

        if (outputIndex === -1 || inputIndex === -1) {
            return;
        }

        synth.port.postMessage({
            command: 'removeConnection',
            connection: [outputIndex, inputIndex]
        });
    }
}

export const synthStore = new SynthStore();
