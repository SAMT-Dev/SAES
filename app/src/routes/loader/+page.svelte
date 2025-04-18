<script lang="ts">
	import { check } from '@tauri-apps/plugin-updater';
	import { invoke } from '@tauri-apps/api/core';
	import { getVersion } from '@tauri-apps/api/app';
	import { relaunch } from '@tauri-apps/plugin-process';
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	let text = $state('Frissítések keresése');
	let ver = $state('');
	let envserr = $state(false);

	listen<string>('setloadertext', (ev) => {
		text = ev.payload;
	});

	onMount(async () => {
		ver = await getVersion();
		setTimeout(async () => {
			const update = await check();
			if (update) {
				text = 'Frissítés találva';
				console.log(`found update ${update.version} from ${update.date} with notes ${update.body}`);
				let downloaded = 0;
				let contentLength = 0;
				// alternatively we could also call update.download() and update.install() separately
				await update.downloadAndInstall((event) => {
					switch (event.event) {
						case 'Started':
							contentLength = event.data.contentLength!;
							console.log(`started downloading ${event.data.contentLength} bytes`);
							break;
						case 'Progress':
							downloaded += event.data.chunkLength;
							text = 'Letöltés: ' + Math.round((downloaded / contentLength) * 100) + '%';
							console.log(`downloaded ${downloaded} from ${contentLength}`);
							break;
						case 'Finished':
							console.log('download finished');
							text = 'Telepítés';
							break;
					}
				});

				console.log('update installed');
				text = 'Frissítés kész, újraindítás';
				await relaunch();
			}
			text = 'ENV ellenőrzése';
			let envs: string = await invoke('check_envs');
			if (envs === 'ok') {
				text = 'App indítása';
				setTimeout(() => {
					invoke('update_done');
				}, 500);
			} else if (envs === 'multiple') {
				envserr = true;
				text = '';
			} else {
				await relaunch();
			}
		}, 300);
	});
</script>

<div
	class="pointer-events-none select-none justify-center items-center text-center bg-gray-900 w-screen h-screen"
>
	<div class="flex flex-col absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
		<img src="/favicon.png" class="w-[200px] m-auto" alt="" />
		<h1 class="font-bold text-3xl text-white w-screen">SAMT App</h1>
		<h2 class="text-gray-300 font-light">{text}</h2>
		{#if envserr}
			<h2 class="text-red-500 font-light">.env beolvasása sikertelen.</h2>
		{/if}
	</div>
	<h2 class="text-gray-400 absolute bottom-0 left-1">v{ver}</h2>
</div>
