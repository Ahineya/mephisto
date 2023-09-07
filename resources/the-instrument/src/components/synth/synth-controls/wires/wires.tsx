import {Wire} from './wire/wire.tsx';
import './wires.scss';
import {useStoreSubscribe} from "@dgaa/use-store-subscribe";
import {wireStore} from "../../../../stores/wire-store.ts";


const offset = {x: 0, y: 0};

export function Wires() {
    const wires = useStoreSubscribe(wireStore.wires);
    return (
        <div
            className="wires"
        >
            <svg width="100%" height="100%" fill="none">
                <g transform={`translate(${offset.x}, ${offset.y})`}>
                    {
                        wires.map((wire) => {
                            const from = {
                                x: wire.from?.position?.x || 0,
                                y: wire.from?.position?.y || 0,
                            };

                            const to = {
                                x: wire.to?.position?.x || 0,
                                y: wire.to?.position?.y || 0,
                            };

                            return <Wire
                                key={wire.uuid}
                                color={wire.color}
                                from={from}
                                to={to}
                                loose={0.4}
                                opacity={0.5}
                                connected={!!wire.connected}
                            />;
                        })
                    }
                </g>
            </svg>
        </div>
    );
}
