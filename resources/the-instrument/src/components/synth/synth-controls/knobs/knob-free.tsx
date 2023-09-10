import {useCallback, useEffect, useState} from "react";
import "./knob.scss";
import classNames from "classnames";
import {KnobSize} from "./knob.interface";
import {dragStore} from "../../../../stores/drag.store.ts";
import {createConversionFunctions} from "../../../../helpers/linear-conversion.ts";
import {Tooltip} from "../../../tooltip/tooltip.tsx";

interface IProps {
    id: string;
    caption: string;
    filled?: boolean;

    onValueChanged: (value: number) => void;
    value: number;
    from: number;
    to: number;

    displayValue?: string;

    size?: KnobSize;

    conversionType?: 'linear' | 'exponential';

    position?: {
        x: number;
        y: number;
    }
}

export const KnobFree = ({
                             id,
                             filled,
                             value,
                             from,
                             to,
                             onValueChanged,
                             caption,
                             displayValue,
                             size,
                             conversionType,
                             position = {x: 0, y: 0}
                         }: IProps) => {

    const [isMoving, setIsMoving] = useState(false);
    const [isHover, setIsHover] = useState(false);
    const [knobElement, setKnobElement] = useState<HTMLElement | null>(null);

    const {
        convertFrom,
        convertTo
    } = createConversionFunctions(conversionType || 'linear', from, to, 0, 270);

    const rotation = convertFrom(value) - 135;

    const rotate = useCallback((diff: number) => {
        const converted = convertTo(rotation - diff + 135);

        if (converted < from) {
            onValueChanged(from);
            return;
        }

        if (converted > to) {
            onValueChanged(to);
            return;
        }

        onValueChanged(converted);

    }, [onValueChanged, from, to, rotation]);

    useEffect(() => {
        const subscriptions = [
            dragStore.onDragStart.subscribe(draggingId => {
                setIsMoving(draggingId === id);
            }),
            dragStore.onDragMoveDiffChanged.subscribe((diff: number) => {
                if (isMoving) {
                    rotate(diff);
                }
            })
        ];

        return () => subscriptions.forEach(s => s.unsubscribe());
    }, [isMoving, rotation, from, to, id, rotate]);

    const setDragStart = () => {
        dragStore.startDrag(id);
    }

    return <div
        className={classNames("knob", {active: isMoving})}
        style={{
            left: position.x,
            top: position.y
        }}
    >
        <div
            className={classNames("knob-circle-container", {active: isMoving, [`knob-size-${size || 'medium'}`]: true})}
            ref={setKnobElement} onMouseEnter={() => setIsHover(true)}
            onMouseLeave={() => setIsHover(false)}>
            <div className={classNames("knob-circle", {"knob-circle-filled": filled})} onMouseDown={setDragStart}
                 style={{transform: `rotate(${rotation}deg)`}}>
                <div className="knob-marker"/>
            </div>
        </div>

        <div className="knob-caption">
            {caption}
        </div>

        <Tooltip show={isHover || isMoving} referenceElement={knobElement as HTMLElement}>
            {displayValue || rotation}
        </Tooltip>
    </div>
}