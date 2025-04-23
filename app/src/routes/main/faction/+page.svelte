<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	let facts: string[] = $state([]);
	onMount(async () => {
		facts = await invoke<string[]>('get_faction_options');
	});
</script>

<div class="h-screen w-screen bg-gray-950 pointer-events-none select-none text-white text-center">
	<div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
		<h1 class="text-3xl font-bold">Kérlek lépj be újra!</h1>
		{#each facts as f}
			<h1>{f}</h1>
		{/each}
	</div>
</div>
