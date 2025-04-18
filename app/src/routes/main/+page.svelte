<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { openUrl } from '@tauri-apps/plugin-opener';
	import { relaunch } from '@tauri-apps/plugin-process';
	let page = $state('noconfig');
	let userinfo: { name?: string; admin?: boolean } = $state({});
	let setupStep = $state(0);
	let game_loc = $state('C:\\SeeMTA');

	listen<string>('setmainpage', (ev) => {
		page = ev.payload;
	});
	listen<string>('selectedGameDir', (ev) => {
		game_loc = ev.payload;
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
	async function setGameDir() {
		await invoke('set_game_dir');
	}
	async function saveDir() {
		await invoke('save_game_dir', { dir: game_loc });
		setupStep = 3;
	}
</script>

<div class="bg-gray-950 h-screen w-screen text-white text-center pointer-events-none select-none">
	{#if page === 'nothing'}
		<div class="flex justify-center items-center h-full">
			<img src="/favicon.png" alt="" />
		</div>
	{/if}
	{#if page === 'noconfig'}
		{#if setupStep !== 3}
			<div class="bg-slate-500 w-screen py-1">
				<h1 class="font-bold text-3xl">Konfigurációs mágus</h1>
				<h2>Pár kattintás, és már is élvezheted az app nyújtotta lehetőségeket.</h2>
			</div>
		{/if}
		{#if setupStep === 0}
			<div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[80vw]">
				<h1 class="mt-2 text-5xl font-regular mb-3">Üdv a SAMT App-ban!</h1>
				<h2 class="font-regular mb-2">
					A SAMT App egy új megoldás, amely célja a képek könnyebb feltöltése a weboldalra.
				</h2>
				<h2 class="font-regular mb-2">
					Működésének lényege, hogy a játékban F12-vel készített képeket itt egy kattintással fel
					tudod tölteni.
				</h2>
				<h2 class="font-bold mb-2 text-red-400">
					Az App KIZÁRÓLAGOS feladata a screenshots mappából a képek feltöltése, jelenleg NINCS
					tervben a képek máshonnan való kezelése.
				</h2>
				<button
					onclick={() => (setupStep = 1)}
					class="font-bold uppercase bg-blue-600 text-xl px-2 py-1 rounded-xl hover:bg-blue-800 transition-all duration-200 cursor-pointer pointer-events-auto"
					>Blah blah blah, haladjunk már!</button
				>
			</div>
		{/if}
		{#if setupStep === 1}
			<div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[80vw]">
				<h1 class="font-bold text-2xl mt-2">Lépj be Discordal!</h1>
				<button
					onclick={async () => beginLogin()}
					class="bg-amber-600 font-bold text-xl uppercase pointer-events-auto px-2 py-1 rounded-xl hover:bg-amber-800 transition-all duration-200 cursor-pointer"
				>
					Belépek</button
				>
			</div>
		{/if}
		{#if setupStep === 2}
			<div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[80vw]">
				<h1 class="text-3xl font-bold mt-2">
					Üdv {userinfo.admin ? 'szöszadmin ' : ''}{userinfo.name}!
				</h1>
				<h2>A továbblépéshez kérlek válaszd ki a játékod mappáját!</h2>
				<h2 class="text-gray-300">Amennyiben ez az alap, nem kell megváltoztatnod!</h2>
				<div class="flex mx-auto justify-center items-center mt-4 gap-2 flex-col mb-4">
					<h2 class="bg-gray-700 px-4 py-2 rounded-xl">{game_loc}</h2>
					<button
						class="pointer-events-auto uppercase bg-orange-600 font-bold text-md px-2 py-1 rounded-xl hover:bg-orange-800 transition-all duration-200 cursor-pointer"
						onclick={async () => setGameDir()}>nekem máshol van, mutatom</button
					>
				</div>
				<button
					class="pointer-events-auto uppercase bg-emerald-600 text-xl px-2 py-1 rounded-xl hover:bg-emerald-800 transition-all duration-200 cursor-pointer font-bold"
					onclick={async () => saveDir()}>Tovább, haladjunk már!</button
				>
			</div>
		{/if}
		{#if setupStep === 3}
			<div class="flex h-screen">
				<div class="m-auto">
					<h1 class="text-3xl font-bold">Kész is vagyunk!</h1>
					<h2 class="text-gray-400 mb-4">Egy kattintás és használhatod az appot!</h2>
					<button
						onclick={async () => invoke('done_setup')}
						class="text-xl font-bold pointer-events-auto uppercase bg-emerald-600 px-4 py-2 rounded-xl hover:bg-emerald-800 transition-all duration-200 cursor-pointer"
						>Na végre, azt hittem már szakállam fog nőni!</button
					>
				</div>
			</div>
		{/if}
	{/if}
</div>
