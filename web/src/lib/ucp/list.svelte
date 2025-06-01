<script lang="ts">
	import { goto } from '$app/navigation';
	import Error from '$lib/error.svelte';
	import { onMount } from 'svelte';
	import type { PageData } from '../../routes/ucp/potlekok/$types';
	import { formatRelative } from 'date-fns';
	import { locale } from '$lib/time';
	import { get_status_string, get_type_en_string, get_type_number, get_type_string } from './types';
	import { Tooltip } from 'flowbite-svelte';
	import { page } from '$app/state';
	let multipage = $state(false);
	interface Props {
		data: PageData;
		tipus: number;
		display?: string;
	}
	let { data, tipus, display = '' }: Props = $props();
	let handled_potleks: any = $state([]);
	let pagee = $state(data.page as number);
	let usernames: Record<string, { name: string }> = $state({});
	function switchPage(mode: 'next' | 'prev') {
		let url = new URL(page.url);
		if (mode === 'next') {
			url.searchParams.set('page', String(Number(pagee) + 1));
			goto(`?${url.searchParams.toString()}`);
			pagee = Number(pagee) + 1;
			render();
		}
		if (mode === 'prev') {
			url.searchParams.set('page', String(Number(pagee) - 1));
			goto(`?${url.searchParams.toString()}`);
			pagee = Number(pagee) - 1;
			render();
		}
	}
	async function render() {
		handled_potleks = [];
		if (data.potlekok.length > 10 && data.potlekok.length > 0) {
			multipage = true;
			for (let i = (pagee as number) * 10; i < (pagee as number) * 10 + 10; i++) {
				if (data.potlekok[i]) {
					handled_potleks.push(data.potlekok[i]);
				}
			}
		} else {
			handled_potleks = data.potlekok;
		}
		let ids: number[] = [];
		for (const elem of handled_potleks) {
			if (elem.owner !== data.layout?.driverid && !ids.includes(elem.owner)) {
				ids.push(elem.owner);
			}
			if (elem.handled_by !== null && !ids.includes(elem.handled_by)) {
				ids.push(elem.handled_by);
			}
			if (elem.driver && !ids.includes(elem.driver)) {
				ids.push(elem.driver);
			}
		}
		if (ids.length > 0) {
			const fetcs = await fetch('/web-api/getusernames', {
				headers: {
					ids: JSON.stringify(ids)
				}
			});
			let names = await fetcs.json();
			usernames = names;
		}
	}
	onMount(async () => {
		await render();
	});
</script>

