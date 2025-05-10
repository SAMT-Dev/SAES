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
				taxi: '#fece01'
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
