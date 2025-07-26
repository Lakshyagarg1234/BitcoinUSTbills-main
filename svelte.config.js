import adapter from '@sveltejs/adapter-static';
import autoprefixer from 'autoprefixer';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

const config = {
	preprocess: vitePreprocess({
		postcss: {
			plugins: [autoprefixer]
		}
	}),
	kit: {
		adapter: adapter(),
		files: {
			assets: 'src/frontend/static',
			lib: 'src/frontend/src/lib',
			routes: 'src/frontend/src/routes',
			appTemplate: 'src/frontend/src/app.html',
		}
	},
};
export default config;
