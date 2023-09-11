import {StoreSubject} from "@dgaa/store-subject";
import {synth} from "../audio-context.ts";
import {copyToClipboard} from "../helpers/copy-to-clipboard.ts";

const parameters = [
    '__OscVolume__osc1gain',
    '__OscVolume__osc2gain',
    '__OscVolume__osc3gain',
    '__OscVolume__noiseGain',
    '__ADSR__attackTime',
    '__ADSR__decayTime',
    '__ADSR__sustainLevel',
    '__ADSR__releaseTime',
    '__LowPass__cutoffFrequency',
    '__LowPass__resonance',
    '__Echo__delayTime',
    '__Echo__feedback',
    '__Echo__dryWet',
    '__Limiter__threshold',
    '__Limiter__recoveryRate',
    '__Freeverb__dryWet',
    '__Freeverb__roomSize',
    '__Freeverb__damp',
    '__Mix__balance',
    '__Attenuator__balance',
    'osc1waveform',
    'osc2waveform',
    'osc3waveform',
    'lfowaveform',
    'lfoFrequency',
    'osc2octaveoffset',
    'osc2semioffset',
    'osc2detune',
    'osc3octaveoffset',
    'osc3semioffset',
    'osc3detune',
    // 'trigger',
    // 'frequency',
    'frequencyModAmount',
    // 'globalgate'
    'UI_OCTAVE'
] as const;

type Parameter = typeof parameters[number];

type SynthPreset = {
    id: number;
    name: string;
    values: {
        [key in Parameter]: number;
    }
}

export const initPreset: SynthPreset = {
    id: 1,
    name: "Default preset",
    values: {
        '__OscVolume__osc1gain': 0.3,
        '__OscVolume__osc2gain': 0,
        '__OscVolume__osc3gain': 0,
        '__OscVolume__noiseGain': 0,

        '__ADSR__attackTime': 0.01,
        '__ADSR__decayTime': 0.01,
        '__ADSR__sustainLevel': 1,
        '__ADSR__releaseTime': 0.01,

        '__LowPass__cutoffFrequency': 20000,
        '__LowPass__resonance': 0,

        '__Echo__delayTime': 0.5,
        '__Echo__feedback': 0.5,
        '__Echo__dryWet': 0,

        '__Limiter__threshold': 0.8,
        '__Limiter__recoveryRate': 0.0001,

        '__Freeverb__dryWet': 0.5,
        '__Freeverb__roomSize': 0.7,
        '__Freeverb__damp': 0.5,

        '__Mix__balance': 0, // -1 to 1
        '__Attenuator__balance': 0, // -1 to 1

        'osc1waveform': 0,
        'osc2waveform': 0,
        'osc3waveform': 0,

        'lfowaveform': 0,
        'lfoFrequency': 1, // 1 hz

        'osc2octaveoffset': 1, // No offset
        'osc2semioffset': 0,
        'osc2detune': 0,

        'osc3octaveoffset': 1,
        'osc3semioffset': 0,
        'osc3detune': 0,

        // 'trigger': 0,
        // 'frequency': 0,

        'frequencyModAmount': 0,
        'UI_OCTAVE': 2,

        // 'globalgate': 0
    }
}

class SynthStore {
    public chart = new StoreSubject("");
    public inputs = new StoreSubject<string[]>([]);
    public outputs = new StoreSubject<string[]>([]);

    public preset = new StoreSubject<SynthPreset>(initPreset);
    public onLoadedChanged = new StoreSubject(false);

    constructor() {
        const savedPresetJSON = localStorage.getItem('currentPreset');

        if (savedPresetJSON) {
            console.log('Loading preset from local storage', savedPresetJSON)
            this.loadPreset(JSON.parse(savedPresetJSON))
            this.onLoadedChanged.next(true);
        } else {
            console.log('Loading init preset')
            this.loadPreset(initPreset)
            this.onLoadedChanged.next(true);
        }

        this.preset.subscribe(preset => {
            console.log('Saving preset to local storage', preset)
            localStorage.setItem('currentPreset', JSON.stringify(preset));
        });
    }

    public loadPreset(preset: SynthPreset) {
        Object.entries(preset.values as object).forEach(([prop, value]) => {
            synth.port.postMessage({
                command: 'setParameter',
                setter: {
                    name: prop,
                    value
                }
            });
        });

        this.preset.next(preset);
    }

    public loadCurrentPreset() {
        const currentPreset = this.preset.getValue();
        this.loadPreset(currentPreset);
    }

    public setSynthParameter(name: Parameter, value: number) {
        synth.port.postMessage({
            command: 'setParameter',
            setter: {
                name,
                value
            }
        });

        this.preset.next({
            ...this.preset.getValue(),
            values: {
                ...this.preset.getValue().values,
                [name]: value
            }
        });
    }

    public exportPreset() {
        copyToClipboard(JSON.stringify(this.preset.getValue()));
    }

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

    setInternalParameter(name: Parameter, value: number) {
        console.log('Setting internal parameter', name, value)

        this.preset.next({
            ...this.preset.getValue(),
            values: {
                ...this.preset.getValue().values,
                [name]: value
            }
        });
    }

    setIsLoaded(isLoaded: boolean) {
        this.onLoadedChanged.next(isLoaded);
    }
}

export const synthStore = new SynthStore();
