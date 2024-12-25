import './preset-selector.scss';
import {PresetLoader} from "./preset-loader.tsx";
import {
    DETUNED_SAW_PRESET, FX_CRASH_PRESET, FX_FM_PRESET,
    KICK_PRESET,
    NASTY_BASS_PRESET,
    SQUARE_ORGAN_PRESET, SUPER_SAW_PRESET,
    WOODEN_FLUTE_PRESET
} from "./presets.tsx";

export const PresetSelector = () => {
    return (
        <div className="preset-selector">
            <h3>Presets</h3>
            <div className="preset-selector-presets">
                <PresetLoader preset={DETUNED_SAW_PRESET}/>
                <PresetLoader preset={KICK_PRESET}/>
                <PresetLoader preset={WOODEN_FLUTE_PRESET}/>
                <PresetLoader preset={NASTY_BASS_PRESET}/>
                <PresetLoader preset={SQUARE_ORGAN_PRESET}/>
                <PresetLoader preset={SUPER_SAW_PRESET}/>
                <PresetLoader preset={FX_FM_PRESET}/>
                <PresetLoader preset={FX_CRASH_PRESET}/>
            </div>
        </div>
    )
}
