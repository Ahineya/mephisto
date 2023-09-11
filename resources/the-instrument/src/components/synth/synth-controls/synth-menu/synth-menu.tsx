import React, {useState} from "react";
import "./synth-menu.scss";
import {initPreset as initSynthPreset, synthStore} from "../../../../stores/synth.store";
import {keyboardStore} from "../../../../stores/keyboard.store";
import classNames from "classnames";
import {wireStore} from "../../../../stores/wire-store.ts";
import {copyToClipboard} from "../../../../helpers/copy-to-clipboard.ts";

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
    const parameters = synthStore.exportPreset();
    const wires = wireStore.exportWires();

    const preset = {
        parameters,
        wires,
    }

    copyToClipboard(JSON.stringify(preset));
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

    const preset = JSON.parse(importPresetValue);

    synthStore.loadPreset(preset.parameters);
    wireStore.clear();
    wireStore.connectWires(preset.wires);

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

      <div className={classNames("import", {shown: importShown})}>
        <textarea name="import" cols={30} rows={10} onChange={changeImportPreset} value={importPresetValue}></textarea>
        <button className="button-main" onClick={importPreset}>Import</button>
        <button className="button-main" onClick={hideImportModal}>Cancel</button>
      </div>
    </div>
    <div className="synth-menu-item" onClick={exportPreset}>
      Export
    </div>
    <div className="synth-menu-item-spacer"/>
    <div className="synth-menu-item" onClick={toggleNotes}>
      Toggle notes
    </div>
    <div className="synth-menu-item">
      About
    </div>
  </div>;
}