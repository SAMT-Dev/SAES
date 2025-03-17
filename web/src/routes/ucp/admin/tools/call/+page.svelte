<script lang="ts">
	import { Dropzone, Label, Checkbox, Tooltip } from 'flowbite-svelte';

	let files: FileList | undefined = $state();
	interface Drivers {
		[key: string]: Calls;
	}
	interface Calls {
		[key: string]: Date;
	}
	let calls: Drivers = $state({});
	let active = $state(true);
	let samecall = $state(false);
	const zoneChange = async () => {
		active = false;
		if (files?.length === 1) {
			samecall = true;
		}
		for (const file of files!) {
			let lines = (await file.text()).split('\n');
			for (const line of lines) {
				let elements = line.split(' ');
				if (
					(elements.includes('TAXI') || elements.includes('TOW')) &&
					elements.includes('elfogadta')
				) {
					// [2024-11-03 17:50:37] [Output] : [SeeMTA - Tablet (Taxi)]: Suarez Filipe / TAXI elfogadta a következő hívást: 81
					let date = new Date(`${elements[0].slice(1)} ${elements[1].slice(0, -1)}`);
					let call = line.split(':')[5].slice(1).slice(0, -1);
					let driver = line.split(':')[4].slice(1).split('/')[0].slice(0, -1);
					if (!calls[driver]) {
						calls[driver] = {};
					}
					if (!calls[driver][call]) {
						calls[driver][call] = date;
					} else if (calls[driver][call] !== date && samecall) {
						calls[driver][`${call} ${date.getTime()}`] = date;
					}
				}
			}
		}
		console.log(calls);
	};
</script>

<div class="mt-5 items-center justify-center text-center text-black dark:text-white">
	<h1 class=" mb-2 text-3xl font-bold">SCKK Log alapú hívás számláló</h1>
	{#if active}
		<div class="mb-2 mt-1 flex items-center justify-center gap-2">
			<Checkbox bind:checked={samecall} />
			<h2>Ismétlődő hívásszám engedélyezése</h2>
			<h2 class="text-taxi font-bold">?</h2>
			<Tooltip class="bg-gray-600"
				>Ez abban az esetben hasznos, mikor egy hívás számot egy ember lehet többször is elvitt. Egy
				fájl esetében automatikusan engedélyezve van, több fájlnál csak azonos embertől ajánlott.</Tooltip
			>
		</div>
		<Label class="text-black dark:text-white">Kattints a gombra, hogy kiválaszd a fájlokat</Label>
		<Dropzone
			class="m-auto h-12 w-96"
			multiple={true}
			accept=".log"
			on:change={zoneChange}
			bind:files
		/>
	{:else}
		<h2>
			Ha egy hívásszám után egy szép hosszú szám van, akkor az egy meglévő szám későbbi elvitele.
		</h2>
		{#each Object.keys(calls) as driver}
			<div class="mb-4">
				<h1 class="text-xl font-bold">{driver}</h1>
				<h2>hívásai ({Object.keys(calls[driver]).length} összesen):</h2>
				{#each Object.keys(calls[driver]) as call}
					<h2>{call}: {calls[driver][call]}</h2>
				{/each}
			</div>
		{/each}
	{/if}
</div>
