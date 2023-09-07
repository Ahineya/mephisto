import Victor from 'victor';
import './wire.scss';
import {CSSProperties, FC} from "react";

function getSlumpPos(vec1: Victor, vec2: Victor, loose: number) {
    const dist = vec1.distance(vec2);
    const avg = vec1.clone()
        .add(vec2)
        .divideScalar(2);
    avg.y += ((1 - 0.5) * 150 + dist) * loose;
    return avg;
}

type IProps = {
    from: { x: number; y: number };
    to: { x: number; y: number };
    color: string;
    loose: number;
    opacity: number;
    connected: boolean;
}

export const Wire: FC<IProps> = ({
                                     from,
                                     to,
                                     color,
                                     loose,
                                     opacity,
                                     connected
                                 }) => {
    const fromV = new Victor(from.x, from.y);
    const toV = new Victor(to.x, to.y);

    const slumpPos = getSlumpPos(fromV, toV, loose);

    return (
        <>
            <g className="wire" style={{
                "--wire-opacity": opacity,
                "--wire-events-fill": connected ? "fill" : "none",
                "--wire-events-stroke": connected ? "stroke" : "none",
            } as CSSProperties}>
                <path className="wire-wire" d={`M${fromV.x},${fromV.y} Q${slumpPos.x},${slumpPos.y} ${toV.x},${toV.y}`}
                      stroke={color} strokeWidth="6"/>
                <circle className="wire-point-wrap" cx={fromV.x} cy={fromV.y} r="8" stroke={color} strokeWidth="6"/>
                <circle className="wire-point" cx={fromV.x} cy={fromV.y} r="6" fill="black"/>
                <circle className="wire-point-wrap" cx={toV.x} cy={toV.y} r="8" stroke={color} strokeWidth="6"/>
                <circle className="wire-point" cx={toV.x} cy={toV.y} r="6" fill="black"/>
            </g>
        </>
    );
}
