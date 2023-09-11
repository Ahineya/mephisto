import {useEffect, useState} from "react";
import {fromEvent} from "rxjs";
// import {keyboardStore} from "../stores/keyboard.store";
// import {synthStore} from "../stores/synth.store";
// import {ISynthPresetMiscValues} from "../stores/synth.interface";
import {audioContext} from "../../../audio-context.ts";
import {keyboardStore} from "../../../stores/keyboard.store.ts";
import {useStoreSubscribe} from "@dgaa/use-store-subscribe";
import {synthStore} from "../../../stores/synth.store.ts";

const keyToMidi: {
  [key: string]: number;
} = {
  a: 24,
  w: 25,
  s: 26,
  e: 27,
  d: 28,
  f: 29,
  t: 30,
  g: 31,
  y: 32,
  h: 33,
  u: 34,
  j: 35,
  k: 36,
  o: 37,
  l: 38,
  p: 39,
  ";": 40,
};

export const KeyboardListener = () => {

  const [isLoaded, setIsLoaded] = useState(false);

  const [misc, setMisc] = useState({
    hold: 0,
    octave: 1,
    gain: 0.5,
    retrigger: 0,
  })

  const preset = useStoreSubscribe(synthStore.preset);

  useEffect(() => {
    const subscriptions = [
      // synthStore.onLoadedChanged.subscribe(isLoaded => {
      //   setIsLoaded(isLoaded);
      // }),
      // synthStore.onCurrentPresetChanged.subscribe(({values: {misc}}) => {
      //   setMisc(misc);
      // }),
      fromEvent<KeyboardEvent>(document, 'keydown')
        .subscribe((e) => {
          if (!isLoaded) {
            // return;
          }

          if (e.repeat) {
            return;
          }

          if (audioContext.state !== 'running') {
            audioContext.resume();
          }

          if (keyToMidi[e.key]) {
            console.log('play note', keyToMidi[e.key] - 12 + preset.values.UI_OCTAVE * 12);
            keyboardStore.keyOn(keyToMidi[e.key] - 12 + preset.values.UI_OCTAVE * 12);
            return;
          }

          if (e.key === 'z' && preset.values.UI_OCTAVE > 0) {
            // synthStore.changeMiscValue('octave', misc.octave - 1);
            synthStore.setInternalParameter('UI_OCTAVE', preset.values.UI_OCTAVE - 1);
            return;
          }

          if (e.key === 'x' && preset.values.UI_OCTAVE < 5) {
            synthStore.setInternalParameter('UI_OCTAVE', preset.values.UI_OCTAVE + 1);
            return;
          }
        }),
      fromEvent<KeyboardEvent>(document, 'keyup')
        .subscribe((e) => {
          if (!misc.hold) {
            if (keyToMidi[e.key]) {
              keyboardStore.keyOff(keyToMidi[e.key] - 12 + preset.values.UI_OCTAVE * 12);
            }
          }
        })
    ];

    return () => subscriptions.forEach(s => s.unsubscribe());
  }, [misc, isLoaded, preset]);

  useEffect(() => {
    if (!navigator.requestMIDIAccess) {
      return;
    }

    navigator.requestMIDIAccess()
      .then(onMIDISuccess, onMIDIFailure);

    function onMIDISuccess(midiAccess: WebMidi.MIDIAccess) {
      console.log(midiAccess);

      const inputs = midiAccess.inputs;

      Array.from(inputs.values()).forEach(input => {
        input.onmidimessage = getMIDIMessage;
      })
    }

    function onMIDIFailure() {
      console.log('Could not access your MIDI devices.');
    }

    function getMIDIMessage(message: WebMidi.MIDIMessageEvent) {
      console.log(message);
      var command = message.data[0];
      var note = message.data[1];
      var velocity = (message.data.length > 2) ? message.data[2] : 0; // a velocity value might not be included with a noteOff command

      console.log(command);

      switch (command) {
        case 149:
          if (velocity > 0) {
            // keyboardStore.keyOn(note);
          } else {
            // keyboardStore.keyOff(note);
          }
          break;
        case 133:
          // keyboardStore.keyOff(note);
          break;
      }
    }


  }, []);

  return <div/>
}