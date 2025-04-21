<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	let text = $state('');
	let images: string[] = $state([]);
	let image_infos: Record<string, { hash: string; url: string }> = $state({});
	let loaddone = $state(false);
	listen<string>('setmaintext', (ev) => {
		text = ev.payload;
	});
	onMount(async () => {
		text = 'Kép hashek betöltése (sok időbe telhet)';
		await invoke('clear_check_hash');
		text = 'Felület betöltése';
		images = await invoke<string[]>('get_images');
		for (const image of images) {
			let hash = await invoke<string>('get_image_hash', { path: image });
			let src = await invoke<string>('get_image', { path: image });
			image_infos[image] = {
				hash,
				url: src
			};
		}
		loaddone = true;
	});
</script>

<div class="bg-gray-950 h-screen w-screen text-white text-center overflow-auto">
	{#if !loaddone}
		<div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[80vw]">
			<h1>{text}</h1>
		</div>
	{:else}
		{#each images as image}
			<div>
				<h1>{image}</h1>
				<h2>{image_infos[image].hash}</h2>
				<img src={`data:image/png;base64,${image_infos[image].url}`} alt="" />
			</div>
		{/each}
	{/if}
</div>
