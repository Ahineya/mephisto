import {FC, PropsWithChildren, useCallback, useEffect, useRef} from 'react';
import './patch-point.scss';
import {wireStore} from "../../../../stores/wire-store.ts";
import {useStoreSubscribe} from "@dgaa/use-store-subscribe";
import {ReactComponent as PatchPointSvg} from "../../../../assets/patch-point.svg";

type IProps = {
    type: 'input' | 'output';
    position: {
        x: number;
        y: number;
    };
    controlId: string;
    label?: string;
}

export const PatchPoint: FC<PropsWithChildren<IProps>> = ({
                                                              type,
                                                              position,
                                                              controlId,
                                                              label
                                                          }) => {
    return (
        <PatchPointInternal type={type} position={position} controlId={controlId}>
            <PatchPointSvg/>
            <label className={`${type}-label`}>{label || type}</label>
        </PatchPointInternal>
    )
}

export const PatchPointInternal: FC<PropsWithChildren<IProps>> = ({
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

        const closestSynthRect = closestSynth!.getBoundingClientRect();
        const closestSynthPosition = {
            x: closestSynthRect.x,
            y: closestSynthRect.y,
        }

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

        const rect = (e.target! as HTMLDivElement).closest(`.patch-point`)!
            .getBoundingClientRect();

        wireStore.startWireDrag(controlId, {
            position: {
                x: position.x + closestSynthPanelPosition.x - closestSynthPosition.x + rect.width / 2 - 9, // Don't ask me, it is some offsets from my very old code
                y: position.y + closestSynthPanelPosition.y - closestSynthPosition.y + rect.height / 2 - 12.5 - 30,
            },
            type,
            controlId,
        }, {
            position: {
                x: position.x + closestSynthPanelPosition.x - closestSynthPosition.x + rect.width / 2 - 9,
                y: position.y + closestSynthPanelPosition.y - closestSynthPosition.y + rect.height / 2 - 12.5 - 30,
            },
            type: 'cursor',
            controlId: null,
        });
    };

    const updateWireToPos = useCallback((e: PointerEvent) => {
        const closestSynth = realRef.current!.closest('.synth');

        if (!closestSynth) {
            console.error('No closest synth found. Patch point should be in the synth.');
            return;
        }

        const closestSynthRect = closestSynth!.getBoundingClientRect();
        const closestSynthPosition = {
            x: closestSynthRect.x,
            y: closestSynthRect.y,
            width: closestSynthRect.width,
            height: closestSynthRect.height,
        }

        wireStore.updateWireTo(draggedWireId!, {
            position: {
                x: e.clientX - closestSynthPosition.x - 9,
                y: e.clientY - closestSynthPosition.y - 12.5 - 30
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

        const closestSynth = realRef.current!.closest('.synth');

        if (!closestSynth) {
            console.error('No closest synth found. Patch point should be in the synth.');
            return;
        }

        const closestSynthRect = closestSynth!.getBoundingClientRect();
        const closestSynthPosition = {
            x: closestSynthRect.x,
            y: closestSynthRect.y,
        }

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

        wireStore.connectWireTo(draggedWireId, {
            position: {
                x: position.x + closestSynthPanelPosition.x - closestSynthPosition.x + rect.width / 2 - 9,
                y: position.y + closestSynthPanelPosition.y - closestSynthPosition.y + rect.height / 2 - 12.5 - 30,
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
