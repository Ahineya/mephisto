import {BehaviorSubject} from "rxjs";
import {synthFacade} from "../audio-context";
import {synthStore} from "./synth.store.ts";

const FREQ = "frequency";
const TRIGGER = "trigger";

class KeyboardStore {
    public onPressedKeyMidiCodeChange = new BehaviorSubject<number | null>(null);
    public onShowNotesChange = new BehaviorSubject(false);

    constructor() {
    }

    public toggleShowNotes() {
        this.onShowNotesChange.next(!this.onShowNotesChange.getValue());
    }

    public keyOn(key: number) {
        synthStore.initSynth();
        synthFacade.setParameter(TRIGGER, 0);

        setTimeout(() => {
            this.play(key);
        }, 10);
    }

    private play(key: number) {
        this.onPressedKeyMidiCodeChange.next(key);

        const freq = Math.pow(2, ((key + 12) - 69) / 12) * 440; // Ainanenane

        synthFacade.setParameter(FREQ, freq);
        synthFacade.setParameter(TRIGGER, 1)
    }

    public keyOff(key: number | null) {
        if (key === null) {
            this.onPressedKeyMidiCodeChange.next(null);
            synthFacade.setParameter(TRIGGER, 0);

            return;
        }

        const currentKey = this.onPressedKeyMidiCodeChange.getValue();

        if (key !== currentKey) {
            return;
        }

        this.onPressedKeyMidiCodeChange.next(null);

        synthFacade.setParameter(TRIGGER, 0);
    }
}

export const keyboardStore = new KeyboardStore();
