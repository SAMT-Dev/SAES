<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	let text = $state('Bejelentkezés ellenőrzése');
	let images: string[] = $state([]);
	let image_infos: Record<string, { hash: string; url: string }> = $state({});
	let loaddone = $state(false);
	listen<string>('setmaintext', (ev) => {
		text = ev.payload;
	});
	async function rerender(check: boolean = false) {
		if (check) {
			await invoke('check_hash');
		}
		images = await invoke<string[]>('get_images');
		for (const image of images) {
			let hash = await invoke<string>('get_image_hash', { path: image });
			let src = await invoke<string>('get_image', { path: image });
			image_infos[image] = {
				hash,
				url: src
			};
		}
	}
	onMount(async () => {
		let check = await invoke<boolean>('check_auth');
		if (!check) {
			return await goto('/main/login', { replaceState: true });
		}
		text = 'Frakció ellenőrzése';
		let fcheck = await invoke<boolean>('check_faction');
		if (!fcheck) {
			return await goto('/main/faction', { replaceState: true });
		}
		text = 'Kép hashek betöltése (sok időbe telhet)';
		await invoke('clear_check_hash');
		text = 'Felület betöltése';
		await rerender();
		loaddone = true;
	});
</script>

<div class="bg-gray-950 h-screen w-screen text-white text-center overflow-auto">
	<button
		onclick={async () => {
			rerender(true);
		}}>Újratöltés</button
	>
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
