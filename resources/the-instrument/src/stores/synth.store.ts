import {BehaviorSubject} from "rxjs";
// import {
//   ISynthPreset,
//   ISynthPresetDelayValues,
//   ISynthPresetDistortionValues,
//   ISynthPresetEnvelopeValues,
//   ISynthPresetFilterValues,
//   ISynthPresetLfoValues,
//   ISynthPresetMiscValues,
//   ISynthPresetOscillatorsValues,
//   ISynthPresetReverbValues
// } from "./synth.interface";
// import {deepCopy} from "../helpers/deep-copy.helper";
import {synth} from "../audio-context";
// import {ISynthNode} from "../synth-node.interface";
// import {copyToClipboard} from "../helpers/copy-to-clipboard";

export const initPreset: ISynthPreset = {
  id: 1,
  name: "Default preset",
  values: {
    oscillators: {
      osc1frequency: 220,
      osc1waveform: 0,
      osc2waveform: 0,
      osc2on: 0,
      noise: 0,
      osc2octave: 1,
      balance: 0.5,
      pw: 0,
      osc2fine: 0,
      osc2semi: 0,
    },
    filter: {
      lowPassCutoff: 22000,
      lowPassResonance: 0.5,
      highPassCutoff: 1,
      isHighPassOn: 0
    },
    envelope: {
      attack: 0,
      decay: 0,
      sustain: 1,
      release: 0,
      sendToOscFreq: 0,
      sendToFilterCutoff: 0,
      sendToBalance: 0,
    },
    distortion: {
      drive: 0,
    },
    reverb: {
      feedback: 10,
      mix: 0,
    },
    delay: {
      feedback: 0,
      duration: 0,
    },
    lfo: {
      frequency: 2,
      waveform: 0,
      sendToGain: 0,
      sendToOscFreq: 0,
      sendToFilterCutoff: 0,
      sendToPWM: 0,
      sendToBalance: 0,
    },
    misc: {
      gain: 0.5,
      hold: 0,
      retrigger: 0,
      octave: 3,
    }
  }
}

export const defaultPreset: ISynthPreset = {
  "id": 1,
  "name": "Default preset",
  "values": {
    "oscillators": {
      "osc1frequency": 220,
      "osc1waveform": 3,
      "osc2waveform": 2,
      "osc2on": 1,
      "noise": 0,
      "osc2octave": 2,
      "balance": 0.666666666666667,
      "pw": 0.4222222222222223,
      "osc2fine": 0,
      "osc2semi": 7
    },
    "filter": {
      "lowPassCutoff": 7219.263209875957,
      "lowPassResonance": 0.40744444444444333,
      "highPassCutoff": 1,
      "isHighPassOn": 0
    },
    "envelope": {
      "attack": 1.4700370370370373,
      "decay": 1.2407407407407405,
      "sustain": 0.8371999999999999,
      "release": 2.056144444444444,
      "sendToOscFreq": 0,
      "sendToFilterCutoff": 1,
      "sendToBalance": -0.15555555555555567
    },
    "distortion": {"drive": 0},
    "reverb": {"feedback": 40.33333333333333, "mix": 0.07407407407407407},
    "delay": {"feedback": 0.4370370370370371, "duration": 0.4222222222222223},
    "lfo": {
      "frequency": 1.9003362287214798,
      "waveform": 0,
      "sendToGain": 0,
      "sendToOscFreq": 0,
      "sendToFilterCutoff": 0.14814814814814814,
      "sendToPWM": 0.2814814814814815,
      "sendToBalance": 0.003703703703703704
    },
    "misc": {"gain": 0.5407407407407405, "hold": 0, "retrigger": 1, "octave": 1}
  }
};

class SynthStore {
  public onCurrentPresetChanged = new BehaviorSubject<ISynthPreset>(defaultPreset);

  public onLoadedChanged = new BehaviorSubject(false);

