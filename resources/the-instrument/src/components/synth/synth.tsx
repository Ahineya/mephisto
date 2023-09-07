import {SynthKeyboard} from "./keyboard/synth-keyboard.tsx";
import {SynthControls} from "./synth-controls/synth-controls.tsx";
import "./synth.scss";

export const Synth = () => {
    return (
        <div className="synth">
            <SynthControls/>
            <SynthKeyboard/>
        </div>
    )
}
