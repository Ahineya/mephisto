import {SynthPreset} from "../../../../types/synthesizer.types.ts";

export const DETUNED_SAW_PRESET: SynthPreset = {
    "parameters": {
        "id": 1, "name": "Detuned Saw", "values": {
            "__OscVolume__osc1gain": 0.3,
            "__OscVolume__osc2gain": 0.3148148148148148,
            "__OscVolume__osc3gain": 0.32222222222222224,
            "__OscVolume__noiseGain": 0,
            "__ADSR__attackTime": 0.001251970773221269,
            "__ADSR__decayTime": 0.00203766224126608,
            "__ADSR__sustainLevel": 0.837037037037037,
            "__ADSR__releaseTime": 0.3027024355489905,
            "__LowPass__cutoffFrequency": 7282.094054278386,
            "__LowPass__resonance": 0.5320000000000003,
            "__Echo__delayTime": 0.07777777777777778,
            "__Echo__feedback": 0.5,
            "__Echo__dryWet": 0.25925925925925924,
            "__Limiter__threshold": 0.8,
            "__Limiter__recoveryRate": 0.0001,
            "__Freeverb__dryWet": 0.5,
            "__Freeverb__roomSize": 0.579,
            "__Freeverb__damp": 0.5,
            "__Mix__balance": 0.9333333333333333,
            "__Attenuator__balance": 0.014814814814814836,
            "osc1waveform": 2,
            "osc2waveform": 2,
            "osc3waveform": 3,
            "lfowaveform": 0,
            "lfoFrequency": 0.19825433114866736,
            "osc2octaveoffset": 1,
            "osc2semioffset": 0,
            "osc2detune": 2.7755575615628914e-17,
            "osc3octaveoffset": 0.25,
            "osc3semioffset": 0,
            "osc3detune": 1.0269562977782698e-15,
            "frequencyModAmount": 0.051851851851851816,
            "UI_OCTAVE": 2
        }
    },
    "wires": [{
        "uuid": "wire_0.44134625506872327",
        "color": "#face8D",
        "from": {"position": {"x": 693.0703125, "y": 194.5}, "type": "output", "controlId": "LFO#out"},
        "to": {"position": {"x": 643.0703125, "y": 94.5}, "type": "input", "controlId": "Mix#mix1"},
        "connected": true
    }, {
        "uuid": "wire_0.5813066549566026",
        "color": "#61461b",
        "from": {"position": {"x": 743.0703125, "y": 94.5}, "type": "output", "controlId": "Mix#out"},
        "to": {"position": {"x": 743.0703125, "y": 144.5}, "type": "input", "controlId": "osc2detuneMod"},
        "connected": true
    }, {
        "uuid": "wire_0.9756071706897571",
        "color": "#face8D",
        "from": {"position": {"x": 743.0703125, "y": 194.5}, "type": "output", "controlId": "ADSR#curve"},
        "to": {"position": {"x": 793.0703125, "y": 94.5}, "type": "input", "controlId": "LowPass#cutoffMod"},
        "connected": true
    }]
};

