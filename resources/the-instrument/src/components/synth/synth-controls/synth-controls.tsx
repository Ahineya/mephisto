import "./synth-controls.scss";
import {Wires} from "./wires/wires.tsx";
import {SynthPanel} from "./synth-panel/synth-panel.tsx";
import {PatchPoint} from "./patch-point/patch-point.tsx";
import {KnobFree} from "./knobs/knob-free.tsx";
import {KnobSize} from "./knobs/knob.interface.ts";
import {synthStore} from "../../../stores/synth.store.ts";
import {useStoreSubscribe} from "@dgaa/use-store-subscribe";
import {KnobPredefined} from "./knobs/knob-predefined.tsx";
import {closestIndex} from "../../../helpers/closest.ts";
import {SynthMenu} from "./synth-menu/synth-menu.tsx";

enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
}

export const SynthControls = () => {
    const synthParams = useStoreSubscribe(synthStore.preset);

    return <div className="synth-controls-container">
        <SynthMenu />
        <div className="synth-controls">
            <SynthPanel left={610} top={12} width={230} height={320} caption="Ainanenane">

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 20,
                        y: 20
                    }}
                    controlId={"Attenuator#inp"}
                    label="ATTEN"
                />
                <PatchPoint
                    type={"output"}
                    position={{
                        x: 20 + 50,
                        y: 20
                    }}
                    controlId={"Attenuator#out"}
                    label="ATTEN"
                />

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 20 + 100,
                        y: 20
                    }}
                    controlId={"Snh#inp"}
                    label="S&H"
                />
                <PatchPoint
                    type={"output"}
                    position={{
                        x: 20 + 150,
                        y: 20
                    }}
                    controlId={"Snh#out"}
                    label="S&H"
                />

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 20,
                        y: 20 + 50,
                    }}
                    controlId={"Mix#mix1"}
                    label="MIX1"
                />

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 20 + 50,
                        y: 20 + 50,
                    }}
                    controlId={"Mix#mix2"}
                    label="MIX2"
                />

                <PatchPoint
                    type={"output"}
                    position={{
                        x: 120,
                        y: 20 + 50,
                    }}
                    controlId={"Mix#out"}
                    label="MIX"
                />

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 20 + 150,
                        y: 20 + 50
                    }}
                    controlId={"LowPass#cutoffMod"}
                    label="LPF"
                />

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 20,
                        y: 20 + 100
                    }}
                    controlId={"frequencyMod"}
                    label="FREQ"
                />

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 20 + 50,
                        y: 20 + 100
                    }}
                    controlId={"osc1gainMod"}
                    label="OSC2 GAIN"
                />

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 120,
                        y: 20 + 100
                    }}
                    controlId={"osc2detuneMod"}
                    label="OSC2 DTN"
                />

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 120 + 50,
                        y: 20 + 100
                    }}
                    controlId={"noiseGainMod"}
                    label="NOISE GAIN"
                />

                <PatchPoint
                    type={"output"}
                    position={{
                        x: 20,
                        y: 20 + 150
                    }}
                    controlId={"Osc3#out"}
                    label="OSC3"
                />

                <PatchPoint
                    type={"output"}
                    position={{
                        x: 20 + 50,
                        y: 20 + 150
                    }}
                    controlId={"LFO#out"}
                    label="LFO"
                />
                <PatchPoint
                    type={"output"}
                    position={{
                        x: 120,
                        y: 20 + 150
                    }}
                    controlId={"ADSR#curve"}
                    label="ADSR"
                />

                <PatchPoint
                    type={"output"}
                    position={{
                        x: 20 + 150,
                        y: 20 + 150,
                    }}
                    controlId={"Noise#out"}
                    label="NOISE"
                />

            </SynthPanel>

            <KnobFree id="freq-mod"
                      caption="Freq Mod"
                      onValueChanged={(v) => {
                          synthStore.setSynthParameter("frequencyModAmount", v);
                      }}
                      value={synthParams.values.frequencyModAmount}
                      from={-1}
                      to={1}
                      filled={false}
                      displayValue={synthParams.values.frequencyModAmount.toFixed(2)}
                      size={KnobSize.SMALL}
                      position={{
                          x: 120,
                          y: 30
                      }}
            />

            <KnobFree id="attenuator-balance"
                      caption="Attenuator"
                      onValueChanged={(v) => {
                          synthStore.setSynthParameter("__Attenuator__balance", v);
                      }}
                      value={synthParams.values.__Attenuator__balance}
                      from={-1}
                      to={1}
                      filled={false}
                      displayValue={synthParams.values.__Attenuator__balance.toFixed(2)}
                      size={KnobSize.SMALL}
                      position={{
                          x: 650,
                          y: 250
                      }}
            />

            <KnobFree id="mixer-balance"
                      caption="Mix"
                      onValueChanged={(v) => {
                          synthStore.setSynthParameter("__Mix__balance", v);
                      }}
                      value={synthParams.values.__Mix__balance}
                      from={-1}
                      to={1}
                      filled={false}
                      displayValue={synthParams.values.__Mix__balance.toFixed(2)}
                      size={KnobSize.SMALL}
                      position={{
                          x: 750,
                          y: 250
                      }}
            />

            <KnobPredefined id="oscillators-second-octave"
                            caption="Octave"
                            values={[0.25, 0.5, 1, 2, 4]}
                            valueIndex={closestIndex([0.25, 0.5, 1, 2, 4], synthParams.values.osc2octaveoffset)}
                            onValueIndexChanged={valueIndex => synthStore.setSynthParameter('osc2octaveoffset', [0.25, 0.5, 1, 2, 4][valueIndex])}
                            from={0.25}
                            to={4}
                            displayValue={`${["-2", "-1", "0", "+1", "+2"][closestIndex([0.25, 0.5, 1, 2, 4], synthParams.values.osc2octaveoffset)]}`}
                            size={KnobSize.SMALL}
                            position={{
                                x: 113,
                                y: 130
                            }}
            />

            <KnobPredefined id="oscillators-third-octave"
                            caption="Octave"
                            values={[0.25, 0.5, 1, 2, 4]}
                            valueIndex={closestIndex([0.25, 0.5, 1, 2, 4], synthParams.values.osc3octaveoffset)}
                            onValueIndexChanged={valueIndex => synthStore.setSynthParameter('osc3octaveoffset', [0.25, 0.5, 1, 2, 4][valueIndex])}
                            from={0.25}
                            to={4}
                            displayValue={`${["-2", "-1", "0", "+1", "+2"][closestIndex([0.25, 0.5, 1, 2, 4], synthParams.values.osc3octaveoffset)]}`}
                            size={KnobSize.SMALL}
                            position={{
                                x: 113,
                                y: 230
                            }}
            />

            <KnobPredefined id="oscillators-second-semi"
                            caption="Semi"
                            values={[-7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7]}
                            valueIndex={closestIndex([-7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7], synthParams.values.osc2semioffset)}
                            onValueIndexChanged={valueIndex => synthStore.setSynthParameter('osc2semioffset', [-7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7][valueIndex])}
                            from={-7}
                            to={7}
                            displayValue={`${synthParams.values.osc2semioffset}`}
                            size={KnobSize.SMALL}
                            position={{
                                x: 180,
                                y: 130
                            }}
            />

            <KnobFree id="oscillators-second-fine"
                      caption="Fine"
                      onValueChanged={value => synthStore.setSynthParameter('osc2detune', value)}
                      value={synthParams.values.osc2detune}
                      from={-0.1}
                      to={0.1}
                      displayValue={synthParams.values.osc2detune.toFixed(3)}
                      size={KnobSize.SMALL}
                      position={{
                          x: 250,
                          y: 130
                      }}
            />

            <KnobPredefined id="oscillators-third-semi"
                            caption="Semi"
                            values={[-7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7]}
                            valueIndex={closestIndex([-7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7], synthParams.values.osc3semioffset)}
                            onValueIndexChanged={valueIndex => synthStore.setSynthParameter('osc3semioffset', [-7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7][valueIndex])}
                            from={-7}
                            to={7}
                            displayValue={`${synthParams.values.osc3semioffset}`}
                            size={KnobSize.SMALL}
                            position={{
                                x: 180,
                                y: 230
                            }}
            />

            <KnobFree id="oscillators-third-fine"
                      caption="Fine"
                      onValueChanged={value => synthStore.setSynthParameter('osc3detune', value)}
                      value={synthParams.values.osc3detune}
                      from={-0.1}
                      to={0.1}
                      displayValue={synthParams.values.osc3detune.toFixed(3)}
                      size={KnobSize.SMALL}
                      position={{
                          x: 250,
                          y: 230
                      }}
            />

            <KnobPredefined id="oscillators-first-waveform"
                            caption="OSC1 waveform"
                            values={[0, 1, 2, 3]}
                            valueIndex={closestIndex([0, 1, 2, 3], synthParams.values.osc1waveform)}
                            onValueIndexChanged={valueIndex => synthStore.setSynthParameter('osc1waveform', [0, 1, 2, 3][valueIndex])}
                            from={0}
                            to={3}
                            filled={true}
                            displayValue={Waveform[synthParams.values.osc1waveform]}
                            position={{
                                x: 20,
                                y: 20
                            }}
            />

            <KnobPredefined id="oscillators-second-waveform"
                            caption="OSC2 waveform"
                            values={[0, 1, 2, 3]}
                            valueIndex={closestIndex([0, 1, 2, 3], synthParams.values.osc2waveform)}
                            onValueIndexChanged={valueIndex => synthStore.setSynthParameter('osc2waveform', [0, 1, 2, 3][valueIndex])}
                            from={0}
                            to={3}
                            filled={true}
                            displayValue={Waveform[synthParams.values.osc2waveform]}
                            position={{
                                x: 20,
                                y: 120
                            }}
            />

            <KnobPredefined id="oscillators-third-waveform"
                            caption="OSC3 waveform"
                            values={[0, 1, 2, 3]}
                            valueIndex={closestIndex([0, 1, 2, 3], synthParams.values.osc3waveform)}
                            onValueIndexChanged={valueIndex => synthStore.setSynthParameter('osc3waveform', [0, 1, 2, 3][valueIndex])}
                            from={0}
                            to={3}
                            filled={true}
                            displayValue={Waveform[synthParams.values.osc3waveform]}
                            position={{
                                x: 20,
                                y: 220
                            }}
            />

            <KnobFree id="oscillators-first-gain"
                      caption="Volume"
                      onValueChanged={value => synthStore.setSynthParameter('__OscVolume__osc1gain', value)}
                      value={synthParams.values.__OscVolume__osc1gain}
                      from={0}
                      to={1}
                      filled={true}
                      displayValue={synthParams.values.__OscVolume__osc1gain.toFixed(2)}
                      position={{
                          x: 320,
                          y: 20
                      }}
            />

            <KnobFree id="oscillators-second-gain"
                      caption="Volume"
                      onValueChanged={value => synthStore.setSynthParameter('__OscVolume__osc2gain', value)}
                      value={synthParams.values.__OscVolume__osc2gain}
                      from={0}
                      to={1}
                      filled={true}
                      displayValue={synthParams.values.__OscVolume__osc2gain.toFixed(2)}
                      position={{
                          x: 320,
                          y: 120
                      }}
            />

            <KnobFree id="oscillators-third-gain"
                      caption="Volume"
                      onValueChanged={value => synthStore.setSynthParameter('__OscVolume__osc3gain', value)}
                      value={synthParams.values.__OscVolume__osc3gain}
                      from={0}
                      to={1}
                      filled={true}
                      displayValue={synthParams.values.__OscVolume__osc3gain.toFixed(2)}
                      position={{
                          x: 320,
                          y: 220
                      }}
            />

            <KnobFree id="noise-gain"
                      caption="Noise"
                      onValueChanged={value => synthStore.setSynthParameter('__OscVolume__noiseGain', value)}
                      value={synthParams.values.__OscVolume__noiseGain}
                      from={0}
                      to={1}
                      displayValue={synthParams.values.__OscVolume__noiseGain.toFixed(2)}
                      position={{
                          x: 400,
                          y: 80
                      }}
                      size={KnobSize.SMALL}
            />

            <KnobFree id="filter-cutoff"
                      caption="LPF Cutoff"
                      onValueChanged={value => synthStore.setSynthParameter('__LowPass__cutoffFrequency', value)}
                      value={synthParams.values.__LowPass__cutoffFrequency}
                      from={40}
                      to={22000}
                      displayValue={`${synthParams.values.__LowPass__cutoffFrequency.toFixed(2)}Hz`}
                      position={{
                          x: 470,
                          y: 20
                      }}
                      conversionType="exponential"
            />

            <KnobFree id="filter-lp-resonance"
                      caption="Resonance"
                      onValueChanged={value => synthStore.setSynthParameter('__LowPass__resonance', value)}
                      value={synthParams.values.__LowPass__resonance}
                      from={0.01}
                      to={4}
                      filled={false}
                      displayValue={synthParams.values.__LowPass__resonance.toFixed(2)}
                      size={KnobSize.SMALL}
                      position={{
                          x: 468,
                          y: 120
                      }}
            />

            <KnobFree
                id="adsr-attack"
                caption="Attack"
                onValueChanged={value => synthStore.setSynthParameter('__ADSR__attackTime', value === 0 ? 0.000001 : value)}
                value={synthParams.values.__ADSR__attackTime}
                from={0}
                to={5}
                filled={false}
                displayValue={`${synthParams.values.__ADSR__attackTime.toFixed(3)}s`}
                position={{
                    x: 35,
                    y: 340
                }}
                conversionType="exponential"
            />

            <KnobFree
                id="adsr-decay"
                caption="Decay"
                onValueChanged={value => synthStore.setSynthParameter('__ADSR__decayTime', value === 0 ? 0.000001 : value)}
                value={synthParams.values.__ADSR__decayTime}
                from={0}
                to={5}
                filled={false}
                displayValue={`${synthParams.values.__ADSR__decayTime.toFixed(3)}s`}
                position={{
                    x: 105,
                    y: 340
                }}
                conversionType="exponential"
            />

            <KnobFree
                id="adsr-sustain"
                caption="Sustain"
                onValueChanged={value => synthStore.setSynthParameter('__ADSR__sustainLevel', value)}
                value={synthParams.values.__ADSR__sustainLevel}
                from={0}
                to={1}
                filled={false}
                displayValue={synthParams.values.__ADSR__sustainLevel.toFixed(2)}
                position={{
                    x: 175,
                    y: 340
                }}
            />

            <KnobFree
                id="adsr-release"
                caption="Release"
                onValueChanged={value => synthStore.setSynthParameter('__ADSR__releaseTime', value === 0 ? 0.000001 : value)}
                value={synthParams.values.__ADSR__releaseTime}
                from={0}
                to={5}
                filled={false}
                displayValue={`${synthParams.values.__ADSR__releaseTime.toFixed(3)}s`}
                position={{
                    x: 245,
                    y: 340
                }}
                conversionType="exponential"
            />

            {/*Freeverb*/}

            <KnobFree
                id="freeverb-room-size"
                caption="Room Size"
                onValueChanged={value => synthStore.setSynthParameter('__Freeverb__roomSize', value === 1 ? 0.99 : value)}
                value={synthParams.values.__Freeverb__roomSize}
                from={0}
                to={0.99}
                filled={false}
                displayValue={synthParams.values.__Freeverb__roomSize.toFixed(2)}
                position={{
                    x: 425,
                    y: 220
                }}
                size={KnobSize.SMALL}
            />

            <KnobFree
                id="freeverb-dampening"
                caption="Damp"
                onValueChanged={value => synthStore.setSynthParameter('__Freeverb__damp', value)}
                value={synthParams.values.__Freeverb__damp}
                from={0}
                to={1}
                filled={false}
                displayValue={synthParams.values.__Freeverb__damp.toFixed(2)}
                position={{
                    x: 435,
                    y: 300
                }}
                size={KnobSize.SMALL}
            />

            <KnobFree
                id="freeverb-wet"
                caption="Wet"
                onValueChanged={value => synthStore.setSynthParameter('__Freeverb__dryWet', value)}
                value={synthParams.values.__Freeverb__dryWet}
                from={0}
                to={1}
                filled={false}
                displayValue={synthParams.values.__Freeverb__dryWet.toFixed(2)}
                position={{
                    x: 435,
                    y: 380
                }}
                size={KnobSize.SMALL}
            />

            {/* Echo */}
            <KnobFree
                id="echo-delay"
                caption="Delay"
                onValueChanged={value => synthStore.setSynthParameter('__Echo__delayTime', value)}
                value={synthParams.values.__Echo__delayTime}
                from={0}
                to={1}
                filled={false}
                displayValue={synthParams.values.__Echo__delayTime.toFixed(2)}
                position={{
                    x: 525,
                    y: 220
                }}
                size={KnobSize.SMALL}
            />

            <KnobFree
                id="echo-feedback"
                caption="Feedback"
                onValueChanged={value => synthStore.setSynthParameter('__Echo__feedback', value)}
                value={synthParams.values.__Echo__feedback}
                from={0}
                to={1}
                filled={false}
                displayValue={synthParams.values.__Echo__feedback.toFixed(2)}
                position={{
                    x: 518,
                    y: 300
                }}
                size={KnobSize.SMALL}
            />

            <KnobFree
                id="echo-wet"
                caption="Wet"
                onValueChanged={value => synthStore.setSynthParameter('__Echo__dryWet', value)}
                value={synthParams.values.__Echo__dryWet}
                from={0}
                to={1}
                filled={false}
                displayValue={synthParams.values.__Echo__dryWet.toFixed(2)}
                position={{
                    x: 527,
                    y: 380
                }}
                size={KnobSize.SMALL}
            />

            {/* LFO */}
            <KnobFree
                id="lfo-frequency"
                caption="LFO Freq"
                onValueChanged={value => synthStore.setSynthParameter('lfoFrequency', value)}
                value={synthParams.values.lfoFrequency}
                from={0}
                to={20}
                filled={false}
                displayValue={`${synthParams.values.lfoFrequency.toFixed(2)}Hz`}
                position={{
                    x: 340,
                    y: 380
                }}
                conversionType="exponential"
                size={KnobSize.SMALL}
            />

            <KnobPredefined
                id="lfo-waveform"
                caption="LFO Wave"
                values={[0, 1, 2, 3]}
                valueIndex={closestIndex([0, 1, 2, 3], synthParams.values.lfowaveform)}
                onValueIndexChanged={valueIndex => synthStore.setSynthParameter('lfowaveform', [0, 1, 2, 3][valueIndex])}
                from={0}
                to={3}
                filled={true}
                displayValue={Waveform[synthParams.values.lfowaveform]}
                position={{
                    x: 340,
                    y: 310
                }}
                size={KnobSize.SMALL}
            />

            <Wires/>

        </div>
    </div>
}
