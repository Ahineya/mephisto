export type Wire = {
    uuid: string;
    from: WireConnectionPoint;
    to: WireConnectionPoint;
    color: string;

    connected?: boolean;
}

export type WireConnectionPoint = {
    position: {
        x: number;
        y: number;
    };
    type: 'cursor' | 'input' | 'output';
    controlId: string | null;
}
