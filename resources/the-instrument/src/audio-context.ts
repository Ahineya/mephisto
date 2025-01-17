// declare const Faustsynth_and_effects: any;

import {synthStore} from "./stores/synth.store.ts";

export const audioContext = new AudioContext();
// export const analyser = audioContext.createAnalyser();

await audioContext.audioWorklet.addModule('processor.js');
audioContext.suspend();
export const synthNode = new AudioWorkletNode(audioContext, 'mephisto-generator', {
    outputChannelCount: [2],
});

type Connection = {
    path: string[];
    io: string;
};

function parseConnection(str: string): Connection {
    const parts = str.split('#');
    const io = parts.pop()!;
    const path = parts.length > 0 ? parts : ['Head'];

    return { path, io };
}

function toMermaid(connections: string[]): string {
    const mm = connections.map(connection => {
        const [outputStr, inputStr] = connection.split(' -> ').map(str => {
            const { path, io } = parseConnection(str);
            return { node: path.join(''), io, label: path.join('#') };
        });

        return `${outputStr.node} -->|"${outputStr.io} > ${inputStr.io}"| ${inputStr.node}[${inputStr.label}]`;
    }).join('\n');

    return `graph LR\n${mm}`;
}

synthNode.port.onmessage = (event) => {
    const port = synthNode.port;

    if (event.data.command === 'init') {
        const controls = document.createElement('div');

        event.data.parameters.forEach((parameter: any) => {
            if (parameter.type === 0) {
                const button = document.createElement('button');
                button.innerText = parameter.name;
                button.addEventListener('mousedown', () => {
                    if (!port) {
                        return;
                    }

                    port.postMessage({
                        command: 'setParameter',
                        setter: {
                            name: parameter.name,
                            value: 1
                        }
                    })
                });

                button.addEventListener('mouseup', () => {
                    if (!port) {
                        return;
                    }

                    port.postMessage({
                        command: 'setParameter',
                        setter: {
                            name: parameter.name,
                            value: 0
                        }
                    })
                });

                controls.appendChild(button);
            } else if (parameter.type === 1) {
                const sliderLabel = document.createElement('label');
                const slider = document.createElement('input');
                slider.type = 'range';
                slider.min = parameter.min;
                slider.max = parameter.max;
                slider.step = parameter.step;
                slider.value = parameter.initial;
                slider.id = parameter.name;

                // create number input
                const numberInput = document.createElement('input');
                numberInput.type = 'number';
                numberInput.min = parameter.min;
                numberInput.max = parameter.max;
                numberInput.step = parameter.step;
                numberInput.value = parameter.initial;
                numberInput.id = `${parameter.name}_input`;

                slider.addEventListener('input', (event) => {
                    if (!port) {
                        return;
                    }

                    port.postMessage({
                        command: 'setParameter',
                        setter: {
                            name: parameter.name,
                            value: +(event.target as any).value
                        }
                    });

                    numberInput.value = (event.target as any).value;
                });

                sliderLabel.innerText = parameter.name;
                sliderLabel.appendChild(slider);

                numberInput.addEventListener('input', (event) => {
                    if (!port) {
                        return;
                    }

                    port.postMessage({
                        command: 'setParameter',
                        setter: {
                            name: parameter.name,
                            value: +(event.target as any).value
                        }
                    })

                    slider.value = (event.target as any).value;
                });

                sliderLabel.appendChild(numberInput);
                controls.appendChild(sliderLabel);
            } else if (parameter.type === 2) {
                // Type 2 is toggle
                const toggleLabel = document.createElement('label');
                const toggle = document.createElement('input');
                toggle.type = 'checkbox';
                toggle.checked = parameter.initial;
                toggle.id = parameter.name;

                toggle.addEventListener('change', (event) => {
                    if (!port) {
                        return;
                    }

                    port.postMessage({
                        command: 'setParameter',
                        setter: {
                            name: parameter.name,
                            value: +(event.target as any).checked
                        }
                    })
                });

                toggleLabel.innerText = parameter.name;
                toggleLabel.appendChild(toggle);
                controls.appendChild(toggleLabel);
            }
        })

        let connections = event.data.connections.map(([out, inp]: [number, number]) => {
            return `${event.data.outputNames[out]} -> ${event.data.inputNames[inp]}`;
        });

        console.log('Inputs', event.data.inputNames);
        console.log('Outputs', event.data.outputNames);

        const parameterNames = event.data.parameters.map((parameter: any) => parameter.name);
        console.log('Parameters', parameterNames);

        const chart = toMermaid(connections);

        synthStore.setChart(chart);
        synthStore.setInputs(event.data.inputNames);
        synthStore.setOutputs(event.data.outputNames);

        synthStore.setIsLoaded(true);

        const container = document.querySelector('#container');

        if (!container) {
            return;
        }

        container!.appendChild(controls);
    }

    if (event.data.command === 'connectionsChanged') {
        console.log('Connections changed', event.data.connections);
        let connections = event.data.connections.map(([out, inp]: [number, number]) => {
            return `${event.data.outputNames[out]} -> ${event.data.inputNames[inp]}`;
        });

        const chart = toMermaid(connections);

        synthStore.setChart(chart);
    }
};

synthNode.connect(audioContext.destination);

class SynthFacade {
    private _port: MessagePort;

    constructor(port: MessagePort) {
        this._port = port;
    }

    init() {
        this._port.postMessage({
            command: 'init'
        });
    }

    setParameter(name: string, value: number) {
        this._port.postMessage({
            command: 'setParameter',
            setter: {
                name,
                value
            }
        });
    }

    connect(output: string, input: string) {
        const outputIndex = synthStore.getNumericOutput(output);
        const inputIndex = synthStore.getNumericInput(input);

        if (outputIndex === -1 || inputIndex === -1) {
            return;
        }

        this._port.postMessage({
            command: 'addConnection',
            connection: [outputIndex, inputIndex]
        });
    }

    disconnect(output: string, input: string) {
        const outputIndex = synthStore.getNumericOutput(output);
        const inputIndex = synthStore.getNumericInput(input);

        if (outputIndex === -1 || inputIndex === -1) {
            return;
        }

        this._port.postMessage({
            command: 'removeConnection',
            connection: [outputIndex, inputIndex]
        });
    }
}

export const synthFacade = new SynthFacade(synthNode.port);

console.log(synthNode);
