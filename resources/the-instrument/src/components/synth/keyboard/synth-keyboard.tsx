import "./synth-keyboard.scss";

import classNames from "classnames";

import {audioContext} from "../../../audio-context.ts";
import {KeyboardListener} from "./keyboard-listener.tsx";
import {keyboardStore} from "../../../stores/keyboard.store.ts";
import {useStoreSubscribe} from "@dgaa/use-store-subscribe";
import {synthStore} from "../../../stores/synth.store.ts";

console.log(audioContext);

const blackKeysLeft = [
    37, 88, 138, 219, 269, 351, 401, 451, 532, 582, 664, 714, 764
]

const notes = "C,Db,D,Eb,E,F,Gb,G,Ab,A,Bb,B,".split(',');
const showNotes = false;


export const SynthKeyboard = () => {

    const preset = useStoreSubscribe(synthStore.preset);

    const playNote = (note: number) => {
        keyboardStore.keyOn(note);
    }

    const stopNote = (note: number) => {
        keyboardStore.keyOff(note);
    }

    const isWhite = (key: number) => {
        return [0, 2, 4, 5, 7, 9, 11].includes(key % 12);
    }

    const drawBlackKeys = () => {
        let currentKeyNumber = 0;

        return Array.from(new Array(32), (_, i) => i + 5 + (preset.values.UI_OCTAVE * 12))
            .map(n => {
                if (isWhite(n)) {
                    return null;
                }

                const currentKey = notes[n % 12];
                return <div className={classNames("key black-key")}
                            style={{left: blackKeysLeft[currentKeyNumber++] - 10}} key={n}
                            onMouseDown={() => playNote(n)}
                            onMouseUp={() => stopNote(n)}>{showNotes ? currentKey : ''}</div>
            })
    }

    return <div className="synth-keyboard">
        <div className="keyboard-white-keys">
            {
                Array.from(new Array(32), (_, i) => i + 5 + (preset.values.UI_OCTAVE * 12))
                    .map(n => {
                        return isWhite(n)
                            ? <div className={classNames("key white-key")} key={n}
                                   onMouseDown={() => playNote(n)}
                                   onMouseUp={() => stopNote(n)}>{showNotes ? notes[n % 12] : ''}</div>
                            : null;
                    })
            }
        </div>
        <div className="keyboard-black-keys">
            {
                drawBlackKeys()
            }
        </div>

        <KeyboardListener/>
    </div>
}
