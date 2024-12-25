import {SynthPreset} from "../../../../types/synthesizer.types.ts";
import {FC} from "react";
import {presetStore} from "../../../../stores/preset.store.ts";

import "./preset-loader.scss";

type IProps = {
    preset: SynthPreset;
}

export const PresetLoader: FC<IProps> = ({preset}) => {
    const loadPreset = (preset: SynthPreset) => () => {
        presetStore.loadPreset(preset);
        window.location.reload();
    }

    return (
        <button className="preset-loader" onClick={loadPreset(preset)}>{preset.parameters.name}</button>
    )
}