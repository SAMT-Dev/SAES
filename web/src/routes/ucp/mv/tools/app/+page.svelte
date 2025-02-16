<script lang="ts">
	import { DateTime } from 'luxon';
	import { Select, Textarea } from 'flowbite-svelte';
	let code = $state('');
	interface muszak {
		[key: string]: number;
	}
	interface allmuszak {
		[key: string]: {
			a: number;
			b: number;
			n: number;
		};
	}
	let n: muszak = $state({});
	let a: muszak = $state({});
	let b: muszak = $state({});
	let all: allmuszak = $state({});
	let hivasok = $state([0, 0, 0]);
	let time_format = $state('yyyy. MM. dd. HH:mm:ss');
	let timeerror = $state(false);
	const handle = () => {
		timeerror = false;
		n = {};
		a = {};
		b = {};
		all = {};
		hivasok = [0, 0, 0];
		let rows = code.split('\n');
		for (let row of rows) {
			if (row.length > 0) {
				let row_g = row.split('\t');
				let unstable_time = DateTime.fromFormat(row_g[3], time_format).toMillis();
				if (isNaN(unstable_time)) {
					timeerror = true;
				}
				let call_date = new Date(unstable_time);
				if (call_date.getHours() < 15) {
					if (row_g[0] !== 'Lemondott') {
						hivasok[2]++;
					}
					if (Object.keys(n).includes(row_g[0])) {
						n[row_g[0]]++;
					} else {
						n[row_g[0]] = 1;
					}
					if (Object.keys(all).includes(row_g[0])) {
						all[row_g[0]].n++;
					} else {
						all[row_g[0]] = {
							a: 0,
							b: 0,
							n: 1
						};
					}
				}
				if (
					call_date.getHours() >= 15 &&
					(call_date.getHours() < 18 ||
						(call_date.getHours() === 18 && call_date.getMinutes() < 30))
				) {
					if (row_g[0] !== 'Lemondott') {
						hivasok[0]++;
					}
					if (Object.keys(a).includes(row_g[0])) {
						a[row_g[0]]++;
					} else {
						a[row_g[0]] = 1;
					}
					if (Object.keys(all).includes(row_g[0])) {
						all[row_g[0]].a++;
					} else {
						all[row_g[0]] = {
							a: 1,
							b: 0,
							n: 0
						};
					}
				}
				if (
					(call_date.getHours() > 18 ||
						(call_date.getHours() === 18 && call_date.getMinutes() >= 30)) &&
					call_date.getHours() < 22
				) {
					if (row_g[0] !== 'Lemondott') {
						hivasok[1]++;
					}
					if (Object.keys(b).includes(row_g[0])) {
						b[row_g[0]]++;
					} else {
						b[row_g[0]] = 1;
					}
					if (Object.keys(all).includes(row_g[0])) {
						all[row_g[0]].b++;
					} else {
						all[row_g[0]] = {
							a: 0,
							b: 1,
							n: 0
						};
					}
				}
				if (call_date.getHours() > 22) {
					if (row_g[0] !== 'Lemondott') {
						hivasok[2]++;
					}
					if (Object.keys(n).includes(row_g[0])) {
						n[row_g[0]]++;
					} else {
						n[row_g[0]] = 1;
					}
					if (Object.keys(all).includes(row_g[0])) {
						all[row_g[0]].n++;
					} else {
						all[row_g[0]] = {
							a: 0,
							b: 0,
							n: 1
						};
					}
				}
			}
		}
	};
</script>

<div class="flex text-center text-white">
	<div class="m-auto">
		{#if timeerror}
			<h1 class="text-xl font-bold text-red-600">
				Időkezelés hiba, problálj meg egy másik időformátumot, ha egyik se megy keress fel
				Discordon! (@hvcsano)
			</h1>
		{/if}
		<h1 class="mt-4 text-3xl font-bold">APP Feldolgozó</h1>
		<h2>
			Illeszd be az app kódját ide, <a href="/app.mp4" class="text-taxi" target="_blank">így</a> (kiválasztasz
			egyet, majd CTRL+A és CTRL+C):
		</h2>
		<Select
			bind:value={time_format}
			class="mb-3"
			placeholder="Kérlek válassz: (2024 április elseje, 15 óra 12 perc 22 másodperc a példákban az idő!)"
		>
			<option value="yyyy. MM. dd. HH:mm:ss">Magyar (2024. 04. 01. 15:12:22)</option>
			<option value="dd/MM/yyyy HH:mm:ss">Angol (01/04/2024 15:12:22)</option>
			<option value="dd.MM.yyyy HH:mm:ss">Német (01.04.2024 15:12:22)</option>
		</Select>
		<Textarea bind:value={code}></Textarea>
		<button
			onclick={handle}
			class="hover:bg-pos-100 bg-size-200 bg-pos-0 w-full rounded-lg bg-gradient-to-r from-emerald-500 via-amber-400 to-rose-600 py-2 text-xl font-bold drop-shadow-lg transition-all duration-500"
			>Feldolgozás</button
		>
		{#if Object.keys(a).length > 0}
			<div class="mt-4">
				<h1 class="text-3xl font-bold">A műszak</h1>
				<h2>Elfogadott: {hivasok[0]}</h2>
				<h2 class="mb-2">Lemondott: {a['Lemondott'] ? a['Lemondott'] : 0}</h2>
				{#each Object.keys(a) as ember}
					{#if ember !== 'Lemondott'}
						<h2>{ember}: {a[ember]}</h2>
					{/if}
				{/each}
			</div>
		{/if}
		{#if Object.keys(b).length > 0}
			<div class="mt-4">
				<h1 class="text-3xl font-bold">B műszak</h1>
				<h2>Elfogadott: {hivasok[1]}</h2>
				<h2 class="mb-2">Lemondott: {b['Lemondott'] ? b['Lemondott'] : 0}</h2>
				{#each Object.keys(b) as ember}
					{#if ember !== 'Lemondott'}
						<h2>{ember}: {b[ember]}</h2>
					{/if}
				{/each}
			</div>
		{/if}
		{#if Object.keys(n).length > 0}
			<div class="mt-4">
				<h1 class="text-3xl font-bold">Műszakon kívül</h1>
				<h2>Elfogadott: {hivasok[2]}</h2>
				<h2 class="mb-2">Lemondott: {n['Lemondott'] ? n['Lemondott'] : 0}</h2>
				{#each Object.keys(n) as ember}
					{#if ember !== 'Lemondott'}
						<h2>{ember}: {n[ember]}</h2>
					{/if}
				{/each}
			</div>
		{/if}
		{#if Object.keys(all).length > 0}
			<div class="mb-4 mt-4">
				<h1 class="text-3xl font-bold">Emberek szerint</h1>
				<h2 class="text-gray-400">(a műszak + b műszak + műszakon kívül)</h2>
				{#each Object.keys(all) as ember}
					<h2>
						{ember}:
						{all[ember].a} + {all[ember].b} + {all[ember].n} = {all[ember].a +
							all[ember].b +
							all[ember].n}
					</h2>
				{/each}
			</div>
		{/if}
	</div>
</div>
