import {StoreSubject} from "@dgaa/store-subject";
import {audioContext, synthFacade} from "../audio-context.ts";
import {Parameter, SynthParametersPreset} from "../types/synthesizer.types.ts";



export const initPreset: SynthParametersPreset = {
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

    public preset = new StoreSubject<SynthParametersPreset>(initPreset);
    public onLoadedChanged = new StoreSubject(false);

    private firstLoad = true;

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

    public initSynth() {
        if (audioContext.state === "suspended" || this.firstLoad) {
            audioContext.resume();

            synthFacade.init();
            synthFacade.setParameter('globalgate', 1);

            synthStore.loadCurrentPreset();

            this.firstLoad = false;
        }
    }

    public loadPreset(preset: SynthParametersPreset) {
        Object.entries(preset.values as object).forEach(([prop, value]) => {
            synthFacade.setParameter(prop as Parameter, value)
        });

        this.preset.next(preset);
    }

    public loadCurrentPreset() {
        const currentPreset = this.preset.getValue();
        this.loadPreset(currentPreset);
    }

    public setSynthParameter(name: Parameter, value: number) {
        synthFacade.setParameter(name, value);

        this.preset.next({
            ...this.preset.getValue(),
            values: {
                ...this.preset.getValue().values,
                [name]: value
            }
        });
    }

    public exportPreset() {
        return this.preset.getValue();
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

        synthFacade.connect(output, input);
    }

    disconnect(output: string, input: string) {
        const outputIndex = this.getNumericOutput(output);
        const inputIndex = this.getNumericInput(input);

        if (outputIndex === -1 || inputIndex === -1) {
            return;
        }

        synthFacade.disconnect(output, input);
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
