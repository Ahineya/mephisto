import {useEffect, useState} from "react";
import "./synth-keyboard.scss";
// import {synth} from "../../../../audio-context";
// import {synthStore} from "../../../../stores/synth.store";
// import {ISynthPresetMiscValues} from "../../../../stores/synth.interface";
// import {keyboardStore} from "../../../../stores/keyboard.store";
// import {ISynthNode} from "../../../../synth-node.interface";
import classNames from "classnames";

import {audioContext, synth} from "../audio-context.ts";
import {KeyboardListener} from "./keyboard-listener.tsx";
import {keyboardStore} from "../stores/keyboard.store.ts";
console.log(audioContext);

const blackKeysLeft = [
  37, 88, 138, 219, 269, 351, 401, 451, 532, 582, 664, 714, 764
]

const notes = "C,Db,D,Eb,E,F,Gb,G,Ab,A,Bb,B,".split(',');

export const SynthKeyboard = () => {

  const [misc, setMisc] = useState({
    gain: 0.5,
    hold: 0,
    retrigger: 0,
    octave: 0,
  });
  //
  // const [synthController, setSynthController] = useState<ISynthNode | null>(null);
  //
  const [showNotes, setShowNotes] = useState(false);
  const [pressedKey, setPressedKey] = useState<number | null>(null);

  // useEffect(() => {
  //   synth
  //     .then((s: ISynthNode) => {
  //       console.log(Object.keys(s).filter(s => s.startsWith('set')));
  //       setSynthController(s);
  //     })
  // }, []);
  //
  // useEffect(() => {
  //   const subscriptions = [
  //     synthStore.onCurrentPresetChanged.subscribe(({values: {misc}}) => {
  //       if (!misc.hold && synthController) {
  //         keyboardStore.keyOff(null);
  //       }
  //
  //       setMisc(misc);
  //     }),
  //     keyboardStore.onShowNotesChange.subscribe(showNotes => {
  //       setShowNotes(showNotes);
  //     }),
  //     keyboardStore.onPressedKeyMidiCodeChange.subscribe(pressedKey => {
  //       setPressedKey(pressedKey);
  //     })
  //   ];
  //
  //   return () => subscriptions.forEach(s => s.unsubscribe());
  // }, [synthController]);

  const playNote = (note: number) => {
    // keyboardStore.keyOn(note);


    keyboardStore.keyOn(note);

    console.log('play note', note);
  }

  const stopNote = (note: number) => {
    // if (!misc.hold) {
      keyboardStore.keyOff(note);
    // }

    console.log('stop note', note);
  }

  const isWhite = (key: number) => {
    return [0, 2, 4, 5, 7, 9, 11].includes(key % 12);
  }

  const drawBlackKeys = () => {
    let currentKeyNumber = 0;

    return Array.from(new Array(32), (_, i) => i + 5 + (misc.octave * 12))
      .map(n => {
        if (isWhite(n)) {
          return null;
        }

        const currentKey = notes[n % 12];
        return <div className={classNames("key black-key", {pressed: pressedKey === n})} style={{left: blackKeysLeft[currentKeyNumber++] - 10}} key={n}
                    onMouseDown={() => playNote(n)} onMouseUp={() => stopNote(n)}>{showNotes ? currentKey : ''}</div>
      })
  }

  return <div className="synth-keyboard">
    <div className="keyboard-white-keys">
      {
        Array.from(new Array(32), (_, i) => i + 5 + (misc.octave * 12))
          .map(n => {
            return isWhite(n)
              ? <div className={classNames("key white-key", {pressed: pressedKey === n})} key={n} onMouseDown={() => playNote(n)} onMouseUp={() => stopNote(n)}>{showNotes ? notes[n % 12] : ''}</div>
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
