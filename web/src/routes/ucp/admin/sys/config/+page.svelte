<script lang="ts">
	import { Tooltip } from 'flowbite-svelte';
	import { Button } from 'flowbite-svelte';

	let { data } = $props();

	let announcement = $state(
		data.config?.global.announcement ? data.config?.global.announcement : null
	);
	let maintenance = $state(
		data.config?.global.maintenance ? data.config?.global.maintenance : null
	);

	const handleChange = () => {
		if (announcement === '') {
			announcement = null;
		}
		if (maintenance === '') {
			maintenance = null;
		}
	};
</script>

<div class="m-5 grid grid-cols-2 gap-2 text-center text-white">
	<div class="rounded-lg bg-amber-300">
		<h1 class="mt-2 text-3xl font-bold">Global Config</h1>
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
				class="icon-[material-symbols--save-as] h-6 w-6 rounded-xl bg-white font-bold transition-all duration-150 hover:bg-slate-500"
			></Button>
			<Tooltip>Változás mentése</Tooltip>
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
				class="icon-[material-symbols--save-as] h-6 w-6 rounded-xl bg-white font-bold transition-all duration-150 hover:bg-slate-500"
			></Button>
			<Tooltip>Változás mentése</Tooltip>
		{/if}
	</div>
</div>
