import React, {useState} from "react";
import "./synth-menu.scss";
import {initPreset as initSynthPreset, synthStore} from "../../../../stores/synth.store";
import {keyboardStore} from "../../../../stores/keyboard.store";
import classNames from "classnames";
import {wireStore} from "../../../../stores/wire-store.ts";
import {SynthPreset} from "../../../../types/synthesizer.types.ts";
import {presetStore} from "../../../../stores/preset.store.ts";
import {SynthMenuPreset} from "./synth-menu-preset.tsx";
import {Portal} from "../../../portal/portal.tsx";

export const SynthMenu = () => {

    const [importShown, setImportShown] = useState(false);
    const [importPresetValue, setImportPresetValue] = useState("");

    const initPreset = () => {
        synthStore.loadPreset(initSynthPreset);
        wireStore.clear();
    }

    const panic = () => {
        window.location.reload();
    }

    const toggleNotes = () => {
        keyboardStore.toggleShowNotes();
    }

    const exportPreset = () => {
        presetStore.exportPreset();
    }

    const showImportModal = () => {
        setImportShown(true);
    }

    const hideImportModal = (e: React.MouseEvent) => {
        e.stopPropagation();
        setImportShown(false);
    }

    const changeImportPreset = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        setImportPresetValue(e.target.value);
    }

    const importPreset = () => {
        const preset: SynthPreset = JSON.parse(importPresetValue);
        presetStore.loadPreset(preset);
        setImportShown(false);
    }

    return <div className="synth-menu">
        <div className="synth-menu-item" onClick={initPreset}>
            Init
        </div>
        <div className="synth-menu-item" onClick={panic}>
            Panic
        </div>
        <div className="synth-menu-item" onClick={showImportModal}>
            Import

            <Portal>
                <div className={classNames("import", {shown: importShown})}>
                    <textarea name="import" cols={30} rows={10} onChange={changeImportPreset}
                              value={importPresetValue}></textarea>
                    <button className="button-main" onClick={importPreset}>Import</button>
                    <button className="button-main" onClick={hideImportModal}>Cancel</button>
                </div>
            </Portal>
        </div>
        <div className="synth-menu-item" onClick={exportPreset}>
            Export
        </div>
        <SynthMenuPreset/>
        <div className="synth-menu-item" onClick={toggleNotes}>
            Toggle notes
        </div>
        <div className="synth-menu-item">
            About
        </div>
    </div>;
}