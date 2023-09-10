import {FC, useCallback, useEffect, useState} from "react";
import "./knob.scss";
import classNames from "classnames";
import {KnobSize} from "./knob.interface";
import {createConversionFunctions} from "../../../../helpers/linear-conversion.ts";
import {dragStore} from "../../../../stores/drag.store.ts";
import {Tooltip} from "../../../tooltip/tooltip.tsx";

type IProps = {
    id: string;
    caption: string;
    filled?: boolean;

    onValueIndexChanged: (valueIndex: number) => void;
    values: number[];
    valueIndex: number;
    from: number;
    to: number;

    displayValue?: string;

    size?: KnobSize;

    conversionType?: 'linear' | 'exponential';

    disableDisplayValue?: boolean;

    position?: {
        x: number;
        y: number;
    }
}

export const KnobPredefined: FC<IProps> = ({
                                   id,
                                   filled,
                                   values,
                                   from,
                                   to,
                                   valueIndex,
                                   onValueIndexChanged,
                                   caption,
                                   displayValue,
                                   size,
                                   conversionType,
                                   disableDisplayValue,
                                   position = {x: 0, y: 0}
                               }) => {

    const [isMoving, setIsMoving] = useState(false);
    const [isHover, setIsHover] = useState(false);
    const [knobElement, setKnobElement] = useState<HTMLElement | null>(null);
    const [distance, setDistance] = useState(0);

    const {
        convertFrom
    } = createConversionFunctions(conversionType || 'linear', from, to, 0, 270);

    const threshold = Math.round(271 / (values.length - 1)) / 2;

    const rotation = convertFrom(values[valueIndex]) - 135;

    const rotate = useCallback((diff: number) => {
        setDistance(distance + diff);

        if (distance + diff < threshold + 1 && distance + diff > -threshold - 1) {
            return;
        }

        if (distance < 0 && valueIndex < values.length - 1) {
            onValueIndexChanged(valueIndex + 1);
        }

        if (distance > 0 && valueIndex > 0) {
            onValueIndexChanged(valueIndex - 1);
        }

        setDistance(0);
    }, [from, to, rotation, distance]);

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

        {
            disableDisplayValue || <Tooltip show={isHover || isMoving} referenceElement={knobElement as HTMLElement}>
                {displayValue || rotation}
            </Tooltip>
        }
    </div>
}