  constructor() {
    const savedPresetJSON = localStorage.getItem('currentPreset');

    if (savedPresetJSON) {
      this.loadPreset(JSON.parse(savedPresetJSON))
        .then(() => {
          this.onLoadedChanged.next(true);
        });
    } else {
      this.loadPreset(defaultPreset)
        .then(() => {
          this.onLoadedChanged.next(true);
        });
    }

    this.onCurrentPresetChanged.subscribe(preset => {
      localStorage.setItem('currentPreset', JSON.stringify(preset));
    });
  }

  public loadPreset(preset: ISynthPreset) {
    return synth.then(() => {
      console.log('loading preset');

      return new Promise((resolve) => {
        setTimeout(() => {
          Object.entries(preset.values.oscillators as object).forEach(([prop, value]) => this.changeOscillatorsValue((prop as (keyof ISynthPresetOscillatorsValues)), value as number));
          Object.entries(preset.values.misc as object).forEach(([prop, value]) => this.changeMiscValue((prop as (keyof ISynthPresetMiscValues)), value as number));
          Object.entries(preset.values.lfo as object).forEach(([prop, value]) => this.changeLfoValue((prop as (keyof ISynthPresetLfoValues)), value as number));
          Object.entries(preset.values.delay as object).forEach(([prop, value]) => this.changeDelayValue((prop as (keyof ISynthPresetDelayValues)), value as number));
          Object.entries(preset.values.reverb as object).forEach(([prop, value]) => this.changeReverbValue((prop as (keyof ISynthPresetReverbValues)), value as number));
          Object.entries(preset.values.distortion as object).forEach(([prop, value]) => this.changeDistortionValue((prop as (keyof ISynthPresetDistortionValues)), value as number));
          Object.entries(preset.values.envelope as object).forEach(([prop, value]) => this.changeEnvelopeValue((prop as (keyof ISynthPresetEnvelopeValues)), value as number));
          Object.entries(preset.values.filter as object).forEach(([prop, value]) => this.changeFilterValue((prop as (keyof ISynthPresetFilterValues)), value as number));
          resolve(true);
        }, 200);
      })

    })
  }

