import {synthStore} from "./synth.store.ts";
import {wireStore} from "./wire-store.ts";
import {SynthPreset} from "../types/synthesizer.types.ts";
import {copyToClipboard} from "../helpers/copy-to-clipboard.ts";
import {StoreSubject} from "@dgaa/store-subject";

class PresetStore {
    public showPresets = new StoreSubject(false);

    public exportPreset() {
        const parameters = synthStore.exportPreset();
        const wires = wireStore.exportWires();

        const preset: SynthPreset = {
            parameters,
            wires,
        }

        copyToClipboard(JSON.stringify(preset));
    }

    public loadPreset(preset: SynthPreset) {
        synthStore.loadPreset(preset.parameters);
        wireStore.clear();
        wireStore.connectWires(preset.wires);
    }

    public toggleShowPresets() {
        this.showPresets.next(!this.showPresets.getValue());
    }
}

export const presetStore = new PresetStore();