export const KICK_PRESET: SynthPreset = {
    "parameters": {
        "id": 1,
        "name": "Kick",
        "values": {
            "__OscVolume__osc1gain": 1,
            "__OscVolume__osc2gain": 0,
            "__OscVolume__osc3gain": 0,
            "__OscVolume__noiseGain": 0.17777777777777778,
            "__ADSR__attackTime": 0.0005056079712805174,
            "__ADSR__decayTime": 0.23921833985971552,
            "__ADSR__sustainLevel": 0,
            "__ADSR__releaseTime": 0.0003007522025923804,
            "__LowPass__cutoffFrequency": 10225.442505715393,
            "__LowPass__resonance": 0,
            "__Echo__delayTime": 0.5,
            "__Echo__feedback": 0.5,
            "__Echo__dryWet": 0,
            "__Limiter__threshold": 0.8,
            "__Limiter__recoveryRate": 0.0001,
            "__Freeverb__dryWet": 0.25555555555555554,
            "__Freeverb__roomSize": 0.7,
            "__Freeverb__damp": 0.5,
            "__Mix__balance": 0,
            "__Attenuator__balance": 0.10370370370370385,
            "osc1waveform": 0,
            "osc2waveform": 0,
            "osc3waveform": 0,
            "lfowaveform": 0,
            "lfoFrequency": 1,
            "osc2octaveoffset": 1,
            "osc2semioffset": 0,
            "osc2detune": 0,
            "osc3octaveoffset": 1,
            "osc3semioffset": 0,
            "osc3detune": 0,
            "frequencyModAmount": 0.31111111111111245,
            "UI_OCTAVE": 1
        }
    },
    "wires": [{
        "uuid": "wire_0.4424437670090744",
        "color": "#face8D",
        "from": {"position": {"x": 743.0703125, "y": 194.5}, "type": "output", "controlId": "ADSR#curve"},
        "to": {"position": {"x": 643.0703125, "y": 144.5}, "type": "input", "controlId": "frequencyMod"},
        "connected": true
    }, {
        "uuid": "wire_0.30448791128721897",
        "color": "#face8D",
        "from": {"position": {"x": 743.0703125, "y": 194.5}, "type": "output", "controlId": "ADSR#curve"},
        "to": {"position": {"x": 643.0703125, "y": 44.5}, "type": "input", "controlId": "Attenuator#inp"},
        "connected": true
    }, {
        "uuid": "wire_0.21057390492836703",
        "color": "#face8D",
        "from": {"position": {"x": 693.0703125, "y": 44.5}, "type": "output", "controlId": "Attenuator#out"},
        "to": {"position": {"x": 793.0703125, "y": 144.5}, "type": "input", "controlId": "noiseGainMod"},
        "connected": true
    }]
}

export const WOODEN_FLUTE_PRESET: SynthPreset = {
    "parameters": {
        "id": 1,
        "name": "Wooden Flute",
        "values": {
            "__OscVolume__osc1gain": 0.24814814814814815,
            "__OscVolume__osc2gain": 0,
            "__OscVolume__osc3gain": 0,
            "__OscVolume__noiseGain": 0,
            "__ADSR__attackTime": 0.20444564321699177,
            "__ADSR__decayTime": 0.01,
            "__ADSR__sustainLevel": 1,
            "__ADSR__releaseTime": 0.01,
            "__LowPass__cutoffFrequency": 21999.99999999999,
            "__LowPass__resonance": 0,
            "__Echo__delayTime": 0.5,
            "__Echo__feedback": 0.5,
            "__Echo__dryWet": 0,
            "__Limiter__threshold": 0.8,
            "__Limiter__recoveryRate": 0.0001,
            "__Freeverb__dryWet": 0.13333333333333333,
            "__Freeverb__roomSize": 0.7,
            "__Freeverb__damp": 0.5,
            "__Mix__balance": 0,
            "__Attenuator__balance": -0.05925925925925968,
            "osc1waveform": 0,
            "osc2waveform": 0,
            "osc3waveform": 0,
            "lfowaveform": 0,
            "lfoFrequency": 2.415897270822838,
            "osc2octaveoffset": 1,
            "osc2semioffset": 0,
            "osc2detune": 0,
            "osc3octaveoffset": 1,
            "osc3semioffset": 0,
            "osc3detune": 0,
            "frequencyModAmount": 0.09629629629629632,
            "UI_OCTAVE": 4
        }
    },
    "wires": [{
        "uuid": "wire_0.1189422169833283",
        "color": "#61461b",
        "from": {"position": {"x": 693.0703125, "y": 194.5}, "type": "output", "controlId": "LFO#out"},
        "to": {"position": {"x": 643.0703125, "y": 44.5}, "type": "input", "controlId": "Attenuator#inp"},
        "connected": true
    }, {
        "uuid": "wire_0.8714779561922548",
        "color": "#face8D",
        "from": {"position": {"x": 693.0703125, "y": 44.5}, "type": "output", "controlId": "Attenuator#out"},
        "to": {"position": {"x": 643.0703125, "y": 144.5}, "type": "input", "controlId": "frequencyMod"},
        "connected": true
    }]
}

