<script lang="ts">
	import { Checkbox, Select, Tooltip } from 'flowbite-svelte';
	import { Button } from 'flowbite-svelte';

	let { data } = $props();

	const factions: string[][] = [
		['SCKK', 'TAXI', 'bg-taxi'],
		['TOW', 'TOW', 'bg-tow'],
		['APMS', 'APMS', 'bg-apms']
	];

	let localfacts = $state(data.config?.factions);

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
	async function saveFact(faction: string) {
		const post = await fetch(`/web-api/sys/faction/${faction}`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				shift_access: localfacts![faction].shift_access,
				access: localfacts![faction].access,
				site_access: localfacts![faction].site_access
			})
		});
		if (!post.ok) {
			const text = await post.text();
			errortext = 'Sikertelen módosítás: ' + text;
		} else {
			alert('Sikeres módosítás!');
		}
	}
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
		} else {
			alert('Sikeres módosítás!');
		}
		data.config!.global.announcement = announcement!;
		data.config!.global.maintenance = maintenance!;
	}
</script>

<div class="m-5 mx-5 flex flex-col gap-2 text-center text-white">
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
	{#if localfacts}
		{#each factions as f}
			<div class={`rounded-lg ${f[2]}`}>
				<h1 class="mb-3 mt-2 text-3xl font-bold">{f[1]} config</h1>
				<div class="mx-20 flex">
					<label for="shift_access">Műszakok közötti elemmegosztás</label>
					<Select bind:value={localfacts[f[0]].shift_access} class="bg-gray-700">
						<option value="SameShift">Ki</option>
						<option value="OtherManager">Műszakvezetők</option>
						<option value="OtherShift">Mindenki</option>
					</Select>
				</div>
				<div class="flex flex-row items-center justify-center gap-2">
					<div>
						<h1 class="mb-2 mt-3 text-xl font-bold">Elemjogosultságok</h1>
						<div class="mx-20 flex items-center justify-center gap-2">
							<label for={`${f[0]}-supplements`}>Pótlékok</label>
							<Select
								class="bg-gray-700 text-center"
								bind:value={localfacts[f[0]].access.supplements}
								name={`${f[0]}-supplements`}
							>
								<option value="None">nincs</option>
								<option value="Read">Olvasás</option>
								<option value="Write">Írás</option></Select
							>
						</div>
						<div class="mx-20 flex items-center justify-center gap-2">
							<label for={`${f[0]}-hails`}>Leintések</label>
							<Select
								class="bg-gray-700 text-center"
								bind:value={localfacts[f[0]].access.hails}
								name={`${f[0]}-hails`}
							>
								<option value="None">nincs</option>
								<option value="Read">Olvasás</option>
								<option value="Write">Írás</option>
							</Select>
						</div>
						<div class="mx-20 flex items-center justify-center gap-2">
							<label for={`${f[0]}-bills`}>Számlák</label>
							<Select
								class="bg-gray-700 text-center"
								bind:value={localfacts[f[0]].access.bills}
								name={`${f[0]}-bills`}
							>
								<option value="None">nincs</option>
								<option value="Read">Olvasás</option>
								<option value="Write">Írás</option>
							</Select>
						</div>
					</div>
					<div class="mb-5">
						<h1 class="mb-2 mt-3 text-xl font-bold">Weboldaljogosultságok</h1>
						<div class="mx-20 flex items-center justify-center gap-2">
							<label for={`${f[0]}-ucp`}>UCP</label>
							<Checkbox bind:checked={localfacts[f[0]].site_access.ucp} name={`${f[0]}-ucp`} />
						</div>
						<div class="mx-20 flex items-center justify-center gap-2">
							<label for={`${f[0]}-admin`}>Admin</label>
							<Checkbox bind:checked={localfacts[f[0]].site_access.admin} name={`${f[0]}-admin`} />
						</div>
						<div class="mx-20 flex items-center justify-center gap-2">
							<label for={`${f[0]}-shift`}>Shift</label>
							<Checkbox bind:checked={localfacts[f[0]].site_access.shift} name={`${f[0]}-shift`} />
						</div>
						<div class="mx-20 flex items-center justify-center gap-2">
							<label for={`${f[0]}-fleet`}>Fleet</label>
							<Checkbox bind:checked={localfacts[f[0]].site_access.fleet} name={`${f[0]}-fleet`} />
						</div>
						<div class="mx-20 flex items-center justify-center gap-2">
							<label for={`${f[0]}-faction`}>Faction</label>
							<Checkbox
								bind:checked={localfacts[f[0]].site_access.faction}
								name={`${f[0]}-faction`}
							/>
						</div>
					</div>
				</div>
				<button
					class="mb-3 cursor-pointer rounded-xl bg-green-300 px-2 py-1 text-black transition-all duration-300 hover:bg-green-700 hover:text-white"
					onclick={async () => saveFact(f[0])}>Frakciómódosítás elmentése</button
				>
			</div>
		{/each}
	{/if}
</div>
