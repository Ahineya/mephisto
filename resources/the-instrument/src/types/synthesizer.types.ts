export type Wire = {
    uuid: string;
    from: WireConnectionPoint;
    to: WireConnectionPoint;
    color: string;

    connected?: boolean;
}

export type WireConnectionPoint = {
    position: {
        x: number;
        y: number;
    };
    type: 'cursor' | 'input' | 'output';
    controlId: string | null;
}

export const SynthParameterNames = [
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

export type Parameter = typeof SynthParameterNames[number];

export type SynthParametersPreset = {
    id: number;
    name: string;
    values: {
        [key in Parameter]: number;
    }
}

export type SynthPreset = {parameters: SynthParametersPreset, wires: Wire[]};