export const NASTY_BASS_PRESET: SynthPreset = {
    "parameters": {
        "id": 1,
        "name": "Nasty Bass",
        "values": {
            "__OscVolume__osc1gain": 0.6037037037037039,
            "__OscVolume__osc2gain": 0.5481481481481479,
            "__OscVolume__osc3gain": 0.16666666666666666,
            "__OscVolume__noiseGain": 0,
            "__ADSR__attackTime": 0.01,
            "__ADSR__decayTime": 0.15065851690200413,
            "__ADSR__sustainLevel": 0.7592592592592593,
            "__ADSR__releaseTime": 0.21111053807563715,
            "__LowPass__cutoffFrequency": 406.51209876535444,
            "__LowPass__resonance": 0.01,
            "__Echo__delayTime": 0,
            "__Echo__feedback": 0.7185185185185186,
            "__Echo__dryWet": 0,
            "__Limiter__threshold": 0.8,
            "__Limiter__recoveryRate": 0.0001,
            "__Freeverb__dryWet": 0.15185185185185185,
            "__Freeverb__roomSize": 0.7,
            "__Freeverb__damp": 0.5,
            "__Mix__balance": 0,
            "__Attenuator__balance": 0.014814814814814836,
            "osc1waveform": 1,
            "osc2waveform": 2,
            "osc3waveform": 1,
            "lfowaveform": 1,
            "lfoFrequency": 12.765358248557588,
            "osc2octaveoffset": 2,
            "osc2semioffset": 0,
            "osc2detune": 0.004444444444444445,
            "osc3octaveoffset": 1,
            "osc3semioffset": 0,
            "osc3detune": -0.0022222222222222227,
            "frequencyModAmount": -0.08148148148148149,
            "UI_OCTAVE": 1
        }
    },
    "wires": [{
        "uuid": "wire_0.2665118705855889",
        "color": "#61461b",
        "from": {"position": {"x": 743.0703125, "y": 194.5}, "type": "output", "controlId": "ADSR#curve"},
        "to": {"position": {"x": 793.0703125, "y": 94.5}, "type": "input", "controlId": "LowPass#cutoffMod"},
        "connected": true
    }, {
        "uuid": "wire_0.7131603295317741",
        "color": "#face8D",
        "from": {"position": {"x": 693.0703125, "y": 194.5}, "type": "output", "controlId": "LFO#out"},
        "to": {"position": {"x": 643.0703125, "y": 44.5}, "type": "input", "controlId": "Attenuator#inp"},
        "connected": true
    }, {
        "uuid": "wire_0.09723521109922229",
        "color": "#61461b",
        "from": {"position": {"x": 693.0703125, "y": 44.5}, "type": "output", "controlId": "Attenuator#out"},
        "to": {"position": {"x": 643.0703125, "y": 144.5}, "type": "input", "controlId": "frequencyMod"},
        "connected": true
    }]
}

