<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { openUrl } from '@tauri-apps/plugin-opener';

	async function beginLogin() {
		let api = await invoke('get_api_url');
		await openUrl(`${api}/auth?mode=app`);
		await invoke('begin_login');
	}

	listen<string>('loginDone', async () => {
		await invoke('save_auth_token');
		return await goto('/main', { replaceState: true });
	});
</script>

<div class="h-screen w-screen bg-gray-950 pointer-events-none select-none text-white text-center">
	<div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
		<h1 class="text-3xl font-bold">Kérlek lépj be újra!</h1>
		<button
			onclick={async () => await beginLogin()}
			class="pointer-events-auto bg-amber-400 px-2 py-1 rounded-xl mt-2 cursor-pointer hover:bg-amber-600 duration-200 transition-all"
			>Belépek</button
		>
	</div>
</div>
