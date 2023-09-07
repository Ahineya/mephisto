import {BehaviorSubject} from "rxjs";
import {audioContext, synth} from "../audio-context";
// import {synthStore} from "./synth.store";
// import {ISynthPreset} from "./synth.interface";
// import {ISynthNode} from "../synth-node.interface";

// const TRIGGER = "__Karplus__pluckTrigger";
const FREQ = "frequency";
const TRIGGER = "trigger";
// const FREQ = "__Karplus__frequency";

class KeyboardStore {
    public onPressedKeyMidiCodeChange = new BehaviorSubject<number | null>(null);
    public onShowNotesChange = new BehaviorSubject(false);

    // private currentSynthPreset!: ISynthPreset;

    constructor() {
        // synthStore.onCurrentPresetChanged.subscribe(values => {
        // this.currentSynthPreset = values;
        // })
    }

    public toggleShowNotes() {
        this.onShowNotesChange.next(!this.onShowNotesChange.getValue());
    }

    public keyOn(key: number) {
        if (audioContext.state === "suspended") {
            audioContext.resume();

            synth.port.postMessage({
                command: 'init'
            });
        }

        // if (this.currentSynthPreset.values.misc.retrigger) {
        //   this.play(key);
        // } else {
        //   synth.then((s: ISynthNode) => s.setSynthGate(0));

        synth.port.postMessage({
            command: 'setParameter',
            setter: {
                name: TRIGGER,
                value: 0
            }
        });

        setTimeout(() => {
            this.play(key);
        }, 10);
        // }
    }

    private play(key: number) {
        this.onPressedKeyMidiCodeChange.next(key);

        const freq = Math.pow(2, ((key + 12) - 69) / 12) * 440;

        synth.port.postMessage({
            command: 'setParameter',
            setter: {
                name: 'globalgate',
                value: 0
            }
        });

        synth.port.postMessage({
            command: 'setParameter',
            setter: {
                name: FREQ,
                value: freq
            }
        });
        synth.port.postMessage({
            command: 'setParameter',
            setter: {
                name: TRIGGER,
                value: 1
            }
        });
        setTimeout(() => {
            synth.port.postMessage({
                command: 'setParameter',
                setter: {
                    name: 'globalgate',
                    value: 1
                }
            });


        }, 10);
    }

    public keyOff(key: number | null) {
        if (key === null) {
            this.onPressedKeyMidiCodeChange.next(null);

            synth.port.postMessage({
                command: 'setParameter',
                setter: {
                    name: TRIGGER,
                    value: 0
                }
            });

            return;
        }

        const currentKey = this.onPressedKeyMidiCodeChange.getValue();

        if (key !== currentKey) {
            return;
        }

        this.onPressedKeyMidiCodeChange.next(null);

        synth.port.postMessage({
            command: 'setParameter',
            setter: {
                name: TRIGGER,
                value: 0
            }
        });
    }
}

export const keyboardStore = new KeyboardStore();