export const SQUARE_ORGAN_PRESET: SynthPreset = {
    "parameters": {
        "id": 1,
        "name": "Square Rotary Organ",
        "values": {
            "__OscVolume__osc1gain": 0.2777777777777777,
            "__OscVolume__osc2gain": 0.16666666666666666,
            "__OscVolume__osc3gain": 0.2740740740740741,
            "__OscVolume__noiseGain": 0,
            "__ADSR__attackTime": 0.010503036011150628,
            "__ADSR__decayTime": 0.01,
            "__ADSR__sustainLevel": 1,
            "__ADSR__releaseTime": 0.0029240922326728815,
            "__LowPass__cutoffFrequency": 8312.654320986745,
            "__LowPass__resonance": 0.01,
            "__Echo__delayTime": 0.5,
            "__Echo__feedback": 0.5,
            "__Echo__dryWet": 0,
            "__Limiter__threshold": 0.8,
            "__Limiter__recoveryRate": 0.0001,
            "__Freeverb__dryWet": 0.3148148148148148,
            "__Freeverb__roomSize": 0.34099999999999997,
            "__Freeverb__damp": 0.44814814814814813,
            "__Mix__balance": -0.40740740740740744,
            "__Attenuator__balance": 0.14074074074074017,
            "osc1waveform": 1,
            "osc2waveform": 1,
            "osc3waveform": 1,
            "lfowaveform": 2,
            "lfoFrequency": 5.634540466391461,
            "osc2octaveoffset": 1,
            "osc2semioffset": 5,
            "osc2detune": 2.7755575615628914e-17,
            "osc3octaveoffset": 2,
            "osc3semioffset": -3,
            "osc3detune": 0,
            "frequencyModAmount": 0.014814814814815058,
            "UI_OCTAVE": 2
        }
    },
    "wires": [{
        "uuid": "wire_0.624146124431417",
        "color": "#face8D",
        "from": {"position": {"x": 693.0703125, "y": 44.5}, "type": "output", "controlId": "Attenuator#out"},
        "to": {"position": {"x": 793.0703125, "y": 94.5}, "type": "input", "controlId": "LowPass#cutoffMod"},
        "connected": true
    }, {
        "uuid": "wire_0.4275798362798271",
        "color": "#61461b",
        "from": {"position": {"x": 793.0703125, "y": 194.5}, "type": "output", "controlId": "Noise#out"},
        "to": {"position": {"x": 743.0703125, "y": 44.5}, "type": "input", "controlId": "Snh#inp"},
        "connected": true
    }, {
        "uuid": "wire_0.23169796962848843",
        "color": "#face8D",
        "from": {"position": {"x": 693.0703125, "y": 194.5}, "type": "output", "controlId": "LFO#out"},
        "to": {"position": {"x": 643.0703125, "y": 44.5}, "type": "input", "controlId": "Attenuator#inp"},
        "connected": true
    }, {
        "uuid": "wire_0.33721617853268326",
        "color": "#61461b",
        "from": {"position": {"x": 743.0703125, "y": 94.5}, "type": "output", "controlId": "Mix#out"},
        "to": {"position": {"x": 643.0703125, "y": 144.5}, "type": "input", "controlId": "frequencyMod"},
        "connected": true
    }, {
        "uuid": "wire_0.28195271242996833",
        "color": "#61461b",
        "from": {"position": {"x": 793.0703125, "y": 44.5}, "type": "output", "controlId": "Snh#out"},
        "to": {"position": {"x": 693.0703125, "y": 94.5}, "type": "input", "controlId": "Mix#mix2"},
        "connected": true
    }, {
        "uuid": "wire_0.2832615170807802",
        "color": "#61461b",
        "from": {"position": {"x": 743.0703125, "y": 194.5}, "type": "output", "controlId": "ADSR#curve"},
        "to": {"position": {"x": 643.0703125, "y": 94.5}, "type": "input", "controlId": "Mix#mix1"},
        "connected": true
    }]
}

export const SUPER_SAW_PRESET: SynthPreset = {
    "parameters": {
        "id": 1,
        "name": "Super Saw",
        "values": {
            "__OscVolume__osc1gain": 0.3,
            "__OscVolume__osc2gain": 0.25925925925925924,
            "__OscVolume__osc3gain": 0.2629629629629629,
            "__OscVolume__noiseGain": 0,
            "__ADSR__attackTime": 0.01,
            "__ADSR__decayTime": 0.01,
            "__ADSR__sustainLevel": 1,
            "__ADSR__releaseTime": 0.01,
            "__LowPass__cutoffFrequency": 20000,
            "__LowPass__resonance": 0,
            "__Echo__delayTime": 0.5,
            "__Echo__feedback": 0.5,
            "__Echo__dryWet": 0,
            "__Limiter__threshold": 0.8,
            "__Limiter__recoveryRate": 0.0001,
            "__Freeverb__dryWet": 0.15925925925925927,
            "__Freeverb__roomSize": 0.7,
            "__Freeverb__damp": 0.5,
            "__Mix__balance": 0,
            "__Attenuator__balance": 0,
            "osc1waveform": 2,
            "osc2waveform": 2,
            "osc3waveform": 2,
            "lfowaveform": 0,
            "lfoFrequency": 1,
            "osc2octaveoffset": 1,
            "osc2semioffset": 0,
            "osc2detune": 0.005185185185185182,
            "osc3octaveoffset": 1,
            "osc3semioffset": 0,
            "osc3detune": -0.005185185185185182,
            "frequencyModAmount": 0,
            "UI_OCTAVE": 2
        }
    }, "wires": []
}

