import "./synth-menu-preset.scss";
import {presetStore} from "../../../../stores/preset.store.ts";
import {useStoreSubscribe} from "@dgaa/use-store-subscribe";
import classNames from "classnames";
import {synthStore} from "../../../../stores/synth.store.ts";

export const SynthMenuPreset = () => {
    const presetsShown = useStoreSubscribe(presetStore.showPresets);
    const preset = useStoreSubscribe(synthStore.preset);

    const togglePresets = () => {
        presetStore.toggleShowPresets();
    }

    return (
        <div
            className="synth-menu-preset-container"
            onClick={togglePresets}
        >
            <div
                className={classNames("synth-menu-preset", {
                    "synth-menu-preset-active": presetsShown
                })}
            >
                {preset?.name || "Custom"}
            </div>
        </div>
    )
}
