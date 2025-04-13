<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { openUrl } from '@tauri-apps/plugin-opener';
	let page = $state('noconfig');
	let userinfo: { name?: string; admin?: boolean } = $state({});
	let setupStep = $state(1);
	listen<string>('setmainpage', (ev) => {
		page = ev.payload;
	});
	async function beginLogin() {
		let api = await invoke('get_api_url');
		await openUrl(`${api}/auth?mode=app`);
		await invoke('begin_login');
	}
	listen<string>('loginDone', (ev) => {
		let infos = ev.payload.split('-');
		userinfo.name = infos[0];
		userinfo.admin = infos[1] == 'true' ? true : false;
		setupStep = 2;
	});
	listen<string>('loginFailed', (ev) => {
		let err = ev.payload;
		if (err === 'noperms') {
			alert('Nincs jogod belépni!');
		}
		if (err === 'unknown') {
			alert('Ismeretlen hiba miatt nem tudsz belépni!');
		}
	});
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
		{#if setupStep === 1}
			<h1 class="font-bold text-2xl mt-2">1. lépés: Lépj be Discordal</h1>
			<button onclick={async () => beginLogin()} class="bg-amber-400 pointer-events-auto"
				>Belépek</button
			>
		{/if}
		{#if setupStep === 2}
			<h1 class="text-3xl font-bold">
				Üdv {userinfo.admin ? 'Szöszadmin ' : ''}{userinfo.name}!
			</h1>
		{/if}
	{/if}
</div>