export const FX_FM_PRESET: SynthPreset = {
    "parameters": {
        "id": 1,
        "name": "Frequency Modulation FX",
        "values": {
            "__OscVolume__osc1gain": 0,
            "__OscVolume__osc2gain": 0.7222222222222222,
            "__OscVolume__osc3gain": 0,
            "__OscVolume__noiseGain": 0,
            "__ADSR__attackTime": 0.01,
            "__ADSR__decayTime": 0.01,
            "__ADSR__sustainLevel": 1,
            "__ADSR__releaseTime": 0.6534469451436454,
            "__LowPass__cutoffFrequency": 9728.055865853596,
            "__LowPass__resonance": 0,
            "__Echo__delayTime": 0.5,
            "__Echo__feedback": 0.5,
            "__Echo__dryWet": 0,
            "__Limiter__threshold": 0.8,
            "__Limiter__recoveryRate": 0.0001,
            "__Freeverb__dryWet": 0,
            "__Freeverb__roomSize": 0.7,
            "__Freeverb__damp": 0.5,
            "__Mix__balance": -0.45185185185185184,
            "__Attenuator__balance": 0.7925925925925927,
            "osc1waveform": 0,
            "osc2waveform": 0,
            "osc3waveform": 0,
            "lfowaveform": 0,
            "lfoFrequency": 0.4460458263475756,
            "osc2octaveoffset": 1,
            "osc2semioffset": 0,
            "osc2detune": 0,
            "osc3octaveoffset": 2,
            "osc3semioffset": 0,
            "osc3detune": 1.3877787807814457e-16,
            "frequencyModAmount": 0,
            "UI_OCTAVE": 3
        }
    },
    "wires": [{
        "uuid": "wire_0.006409553056723194",
        "color": "#face8D",
        "from": {"position": {"x": 693.0703125, "y": 44.5}, "type": "output", "controlId": "Attenuator#out"},
        "to": {"position": {"x": 643.0703125, "y": 94.5}, "type": "input", "controlId": "Mix#mix1"},
        "connected": true
    }, {
        "uuid": "wire_0.36859797359292745",
        "color": "#61461b",
        "from": {"position": {"x": 643.0703125, "y": 194.5}, "type": "output", "controlId": "Osc3#out"},
        "to": {"position": {"x": 643.0703125, "y": 44.5}, "type": "input", "controlId": "Attenuator#inp"},
        "connected": true
    }, {
        "uuid": "wire_0.5375741160540026",
        "color": "#face8D",
        "from": {"position": {"x": 743.0703125, "y": 94.5}, "type": "output", "controlId": "Mix#out"},
        "to": {"position": {"x": 743.0703125, "y": 144.5}, "type": "input", "controlId": "osc2detuneMod"},
        "connected": true
    }, {
        "uuid": "wire_0.7607575283305272",
        "color": "#face8D",
        "from": {"position": {"x": 743.0703125, "y": 194.5}, "type": "output", "controlId": "ADSR#curve"},
        "to": {"position": {"x": 693.0703125, "y": 144.5}, "type": "input", "controlId": "osc1gainMod"},
        "connected": true
    }, {
        "uuid": "wire_0.690620551477747",
        "color": "#61461b",
        "from": {"position": {"x": 743.0703125, "y": 94.5}, "type": "output", "controlId": "Mix#out"},
        "to": {"position": {"x": 693.0703125, "y": 94.5}, "type": "input", "controlId": "Mix#mix2"},
        "connected": true
    }]
}

