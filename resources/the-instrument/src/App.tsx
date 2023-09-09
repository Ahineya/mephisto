import './App.scss'
import {useStoreSubscribe} from "@dgaa/use-store-subscribe";
import {synthStore} from "./stores/synth.store.ts";
import {Mermaid} from "./components/mermaid/mermaid.tsx";
import {Synth} from "./components/synth/synth.tsx";
import {DragOverlay} from "./components/drag-overlay/drag-overlay.tsx";

function App() {
    const chart = useStoreSubscribe(synthStore.chart);

    return (
        <>
            <Synth />

            {
                chart !== "" && <Mermaid chart={chart}/>
            }

            <DragOverlay/>
        </>
    )
}

export default App
