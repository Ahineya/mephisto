import './App.css'
import {SynthKeyboard} from "./keyboard/synth-keyboard.tsx";
import {useStoreSubscribe} from "@dgaa/use-store-subscribe";
import {synthStore} from "./stores/synth.store.ts";
import {Mermaid} from "./mermaid/mermaid.tsx";

function App() {
    const chart = useStoreSubscribe(synthStore.chart);

    return (
        <>
            <SynthKeyboard/>
            {
                chart !== "" && <Mermaid chart={chart}/>
            }
        </>
    )
}

export default App
