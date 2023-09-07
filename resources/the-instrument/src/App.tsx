import './App.css'
import {SynthKeyboard} from "./keyboard/synth-keyboard.tsx";
import {useStoreSubscribe} from "@dgaa/use-store-subscribe";
import {synthStore} from "./stores/synth.store.ts";
import {Mermaid} from "./mermaid/mermaid.tsx";

function App() {
    const chart = useStoreSubscribe(synthStore.chart);

    const conn = () => {
        synthStore.connectLfoToFreqMod();
    }

    const disconn = () => {
        synthStore.disconnectLfoFromFreqMod();
    }

    return (
        <>
            <SynthKeyboard/>
            <button onClick={conn}>Connect lfo to freqmod</button>
            <button onClick={disconn}>Disconnect lfo from freqmod</button>
            {
                chart !== "" && <Mermaid chart={chart}/>
            }
        </>
    )
}

export default App
