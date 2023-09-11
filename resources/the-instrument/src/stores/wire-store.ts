import {StoreSubject} from "@dgaa/store-subject";
import {Wire} from "../types/modular.ts";
import {WireConnectionPoint} from "../types/modular.ts";
import {synthStore} from "./synth.store.ts";

let currentColor = 0;
const colors = ['#face8D', '#61461b'];

function getNextWireColor() {
    const nextColor = colors[currentColor];
    currentColor = (currentColor + 1) % colors.length;
    return nextColor;
}

/*
   This store is adapted from my very old web-based modular synthesizer project which used Zustand.
   It is a bit of a mess, but it works. I think.
 */
class WireStore {
    public wires = new StoreSubject<Wire[]>([]);

    public draggedWireId = new StoreSubject<string | null>(null);

    constructor() {
        const wires = localStorage.getItem('wires');

        if (wires) {
            this.wires.next(JSON.parse(wires));
        }

        // Save wires to local storage
        this.wires.subscribe(wires => {
            localStorage.setItem('wires', JSON.stringify(wires));
        });

        synthStore.onLoadedChanged.subscribe(loaded => {
            if (loaded) {
                this.connectWires();
            }
        })
    }

    clear() {
        this.wires.getValue().forEach((wire) => {
            if (wire.from.controlId && wire.to.controlId) {
                synthStore.disconnect(wire.from.controlId, wire.to.controlId);
            }
        });

        this.wires.next([]);
    }

    public connectWires(wires?: Wire[]) {
        const realWires = wires || this.wires.getValue();

        realWires.forEach((wire) => {
            if (!wire.from.controlId || !wire.to.controlId) {
                return;
            }

            synthStore.connect(wire.from.controlId, wire.to.controlId);
        });

        this.wires.next(realWires.map((wire) => {
            return {
                ...wire,
                connected: true,
            };
        }));
    }

    private createWire(from: WireConnectionPoint, to: WireConnectionPoint) {
        const uuid = `wire_${Math.random()}`;

        if (from.type === 'input') {
            if (this.wires.getValue().find((wire) => wire.to.controlId === from.controlId)) {
                return;
            }
        }

        this.wires.next([
            ...this.wires.getValue(),
            {
                uuid,
                color: getNextWireColor(),
                from,
                to,
                connected: false,
            }
        ]);

        console.log('CREATE WIRE', this.wires.getValue());

        this.draggedWireId.next(uuid);
    }

    private getWireByControlId(controlId: string) {
        const wire = this.wires.getValue().find((wire) => {
            return wire.from.controlId === controlId || wire.to.controlId === controlId;
        });

        if (!wire) {
            return null;
        }

        return {
            wire,
            type: wire.from.controlId === controlId ? 'from' : 'to',
        };
    }

    startWireDrag(controlId: string, from: WireConnectionPoint, to: WireConnectionPoint) {
        const wire = this.getWireByControlId(controlId);
        if (!wire) {
            this.createWire(from, to);
            return;
        }

        if (wire.wire.from.controlId && wire.wire.to.controlId) {
            synthStore.disconnect(wire.wire.from.controlId, wire.wire.to.controlId);
        }

        const newFrom = wire.type === 'to' ? wire.wire.from : wire.wire.to;

        this.wires.next(this.wires.getValue().map((w) => {
            if (wire.wire.uuid === w.uuid) {
                return {
                    ...w,
                    from: newFrom,
                    to,
                    connected: false,
                };
            }
            return w;
        }));

        this.draggedWireId.next(wire.wire.uuid);
    }

    updateWireTo(wireId: string, to: WireConnectionPoint) {
        this.wires.next(this.wires.getValue().map((wire) => {
            return wire.uuid === wireId
                ? {
                    ...wire,
                    to,
                }
                : wire;
        }));
    }

    connectWireTo(wireId: string, to: WireConnectionPoint) {
        const wire = this.wires.getValue().find((wire) => wire.uuid === wireId);

        if (!wire) {
            return;
        }

        let newFrom = wire.from;
        let newTo = to;

        if ((wire.from.type === 'input') && to.type === 'output') {
            newFrom = to;
            newTo = wire.from;
        } else if (wire.from.type === 'output' && (to.type === 'input')) {
            newFrom = wire.from;
            newTo = to;
        } else {
            console.log('cannot connect wire', wire, to);
            return;
        }

        const wires = this.wires.getValue();

        if (wires.find((wire) => wire.to.controlId === newTo.controlId)) {
            return;
        }

        this.wires.next(this.wires.getValue().map((wire) => {
            return wire.uuid === wireId
                ? {
                    ...wire,
                    to: newTo,
                    from: newFrom,
                    connected: true,
                }
                : wire;
        }));

        this.draggedWireId.next(null);

        if (!newFrom.controlId || !newTo.controlId) {
            return;
        }

        synthStore.connect(newFrom.controlId, newTo.controlId);
    }

    deleteWire(wireId: string, deleteConnected = false) {
        const wire = this.wires.getValue().find((wire) => wire.uuid === wireId);

        if (wire && wire.connected && !deleteConnected) {
            return;
        }

        if (!wire) {
            return;
        }

        this.wires.next(this.wires.getValue().filter((wire) => wire.uuid !== wireId));

        this.draggedWireId.next(null);

        if (!wire.from.controlId || !wire.to.controlId) {
            return;
        }

        synthStore.disconnect(wire.from.controlId, wire.to.controlId);
    }

    exportWires() {
        return this.wires.getValue();
    }
}

export const wireStore = new WireStore();