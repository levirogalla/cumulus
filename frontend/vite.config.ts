import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, type PluginOption } from 'vite';

const fullReloadAlways: PluginOption = {
  name: 'full-reload-always',
  handleHotUpdate({ server }) {
    server.ws.send({
      type: 'full-reload',
    });
    return []; // Returning an empty array prevents other HMR handlers from processing the update
  },
};

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [sveltekit(), tailwindcss(), fullReloadAlways], // Add the custom plugin here
});