<Error {data}>
	<div
		class="grid grid-cols-1 content-center items-center justify-center text-center text-black dark:text-white"
	>
		<h1 class="mt-5 text-5xl font-bold drop-shadow-xl">{display}:</h1>
		<div class="m-auto mb-5 flex gap-5">
			<h2 class="m-auto text-black dark:text-white">
				Összesen {data.potlekok.length} darab.
			</h2>
			<a
				href={`${page.url.pathname}/upload`}
				aria-label="Feltöltés"
				class="from-taxi bg-linear-to-r h-8 w-16 rounded-full via-teal-400 to-green-600 bg-[size:200%] bg-[position:0] text-center text-xl font-bold text-black shadow-2xl drop-shadow-lg transition-all duration-500 hover:bg-[position:100%] dark:text-white"
				><span class="icon-[material-symbols--upload] h-full w-full"></span></a
			>
			{#if data.layout?.access[get_type_en_string(tipus)] === 'Write'}
				<Tooltip class="bg-slate-500">
					Új {get_type_string(tipus)} feltöltése
				</Tooltip>
			{/if}
		</div>
		<div class="mb-3 flex flex-auto flex-wrap items-center justify-center gap-3 align-middle">
			{#if handled_potleks}
				{#each handled_potleks as potle}
					<div
						class="bg-linear-to-bl rounded-lg p-2 drop-shadow-xl"
						class:from-teal-500={get_status_string(potle.status) === 'elfogadva'}
						class:to-green-600={get_status_string(potle.status) === 'elfogadva'}
						class:from-amber-500={get_status_string(potle.status) === 'elutasítva'}
						class:to-red-600={get_status_string(potle.status) === 'elutasítva'}
						class:from-cyan-800={get_status_string(potle.status) === 'feltöltve'}
						class:to-gray-700={get_status_string(potle.status) === 'feltöltve'}
					>
						<h1 class="-mb-2 text-2xl font-bold text-white drop-shadow-xl">
							{get_status_string(potle.status).toUpperCase()}
						</h1>
						<!-- <h1 class="text-gray-200 drop-shadow-xl">
							{formatRelative(new Date(potle.date), new Date(), {
								locale
							})}
						</h1> -->
						<h1 class="text-gray-200 drop-shadow-xl">{potle.date}</h1>
						{#if data.layout?.driverid !== potle.owner}
							<h1 class="drop-shadow-xl">
								Feltöltő: {usernames[potle.owner] ? usernames[potle.owner].name : potle.owner}
							</h1>
						{/if}
						{#if tipus === get_type_number('számla') && potle.driver && data.layout?.driverid !== potle.driver}
							<h1 class="drop-shadow-xl">
								Kedvezményezett: {usernames[potle.driver]
									? usernames[potle.driver].name
									: potle.driver}
							</h1>
						{/if}
						{#if tipus === get_type_number('számla') && potle.price}
							<h1 class="drop-shadow-xl">Végösszeg: {potle.price}$</h1>
						{/if}
						{#if potle.reason}
							<h1 class="drop-shadow-xl">
								Megjegyzés: {potle.reason}
							</h1>
						{/if}

						{#if tipus === get_type_number('leintés')}
							<div class="flex flex-col xl:flex-row">
								<a
									href={`${data.cdn}/get?id=${potle.img_1}`}
									target="_blank"
									onmouseenter={() => (potle.focus1 = true)}
									onmouseleave={() => (potle.focus1 = false)}
								>
									<img
										loading="lazy"
										src={`${data.cdn}/get?id=${potle.img_1}`}
										alt=""
										class="max-h-xl m-auto max-w-xl py-2 drop-shadow-xl"
										class:blur={potle.focus1}
									/>
									{#if potle.focus1}
										<span
											class="icon-[mdi--gesture-touch-hold] absolute left-1/4 top-1/2 h-24 w-24 -translate-x-1/4 -translate-y-1/2 transform shadow-2xl"
										></span>
									{/if}
								</a>
								<a
									href={`${data.cdn}/get?id=${potle.img_2}`}
									target="_blank"
									onmouseenter={() => (potle.focus2 = true)}
									onmouseleave={() => (potle.focus2 = false)}
								>
									<img
										loading="lazy"
										src={`${data.cdn}/get?id=${potle.img_2}`}
										alt=""
										class="max-h-xl m-auto max-w-xl py-2 drop-shadow-xl"
										class:blur={potle.focus2}
									/>
									{#if potle.focus2}
										<span
											class="icon-[mdi--gesture-touch-hold] absolute left-3/4 top-1/2 h-24 w-24 -translate-x-3/4 -translate-y-1/2 transform shadow-2xl"
										></span>
									{/if}
								</a>
							</div>
						{:else}
							<a
								href={`${data.cdn}/get?id=${potle.img_1}`}
								target="_blank"
								onmouseenter={() => (potle.focus = true)}
								onmouseleave={() => (potle.focus = false)}
							>
								<img
									loading="lazy"
									src={`${data.cdn}/get?id=${potle.img_1}`}
									alt=""
									class="max-h-xl m-auto max-w-xl py-2 drop-shadow-xl"
									class:blur={potle.focus}
								/>
								{#if potle.focus}
									<span
										class="icon-[mdi--gesture-touch-hold] absolute left-1/2 top-1/2 h-24 w-24 -translate-x-1/2 -translate-y-1/2 transform shadow-2xl"
									></span>
								{/if}
							</a>
						{/if}

						{#if potle.handled_by}
							<h1 class="drop-shadow-xl">
								Kezelte: {usernames[potle.handled_by]
									? usernames[potle.handled_by].name
									: potle.handled_by}
							</h1>
						{/if}
					</div>
				{/each}
			{:else}
				<h2>Az elmúlt 2 hétben nem töltöttél fel semmit!</h2>
			{/if}
		</div>
	</div>
	{#if multipage}
		<div class="mb-3 flex items-center justify-center gap-4">
			{#if pagee > 0}
				<button
					aria-label="Előző oldal"
					onclick={() => switchPage('prev')}
					class="bg-linear-to-r rounded-full from-emerald-500 via-teal-600 to-red-500 bg-[size:200%] bg-[position:0] text-black duration-300 hover:bg-[position:100%] dark:text-white"
					style="width: calc(5vw*2.5); height: 5vh;"
					><span class="icon-[solar--map-arrow-left-bold] h-full w-full"></span></button
				>
			{/if}
			{#if Math.ceil(data.potlekok.length / 10) - 1 > pagee}
				<button
					aria-label="Következő oldal"
					onclick={() => switchPage('next')}
					class="bg-linear-to-r rounded-full from-emerald-500 via-teal-600 to-red-500 bg-[size:200%] bg-[position:0] text-black duration-300 hover:bg-[position:100%] dark:text-white"
					style="width: calc(5vw*2.5); height: 5vh;"
					><span class="icon-[solar--map-arrow-right-bold] h-full w-full"></span></button
				>
			{/if}
		</div>
	{/if}
</Error>
