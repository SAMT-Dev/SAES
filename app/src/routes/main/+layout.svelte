<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { LayoutData } from './$types';
	import { listen } from '@tauri-apps/api/event';

	let { data, children }: { data: LayoutData; children: Snippet } = $props();

	listen<string>('loginFailed', (ev) => {
		let err = ev.payload;
		if (err === 'noperms') {
			alert('Nincs jogod belépni!');
		}
		if (err.startsWith('unknown')) {
			if (err === 'unknown') {
				alert('Ismeretlen hiba miatt nem tudsz belépni!');
			} else {
				alert('Egy hiba miatt nem tudsz belépni: "' + err.split('/')[1] + '".');
			}
		}
	});
</script>

{@render children()}
