import flowbite from 'flowbite/plugin';
import { addDynamicIconSelectors } from '@iconify/tailwind';

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite/**/*.js'],
	theme: {
		extend: {
			fontFamily: {
				sans: ['Poppins'],
				itim: ['Itim']
			},
			colors: {
				taxi: '#fece01',
				apms: '#d8b47e',
				tow: '#3498db',
				uni: '#08c25c',
				uni2: '#18653b',
				uni3: '#0a4022'
			},
			backgroundSize: {
				'size-200': '200% 200%'
			},
			backgroundPosition: {
				'pos-0': '0% 0%',
				'pos-100': '100% 100%'
			}
		}
	},
	plugins: [
		({ addVariant }) => {
			addVariant('child', '& > *');
			addVariant('child-hover', '& > *:hover');
		},
		flowbite,
		addDynamicIconSelectors()
	]
};
