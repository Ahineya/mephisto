// declare const Faustsynth_and_effects: any;

export const audioContext = new AudioContext();
export const analyser = audioContext.createAnalyser();

// const pluginURL = "audio-engine";
// const plugin = new Faustsynth_and_effects(audioContext, pluginURL);
// export const synth = plugin.load()
//   .then((node: any) => {
//     const synthNode = node;
//
//     console.log(synthNode.getJSON());
//     // Print path to be used with 'setParamValue'
//     console.log(synthNode.getParams());
//     // Connect it to output as a regular WebAudio node
//     synthNode.connect(analyser);
//     analyser.connect(audioContext.destination);
//     return synthNode;
//   });

await audioContext.audioWorklet.addModule('processor.js');
export const synth = new AudioWorkletNode(audioContext, 'mephisto-generator');

synth.port.onmessage = (event) => {
    console.log(event.data);

    const port = synth.port;

    if (event.data.command === 'init') {
        //then event.data.parameters is an array of parameters
        // if parameter.type === 0, then we want to create a button and add it to the dom
        // if parameter.type === 1, then we want to create a slider and add it to the dom

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

        document.body.appendChild(controls);
    }
};

synth.connect(audioContext.destination);

console.log(synth);
