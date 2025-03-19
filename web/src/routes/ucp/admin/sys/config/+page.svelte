<script lang="ts">
	import { Tooltip } from 'flowbite-svelte';
	import { Button } from 'flowbite-svelte';

	let { data } = $props();

	const factions: string[][] = [
		['SCKK', 'TAXI', 'bg-taxi'],
		['TOW', 'TOW', 'bg-tow'],
		['APMS', 'APMS', 'bg-apms']
	];

	let announcement = $state(
		data.config?.global.announcement ? data.config?.global.announcement : null
	);
	let maintenance = $state(
		data.config?.global.maintenance ? data.config?.global.maintenance : null
	);
	let errortext = $state('');
	const handleChange = () => {
		if (announcement === '') {
			announcement = null;
		}
		if (maintenance === '') {
			maintenance = null;
		}
	};
	async function changeState() {
		const post = await fetch('/web-api/sys/global/change', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				announcement: announcement,
				maintenance: maintenance
			})
		});
		if (!post.ok) {
			const text = await post.text();
			errortext = 'Sikertelen módosítás: ' + text;
		}
	}
</script>

<div class="m-5 mx-5 grid grid-cols-2 gap-2 text-center text-white">
	<div class="rounded-lg bg-amber-300">
		<h1 class="mt-2 text-3xl font-bold">Global Config</h1>
		<h2>{errortext}</h2>
		<label for="announcement" class="text-xl font-bold">Hírdetmény: </label>
		<input
			class="rounded-lg bg-black/50 text-center font-bold text-white placeholder:text-white"
			type="text"
			placeholder={!announcement ? 'nincs beállítva' : ''}
			name="announcement"
			onchange={handleChange}
			bind:value={announcement}
		/>
		{#if announcement !== data.config?.global.announcement}
			<Button
				onclick={async () => changeState()}
				class="icon-[material-symbols--save-as] h-6 w-6 rounded-xl bg-white font-bold transition-all duration-150 hover:bg-slate-500"
			></Button>
			<Tooltip class="bg-gray-600">Változás mentése</Tooltip>
		{/if}
		<label for="maintenance" class="text-xl font-bold">Karbantartás: </label>
		<input
			type="text"
			class="mb-2 rounded-lg bg-black/50 text-center font-bold text-white placeholder:text-white"
			name="maintenance"
			placeholder={!maintenance ? 'nincs beállítva' : ''}
			bind:value={maintenance}
			onchange={handleChange}
		/>
		{#if maintenance !== data.config?.global.maintenance}
			<Button
				onclick={async () => changeState()}
				class="icon-[material-symbols--save-as] h-6 w-6 rounded-xl bg-white font-bold transition-all duration-150 hover:bg-slate-500"
			></Button>
			<Tooltip class="bg-gray-600">Változás mentése</Tooltip>
		{/if}
	</div>
	{#each factions as f}
		<div class={`rounded-lg ${f[2]}`}>
			<h1 class="mt-2 text-3xl font-bold">{f[1]} config</h1>
		</div>
	{/each}
</div>