  public changeFilterValue(valueName: keyof ISynthPresetFilterValues, newValue: number) {
    const newPreset = deepCopy(this.onCurrentPresetChanged.getValue());
    newPreset.values.filter[valueName] = newValue;
    this.onCurrentPresetChanged.next(newPreset);

    switch (valueName) {
      case "lowPassCutoff":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthFilterFilterCutoff(newValue));
        return;
      case "highPassCutoff":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthFilterHpFilterCutoff(newValue));
        return;
      case "isHighPassOn":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthFilterLPF_Enable(newValue));
        return;
      case "lowPassResonance":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthFilterFilterResonance(newValue));
        return;
    }
  }

  public changeEnvelopeValue(valueName: keyof ISynthPresetEnvelopeValues, newValue: number) {
    const newPreset = deepCopy(this.onCurrentPresetChanged.getValue());
    newPreset.values.envelope[valueName] = newValue;
    this.onCurrentPresetChanged.next(newPreset);

    switch (valueName) {
      case "attack":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthEnvelopeADSRAdsrAttack(newValue));
        return;
      case "decay":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthEnvelopeADSRAdsrDecay(newValue));
        return;
      case "sustain":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthEnvelopeADSRAdsrSustain(newValue));
        return;
      case "release":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthEnvelopeADSRAdsrRelease(newValue));
        return;
      case "sendToFilterCutoff":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthEnvelopeSendAdsr_to_Filt_Cutoff(newValue));
        return;
      case "sendToOscFreq":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthEnvelopeSendAdsr_To_Osc_Freq(newValue));
        return;
      case "sendToBalance":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthEnvelopeSendAdsr_to_osc_balance(newValue));
        return;
    }
  }

  public changeDistortionValue(valueName: keyof ISynthPresetDistortionValues, newValue: number) {
    const newPreset = deepCopy(this.onCurrentPresetChanged.getValue());
    newPreset.values.distortion[valueName] = newValue;
    this.onCurrentPresetChanged.next(newPreset);

    switch (valueName) {
      case "drive":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthEffectsDistortionDrive(newValue));
        return;
    }
  }

  public changeReverbValue(valueName: keyof ISynthPresetReverbValues, newValue: number) {
    const newPreset = deepCopy(this.onCurrentPresetChanged.getValue());
    newPreset.values.reverb[valueName] = newValue;
    this.onCurrentPresetChanged.next(newPreset);

    switch (valueName) {
      case "feedback":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthEffectsReverbDepth(newValue));
        return;
      case "mix":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthEffectsReverbMix(newValue));
        return;
    }
  }

  public changeDelayValue(valueName: keyof ISynthPresetDelayValues, newValue: number) {
    const newPreset = deepCopy(this.onCurrentPresetChanged.getValue());
    newPreset.values.delay[valueName] = newValue;
    this.onCurrentPresetChanged.next(newPreset);

    switch (valueName) {
      case "feedback":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthEffectsEchoFeedback(newValue));
        return;
      case "duration":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthEffectsEchoDuration(newValue));
        return;
    }
  }

  public changeLfoValue(valueName: keyof ISynthPresetLfoValues, newValue: number) {
    const newPreset = deepCopy(this.onCurrentPresetChanged.getValue());
    newPreset.values.lfo[valueName] = newValue;
    this.onCurrentPresetChanged.next(newPreset);

    switch (valueName) {
      case "frequency":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthLFOLfoFrequency(newValue));
        return;
      case "waveform":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthLFOLfoWaveform(newValue));
        return;
      case "sendToOscFreq":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthLFOSendLfo_to_Osc_Freq(newValue));
        return;
      case "sendToFilterCutoff":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthLFOSendLfo_to_Filt_Cutoff(newValue));
        return;
      case "sendToGain":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthLFOSendLfo_to_Gain(newValue));
        return;
      case "sendToPWM":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthLFOSendLfo_to_PWM(newValue));
        return;
      case "sendToBalance":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthLFOSendLfo_to_OSC_Bal(newValue));
        return;
    }
  }

  public changeMiscValue(valueName: keyof ISynthPresetMiscValues, newValue: number) {
    const newPreset = deepCopy(this.onCurrentPresetChanged.getValue());
    newPreset.values.misc[valueName] = newValue;
    this.onCurrentPresetChanged.next(newPreset);

    if (valueName === "gain") {
      synth
        .then((s: ISynthNode) => {
          s.setSynth0x00SynthGain(newValue);
        });
    }
  }

  public changeOscillatorsValue(valueName: keyof ISynthPresetOscillatorsValues, newValue: number) {
    const newPreset = deepCopy(this.onCurrentPresetChanged.getValue());
    newPreset.values.oscillators[valueName] = newValue;
    this.onCurrentPresetChanged.next(newPreset);

    switch (valueName) {
      case "osc1waveform":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthOscillatorsOSC1Osc1Waveform(newValue));
        return;
      case "osc2waveform":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthOscillatorsOSC2Osc2Waveform(newValue));
        return;
      case "balance":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthOscillatorsOscBalance(newValue));
        return;
      case "osc2on":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthOscillatorsOSC2Osc2Disabled(Math.abs(newValue - 1)));
        return;
      case "noise":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthOscillatorsNoiseNoiseAmount(newValue));
        return;
      case "osc2fine":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthOscillatorsOSC2Osc2Detune(newValue));
        return;
      case "osc2octave":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthOscillatorsOSC2Osc2OctaveOffset(newValue));
        return;
      case "osc2semi":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthOscillatorsOSC2Osc2SemiOffset(newValue));
        return;
      case "pw":
        synth.then((s: ISynthNode) => s.setSynth0x00SynthOscillatorsOSC1Osc1pw(newValue));
        return;
    }
  }

  public exportPreset() {
    copyToClipboard(JSON.stringify(this.onCurrentPresetChanged.getValue()));
  }
}

export const synthStore = new SynthStore();
