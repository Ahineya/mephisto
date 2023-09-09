import "./synth-controls.scss";
import {Wires} from "./wires/wires.tsx";
import {SynthPanel} from "./synth-panel/synth-panel.tsx";
import {PatchPoint} from "./patch-point/patch-point.tsx";
import {KnobFree} from "./knobs/knob-free.tsx";
import {KnobSize} from "./knobs/knob.interface.ts";
import {useState} from "react";
import {synth} from "../../../audio-context.ts";

// import {PanelEnvelope} from "./panels/panel-envelope";
// import {PanelDistortion} from "./panels/panel-distortion";
// import {PanelReverb} from "./panels/panel-reverb";
// import {PanelDelay} from "./panels/panel-delay";
// import {PanelLfo} from "./panels/panel-lfo";
// import {PanelFilter} from "./panels/panel-filter";
// import {PanelAnalyser} from "./panels/panel-analyser";
// import {PanelMisc} from "./panels/panel-misc";
// import {PanelOscillators} from "./panels/panel-oscillators";
// import {SynthMenu} from "./synth-menu";
// import {PanelFreqAnalyser} from "./panels/panel-freq";

export const SynthControls = () => {

    const [freqMod, setFreqMod] = useState(0);

    return <div className="synth-controls-container">

        <div className="synth-controls">
            <SynthPanel left={610} top={12} width={230} height={288} caption="Ainanenane">

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 20,
                        y: 20
                    }}
                    controlId={"attenIn"}
                    label="ATTEN"
                />
                <PatchPoint
                    type={"output"}
                    position={{
                        x: 20 + 50,
                        y: 20
                    }}
                    controlId={"attenOut"}
                    label="ATTEN"
                />

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 20 + 100,
                        y: 20
                    }}
                    controlId={"snhIn"}
                    label="S&H"
                />
                <PatchPoint
                    type={"output"}
                    position={{
                        x: 20 + 150,
                        y: 20
                    }}
                    controlId={"snhOut"}
                    label="S&H"
                />

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 20,
                        y: 20 + 50,
                    }}
                    controlId={"mix1"}
                    label="MIX1"
                />

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 20 + 50,
                        y: 20 + 50,
                    }}
                    controlId={"mix2"}
                    label="MIX2"
                />

                <PatchPoint
                    type={"output"}
                    position={{
                        x: 120,
                        y: 20 + 50,
                    }}
                    controlId={"mixout"}
                    label="MIX"
                />

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 20 + 150,
                        y: 20 + 50
                    }}
                    controlId={"cutoff"}
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
                    controlId={"coscgainmodamount"}
                    label="OSC1 GAIN"
                />

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 120,
                        y: 20 + 100
                    }}
                    controlId={"osc2gainmodamount"}
                    label="OSC2 GAIN"
                />

                <PatchPoint
                    type={"input"}
                    position={{
                        x: 120 + 50,
                        y: 20 + 100
                    }}
                    controlId={"noiseGainmodamount"}
                    label="NOISE GAIN"
                />

                <PatchPoint
                    type={"output"}
                    position={{
                        x: 20,
                        y: 20 + 150
                    }}
                    controlId={"osc1"}
                    label="OSC1"
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
                    controlId={"Adsr"}
                    label="ADSR"
                />

                <PatchPoint
                    type={"output"}
                    position={{
                        x: 20 + 150,
                        y: 20 + 150,
                    }}
                    controlId={"noiseOut"}
                    label="NOISE"
                />


            </SynthPanel>
            <KnobFree id="reverb-mix"
                      caption="Freq Mod"
                      onValueChanged={(v) => {
                          setFreqMod(v);

                          synth.port.postMessage({
                              command: "setParameter",
                              setter: {
                                  name: "frequencyModAmount",
                                  value: v
                              }
                          })
                      }}
                      value={freqMod}
                      from={0}
                      to={1}
                      filled={false}
                      displayValue={freqMod.toFixed(2)}
                      size={KnobSize.SMALL}
            />
            {/*<PanelOscillators left={12} top={12} width={357} height={288}/>*/}
            {/*<PanelMisc left={12} top={318} width={122} height={123}/>*/}
            {/*<PanelAnalyser left={147} top={318} width={222} height={123}/>*/}
            {/*<PanelFreqAnalyser left={146} top={316} width={222} height={123}/>*/}
            {/*<PanelFilter left={382} top={12} width={173} height={107}/>*/}
            {/*<PanelEnvelope left={568} top={12} width={272} height={208}/>*/}
            {/*<PanelLfo left={382} top={137} width={173} height={304}/>*/}
            {/*<PanelDistortion left={568} top={237} width={85} height={204}/>*/}
            {/*<PanelDelay left={662} top={237} width={85} height={204}/>*/}
            {/*<PanelReverb left={755} top={237} width={85} height={204}/>*/}

            <Wires/>

        </div>
    </div>
}
