import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react'
import basicSsl from '@vitejs/plugin-basic-ssl'
import { watch } from "vite-plugin-watch"

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        basicSsl(),
        react(),
        watch({
            pattern: "src/audioengine/**/*.mephisto",
            command: "cd src/audioengine && mephisto -i synth.mephisto -o ../../public/processor.js",
        }),
    ],
})