export const FX_CRASH_PRESET: SynthPreset = {
    "parameters": {
        "id": 1,
        "name": "FX Crash",
        "values": {
            "__OscVolume__osc1gain": 0,
            "__OscVolume__osc2gain": 0.1,
            "__OscVolume__osc3gain": 0,
            "__OscVolume__noiseGain": 0.40740740740740744,
            "__ADSR__attackTime": 0.01,
            "__ADSR__decayTime": 9.999999999999851e-7,
            "__ADSR__sustainLevel": 1,
            "__ADSR__releaseTime": 0.4343533066971494,
            "__LowPass__cutoffFrequency": 5770.839014479802,
            "__LowPass__resonance": 0.5172222222222225,
            "__Echo__delayTime": 0.5,
            "__Echo__feedback": 0.5,
            "__Echo__dryWet": 0,
            "__Limiter__threshold": 0.8,
            "__Limiter__recoveryRate": 0.0001,
            "__Freeverb__dryWet": 0.2518518518518518,
            "__Freeverb__roomSize": 0.7,
            "__Freeverb__damp": 0.5,
            "__Mix__balance": -0.45185185185185184,
            "__Attenuator__balance": 0.7925925925925927,
            "osc1waveform": 0,
            "osc2waveform": 3,
            "osc3waveform": 0,
            "lfowaveform": 0,
            "lfoFrequency": 0.4460458263475756,
            "osc2octaveoffset": 1,
            "osc2semioffset": 0,
            "osc2detune": 0.0007407407407407085,
            "osc3octaveoffset": 2,
            "osc3semioffset": 0,
            "osc3detune": 0.005925925925925973,
            "frequencyModAmount": 0,
            "UI_OCTAVE": 4
        }
    },
    "wires": [{
        "uuid": "wire_0.006409553056723194",
        "color": "#face8D",
        "from": {"position": {"x": 693.0703125, "y": 44.5}, "type": "output", "controlId": "Attenuator#out"},
        "to": {"position": {"x": 643.0703125, "y": 94.5}, "type": "input", "controlId": "Mix#mix1"},
        "connected": true
    }, {
        "uuid": "wire_0.36859797359292745",
        "color": "#61461b",
        "from": {"position": {"x": 643.0703125, "y": 194.5}, "type": "output", "controlId": "Osc3#out"},
        "to": {"position": {"x": 643.0703125, "y": 44.5}, "type": "input", "controlId": "Attenuator#inp"},
        "connected": true
    }, {
        "uuid": "wire_0.5375741160540026",
        "color": "#face8D",
        "from": {"position": {"x": 743.0703125, "y": 94.5}, "type": "output", "controlId": "Mix#out"},
        "to": {"position": {"x": 743.0703125, "y": 144.5}, "type": "input", "controlId": "osc2detuneMod"},
        "connected": true
    }, {
        "uuid": "wire_0.7607575283305272",
        "color": "#face8D",
        "from": {"position": {"x": 743.0703125, "y": 194.5}, "type": "output", "controlId": "ADSR#curve"},
        "to": {"position": {"x": 693.0703125, "y": 144.5}, "type": "input", "controlId": "osc1gainMod"},
        "connected": true
    }, {
        "uuid": "wire_0.690620551477747",
        "color": "#61461b",
        "from": {"position": {"x": 743.0703125, "y": 94.5}, "type": "output", "controlId": "Mix#out"},
        "to": {"position": {"x": 693.0703125, "y": 94.5}, "type": "input", "controlId": "Mix#mix2"},
        "connected": true
    }, {
        "uuid": "wire_0.4339538948154338",
        "color": "#face8D",
        "from": {"position": {"x": 743.0703125, "y": 194.5}, "type": "output", "controlId": "ADSR#curve"},
        "to": {"position": {"x": 793.0703125, "y": 94.5}, "type": "input", "controlId": "LowPass#cutoffMod"},
        "connected": true
    }, {
        "uuid": "wire_0.6642790389401443",
        "color": "#61461b",
        "from": {"position": {"x": 743.0703125, "y": 194.5}, "type": "output", "controlId": "ADSR#curve"},
        "to": {"position": {"x": 793.0703125, "y": 144.5}, "type": "input", "controlId": "noiseGainMod"},
        "connected": true
    }, {
        "uuid": "wire_0.6436074235730067",
        "color": "#face8D",
        "from": {"position": {"x": 793.0703125, "y": 194.5}, "type": "output", "controlId": "Noise#out"},
        "to": {"position": {"x": 643.0703125, "y": 144.5}, "type": "input", "controlId": "frequencyMod"},
        "connected": true
    }]
}