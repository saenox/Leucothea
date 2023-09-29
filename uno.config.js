import { defineConfig } from 'unocss'

export default defineConfig({
    cli: {
        entry: {
            patterns: ['src/**/*.rs'],
            outFile: 'styles.css',
        },
    },
})