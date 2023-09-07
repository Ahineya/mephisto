import {FC, PropsWithChildren, useCallback, useEffect, useRef} from 'react';
import './patch-point.scss';
import {wireStore} from "../../../../stores/wire-store.ts";
import {useStoreSubscribe} from "@dgaa/use-store-subscribe";

type IProps = {
    type: 'input' | 'output';
    position: {
        x: number;
        y: number;
    };
    modulePosition: {
        x: number;
        y: number;
    };
    controlId: string;
}

export const PatchPoint: FC<PropsWithChildren<IProps>> = ({
                                                              type,
                                                              position,
                                                              children,
                                                              controlId,
                                                          }) => {
    const realRef = useRef<HTMLDivElement>(null);

    const draggedWireId = useStoreSubscribe(wireStore.draggedWireId);

    const startWire = (e: React.PointerEvent<HTMLDivElement>) => {
        console.log('startWire');
        e.stopPropagation();

        const closestSynth = realRef.current!.closest('.synth');

        if (!closestSynth) {
            console.error('No closest synth found. Patch point should be in the synth.');
            return;
        }

        const rect = (e.target! as HTMLDivElement).closest(`.patch-point`)!
            .getBoundingClientRect();

        wireStore.startWireDrag(controlId, {
            position: {
                x: position.x + rect.width / 2 + 13, // Don't ask me, it is some offsets from my very old code
                y: position.y + rect.height / 2 - 4 + 12.5,
            },
            type,
            controlId,
        }, {
            position: {
                x: position.x + rect.width / 2 + 13,
                y: position.x + rect.width / 2 - 4 + 12.5,
            },
            type: 'cursor',
            controlId: null,
        });
    };

    const updateWireToPos = useCallback((e: PointerEvent) => {
        const closestSynthPanel = realRef.current!.closest('.synth-panel-container');

        if (!closestSynthPanel) {
            console.error('No closest synth panel found. Patch point should be in the synth panel.');
            return;
        }

        const closestSynthPanelRect = closestSynthPanel!.getBoundingClientRect();
        const closestSynthPanelPosition = {
            x: closestSynthPanelRect.x,
            y: closestSynthPanelRect.y,
        };

        wireStore.updateWireTo(draggedWireId!, {
            position: {
                x: e.clientX - closestSynthPanelPosition.x + 13,
                y: e.clientY - closestSynthPanelPosition.y + 12.5,
            },
            type: 'cursor',
            controlId: null,
        });
    }, [draggedWireId]);

    const cancelWire = useCallback(() => {
        wireStore.deleteWire(draggedWireId!);
    }, [draggedWireId]);

    const connectWire = (e: any) => {
        const rect = e.target.closest(`.patch-point`)
            .getBoundingClientRect();

        if (!draggedWireId) {
            return;
        }

        wireStore.connectWireTo(draggedWireId, {
            position: {
                x: position.x + rect.width / 2 + 13,
                y: position.y + rect.height / 2 - 4 + 12.5,
            },
            type,
            controlId,
        });
    };

    useEffect(() => {
        if (!draggedWireId) {
            return;
        }

        window.addEventListener('pointermove', updateWireToPos);
        window.addEventListener('pointerup', cancelWire);

        return () => {
            window.removeEventListener('pointermove', updateWireToPos);
            window.removeEventListener('pointerup', cancelWire);
        };
    }, [draggedWireId]);

    return (
        <div
            className="patch-point"
            style={{
                left: position.x,
                top: position.y,
            }}
            onPointerDown={startWire}
            onPointerUp={connectWire}
            ref={realRef}
        >
            {children}
        </div>
    );
}
