import "./synth-controls.scss";
import {Wires} from "./wires/wires.tsx";
import {SynthPanel} from "./synth-panel/synth-panel.tsx";
import {PatchPoint} from "./patch-point/patch-point.tsx";

import {ReactComponent as PatchPointSvg} from "../../../assets/patch-point.svg";

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
    return <div className="synth-controls-container">

        {/*<SynthMenu/>*/}

        <div className="synth-controls">
            <SynthPanel left={12} top={12} width={357} height={288} caption="Ainanenane">
                <PatchPoint type={"input"} position={{
                    x: 100,
                    y: 100
                }} modulePosition={{
                    x: 0,
                    y: 0
                }} controlId={"frequencyMod"}
                ><PatchPointSvg/></PatchPoint>
                <PatchPoint type={"output"} position={{
                    x: 200,
                    y: 200
                }} modulePosition={{
                    x: 0,
                    y: 0
                }} controlId={"LFO#out"}
                ><PatchPointSvg/></PatchPoint>
            </SynthPanel>

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
