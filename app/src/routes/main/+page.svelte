<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	let page = $state('noconfig');
	listen<string>('setmainpage', (ev) => {
		page = ev.payload;
	});
	async function beginLogin() {
		await invoke('begin_login');
	}
</script>

<div class="bg-gray-950 h-screen w-screen text-white text-center pointer-events-none select-none">
	{#if page === 'nothing'}
		<div class="flex justify-center items-center h-full">
			<img src="/favicon.png" alt="" />
		</div>
	{/if}
	{#if page === 'noconfig'}
		<div class="bg-slate-500 w-screen py-1">
			<h1 class="font-bold text-3xl">Konfigurációs mágus</h1>

			<h2>Pár kattintás, és már is élvezheted az app nyújtotta lehetőségeket.</h2>
		</div>
		<h1 class="font-bold text-2xl mt-2">1. lépés: Lépj be Discordal</h1>
		<button onclick={async () => beginLogin()} class="bg-amber-400 pointer-events-auto"
			>Belépek</button
		>
	{/if}
</div>
