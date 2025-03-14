<script lang="ts">
	import { loading } from '$lib/loading.svelte';
	import { onMount } from 'svelte';
	import { formatRelative } from 'date-fns';
	import { locale } from '$lib/time';
	import {
		Tooltip,
		Button,
		Select,
		Table,
		TableHead,
		TableBody,
		TableBodyRow,
		TableHeadCell,
		TableBodyCell
	} from 'flowbite-svelte';
	import { page } from '$app/stores';
	import { beforeNavigate, goto } from '$app/navigation';
	import type { PageData } from '../../routes/ucp/admin/items/leintesek/$types';
	import {
		get_status_number,
		get_status_string,
		get_type_number,
		get_type_string
	} from '$lib/ucp/types';
	import { Factions, get_faction_by_id } from '$lib/permissions';
	let haveadmin = $state(false);
	interface Props {
		title?: string;
		data: PageData;
		type: number;
		editdes?: string;
		extraText?: string;
		des?: string;
		tools?: string[];
	}

	let {
		title = '',
		data,
		type,
		editdes = '',
		extraText = '',
		des = '',
		tools = []
	}: Props = $props();
	let modal: HTMLDialogElement | undefined = $state();
	let bindbtn: HTMLButtonElement | undefined = $state();
	let editing = false;
	let originallength = $state(0);
	let potleks: {
		data: {
			items: {
				date: Date;
				id: number;
				owner: string;
				reason: string | null;
				status: number;
				type: number;
				img_1: number;
				img_2: null | number;
				price: null | number;
				target_faction: null | number;
				driver: null | number;
				handled_by: string | null;
				am: boolean;
			}[];
		};

		error: boolean;
	} = $state({ data: { items: [] }, error: false });
	let jona = $state(data.status);
	let multipage = $state(false);
	let bindEdit: any = $state({});
	let editid = 0;
	let bajvan = $state(false);
	async function render() {
		loading.value = true;
		const fatcs = await fetch('/web-api/items', {
			headers: {
				status: jona as string,
				type: type.toString()
			}
		});
		if (fatcs.ok) {
			let handled = [];
			potleks.data.items = [];
			let ret = await fatcs.json();

			if (ret.data.items.length > 10 && ret.data.items.length > 0) {
				multipage = true;
				for (let i = pagee * 10; i < (pagee as number) * 10 + 10; i++) {
					if (ret.data.items[i]) {
						handled.push(ret.data.items[i]);
					}
				}
				potleks = {
					error: ret.error,
					data: {
						items: handled
					}
				};
			} else {
				potleks = ret;
			}
			for (const elem of potleks.data.items) {
				if (elem.handled_by !== null) {
					haveadmin = true;
					break;
				}
				if (potleks.data.items.indexOf(elem) === potleks.data.items.length - 1) {
					haveadmin = false;
				}
			}
			if (potleks.data.items.length === 0) {
				haveadmin = false;
			}
			originallength = ret.data.items.length;
			loading.value = false;
		} else {
			loading.value = false;
			bajvan = true;
		}
	}
	onMount(() => {
		render();
	});

	beforeNavigate((ev) => {
		if (editing) {
			ev.cancel();
		}
	});

	function edit(id: number) {
		modal?.showModal();
		bindEdit = JSON.parse(JSON.stringify(potleks.data.items[id]));
		if (potleks.data.items[id].target_faction) {
			bindEdit.target_faction = get_faction_by_id(potleks.data.items[id].target_faction);
		}
		bindEdit.custombg = false;
		editid = id;
		editing = true;
		console.log(bindEdit);
	}
	async function quickTools(timpo: string, id: number) {
		const fatcs = await fetch('/web-api/items', {
			headers: {
				'Content-Type': 'application/json'
			},
			method: 'POST',
			body: JSON.stringify({
				am: potleks.data.items[id].am,
				id: potleks.data.items[id].id,
				status:
					timpo === 'accept'
						? get_status_number('elfogadva')
						: timpo === 'decline'
							? get_status_number('elutasítva')
							: timpo === 'de' || 'du'
								? get_status_number('elfogadva')
								: potleks.data.items[id].status,
				reason: potleks.data.items[id].reason,
				tipus: type,
				supp_type: timpo === 'de' ? 1 : timpo === 'du' ? 2 : 0
			})
		});
		if (fatcs.ok) {
			const cucc = await fatcs.json();
			modal?.close();
			if (jona === cucc.status) {
				potleks.data.items[id] = cucc;
			} else {
				await render();
			}
		}
	}
	let pagee = $state(data.page as number);
	function switchPage(mode: 'next' | 'prev') {
		let url = new URL($page.url);
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
	function changestatus(ev: Event) {
		if (ev.target) {
			let url = new URL($page.url);
			// @ts-expect-error
			url.searchParams.set('status', ev.target[ev.target.selectedIndex].value);
			url.searchParams.delete('page', pagee.toString());
			goto(`?${url.searchParams.toString()}`);
			// @ts-expect-error
			jona = ev.target[ev.target.selectedIndex].value;
			pagee = 0;
			render();
		}
	}
	async function editDone() {
		bindbtn?.classList.add('cursor-not-allowed');
		bindbtn?.classList.add('bg-emerald-700');
		bindbtn!.disabled = true;
		const fatcs = await fetch('/web-api/items', {
			headers: {
				'Content-Type': 'application/json'
			},

			method: 'POST',
			body: JSON.stringify({
				id: bindEdit.id,
				am: bindEdit.am,
				tipus: type,
				status: bindEdit.status,
				reason: bindEdit.reason,
				supp_type: bindEdit.type,
				price: bindEdit.price,
				target_faction: bindEdit.target_faction,
				driver: bindEdit.driver
			})
		});
		if (fatcs.ok) {
			const cucc = await fatcs.json();
			modal?.close();
			editing = false;
			if (jona === cucc.status) {
				potleks.data.items[editid] = cucc;
			} else {
				await render();
			}
		}
		bindbtn?.classList.remove('cursor-not-allowed');
		bindbtn?.classList.remove('bg-emerald-700');
		bindbtn!.disabled = false;
	}
	function closeModal() {
		modal?.close();
		editing = false;
	}
</script>

<dialog
	bind:this={modal}
	class="m-auto h-screen w-screen rounded-3xl bg-black/75 text-center text-white open:flex lg:h-[800px] lg:w-[600px]"
>
	{#if bindEdit.custombg}
		<img
			src={`${data.cdn}/get?id=${bindEdit.img_1}`}
			class="absolute left-1/2 top-1/2 h-full -translate-x-1/2 -translate-y-1/2 opacity-90"
			alt=""
		/>
	{/if}
	<button
		aria-label="Kép megnézése"
		class="absolute right-16 top-2 flex items-center justify-center rounded-xl bg-black bg-opacity-40 p-2 text-3xl font-bold text-blue-600 duration-150 hover:bg-opacity-90"
		onclick={() => {
			if (bindEdit.custombg) {
				bindEdit.custombg = false;
			} else {
				bindEdit.custombg = true;
			}
		}}><span class="icon-[mdi--image] m-auto"></span></button
	>
	<button
		aria-label="Bezárás"
		class="absolute right-4 top-2 flex items-center justify-center rounded-xl bg-black bg-opacity-40 p-2 text-3xl font-bold text-red-600 duration-150 hover:bg-opacity-90"
		onclick={() => closeModal()}><span class="icon-[carbon--close-filled] m-auto"></span></button
	>
	{#if !bindEdit.custombg}
		<div class="z-20 m-auto h-max w-max rounded-3xl bg-black/25 p-5 lg:w-[500px]">
			<form onsubmit={() => editDone()}>
				<div class="grid grid-cols-2 items-center gap-3">
					<h1 class=" col-span-2 mx-2 text-3xl font-bold">
						{bindEdit.owner}
						{editdes} szerkesztése
					</h1>
					<label for="type" class="text-xl">Státusz</label>
					<Select
						placeholder="Kérlek válassz"
						name="type"
						class="bg-emerald-600 text-xl text-black opacity-80 focus:opacity-100 dark:text-white"
						bind:value={bindEdit.status}
					>
						<option value={1}>feltöltve</option>
						<option value={2}>elfogadva</option>
						<option value={3}>elutasítva</option>
					</Select>

					<label for="reason" class="text-xl">Megjegyzés</label>
					<input
						type="text"
						name="reason"
						id="reason"
						class="text-xl text-black opacity-80 focus:opacity-100"
						bind:value={bindEdit.reason}
					/>
					{#if type === get_type_number('számla')}
						<label for="driver" class="text-xl">Kedvezményezett neve</label>
						<input
							type="text"
							name="driver"
							id="driver"
							class="text-xl text-black opacity-80 focus:opacity-100"
							bind:value={bindEdit.driver}
						/>
						<label for="target_faction" class="text-xl">Frakció</label>
						<Select
							placeholder="Kérlek válassz"
							name="target_faction"
							bind:value={bindEdit.target_faction}
							class="bg-emerald-600 text-xl text-black opacity-80 focus:opacity-100 dark:text-white"
						>
							{#each Object.values(Factions) as fact}
								<option value={fact}>{fact}</option>
							{/each}
						</Select>
					{/if}
					{#if extraText}
						<label for="extra" class="text-xl">{extraText}</label>
						{#if type === get_type_number('pótlék')}
							<Select
								placeholder="Kérlek válassz"
								name="potlek-type"
								class="bg-emerald-600 text-xl text-black opacity-80 focus:opacity-100 dark:text-white"
								bind:value={bindEdit.type}
							>
								<option value={1}>délelőtti</option>
								<option value={2}>éjszakai</option>
							</Select>
						{:else}
							<input
								type="number"
								name="extra"
								id="extra"
								class="text-xl text-black opacity-80 focus:opacity-100"
								bind:value={bindEdit.price}
							/>
						{/if}
					{/if}
					<button
						type="submit"
						bind:this={bindbtn}
						id="dialogbtn"
						class="col-span-2 rounded-xl bg-emerald-500 px-2 py-1 text-2xl opacity-80 transition-all duration-200 hover:bg-emerald-700 hover:opacity-100"
						>Mentés</button
					>
				</div>
			</form>
		</div>
	{/if}
</dialog>

<div class="flex">
	<div class="m-auto text-center text-black dark:text-white">
		{#if potleks && !potleks.error}
			<div class="flex items-end justify-center gap-1 text-end">
				<h1 class="text-2xl font-bold">{title}</h1>
				{#if originallength > 0}
					<h2 class="text-taxi text-xl font-bold">{originallength} db</h2>
				{/if}
			</div>
			<h1 class="mb-2 text-xl text-gray-400">{des}</h1>
			<div class="flex items-center justify-center gap-2 text-center">
				<h2 class="text-xl font-bold text-black dark:text-white">Filter</h2>
				<Select
					placeholder="Kérlek válassz"
					id="potlek-type"
					class="bg-emerald-600 font-bold text-black dark:text-white"
					bind:value={jona}
					on:input={(e) => changestatus(e)}
				>
					<option value="1" class="font-bold">Feltöltve</option>
					<option value="2" class="font-bold">Elfogadva</option>
					<option value="3" class="font-bold">Elutasítva</option>
				</Select>
			</div>
			<Table class="mt-5 table-auto p-10 text-center text-black dark:text-white">
				<TableHead class="rounded-xl bg-gray-700 text-white">
					<TableHeadCell>ID</TableHeadCell>
					<TableHeadCell>Dátum</TableHeadCell>
					<TableHeadCell>IG Név</TableHeadCell>
					<TableHeadCell>Kép (Kattints rá)</TableHeadCell>
					<TableHeadCell>Státusz</TableHeadCell>
					<TableHeadCell>Megjegyzés</TableHeadCell>
					{#if type === get_type_number('számla')}
						<TableHeadCell>Frakció</TableHeadCell>
						<TableHeadCell>Kedvezményezett neve</TableHeadCell>
					{/if}
					{#if extraText}
						<TableHeadCell>{extraText}</TableHeadCell>
					{/if}
					{#if haveadmin}
						<TableHeadCell>Kezelte</TableHeadCell>
					{/if}
					{#if tools.length > 0}
						<TableHeadCell>Műveletek</TableHeadCell>
					{/if}
				</TableHead>
				<TableBody>
					{#each potleks.data.items as potle}
						<TableBodyRow>
							<TableBodyCell>{potle.id}</TableBodyCell>
							<TableBodyCell
								>{formatRelative(
									new Date(new Date(potle.date).valueOf() - data.offset!),
									new Date(),
									{ locale }
								)}</TableBodyCell
							>
							<TableBodyCell
								>{potle.owner}
								{#if potle.am}
									(TOW)
								{/if}</TableBodyCell
							>
							<TableBodyCell>
								{#if type == get_type_number('leintés')}
									<div class="flex flex-col xl:flex-row">
										<a href={`${data.cdn}/get?id=${potle.img_1}`} target="”_blank”"
											><img
												loading="lazy"
												src={`${data.cdn}/get?id=${potle.img_1}`}
												alt=""
												class="lg:w-52"
											/></a
										>
										<a href={`${data.cdn}/get?id=${potle.img_2}`} target="”_blank”"
											><img
												loading="lazy"
												src={`${data.cdn}/get?id=${potle.img_2}`}
												alt=""
												class="lg:w-52"
											/></a
										>
									</div>
								{:else}
									<a href={`${data.cdn}/get?id=${potle.img_1}`} target="”_blank”"
										><img
											loading="lazy"
											src={`${data.cdn}/get?id=${potle.img_1}`}
											alt=""
											class="lg:w-52"
										/></a
									>
								{/if}
							</TableBodyCell>
							<TableBodyCell>{get_status_string(potle.status)}</TableBodyCell>
							<TableBodyCell>{potle.reason ? potle.reason : 'nincs'}</TableBodyCell>
							{#if type === get_type_number('számla')}
								<TableBodyCell
									>{potle.target_faction
										? get_faction_by_id(potle.target_faction)
										: 'nincs'}</TableBodyCell
								>
								<TableBodyCell>{potle.driver ? potle.driver : 'nincs'}</TableBodyCell>
							{/if}
							{#if extraText}
								<TableBodyCell
									>{potle.price
										? potle.price
										: potle.type == 1
											? 'délelőtti'
											: potle.type == 2
												? 'éjszakai'
												: 'nincs'}</TableBodyCell
								>
							{/if}
							{#if haveadmin}
								<TableBodyCell>{potle.handled_by ? potle.handled_by : '-'}</TableBodyCell>
							{/if}
							{#if tools.length > 0}
								<TableBodyCell>
									{#if tools.includes('délelőtti') && jona === get_status_number('feltöltve').toString()}
										<Button
											class="icon-[lucide--sun] h-6 w-6 rounded-xl bg-black font-bold transition-all duration-150 hover:bg-yellow-300 dark:bg-white"
											on:click={() => quickTools('de', potleks.data.items.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500"
											>{get_type_string(type)[0].toUpperCase() + get_type_string(type).substring(1)}
											elfogadása délelőttiként</Tooltip
										>
									{/if}
									{#if tools.includes('éjszakai') && jona === get_status_number('feltöltve').toString()}
										<Button
											class="icon-[lucide--moon] h-6 w-6 rounded-xl bg-black font-bold transition-all duration-150 hover:bg-blue-800 dark:bg-white"
											on:click={() => quickTools('du', potleks.data.items.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500"
											>{get_type_string(type)[0].toUpperCase() + get_type_string(type).substring(1)}
											elfogadása éjszakaiként</Tooltip
										>
									{/if}
									{#if tools.includes('accept') && jona === get_status_number('feltöltve').toString()}
										<Button
											class="icon-[lucide--check] h-6 w-6 rounded-xl bg-black font-bold transition-all duration-150 hover:bg-green-500 dark:bg-white"
											on:click={() => quickTools('accept', potleks.data.items.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500"
											>{get_type_string(type)[0].toUpperCase() + get_type_string(type).substring(1)}
											elfogadása</Tooltip
										>
									{/if}
									{#if tools.includes('decline') && jona === get_status_number('feltöltve').toString()}
										<Button
											class="icon-[lucide--x] h-6 w-6 rounded-xl bg-black font-bold transition-all duration-150 hover:bg-red-600 dark:bg-white"
											on:click={() => quickTools('decline', potleks.data.items.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500"
											>{get_type_string(type)[0].toUpperCase() + get_type_string(type).substring(1)}
											elutasítása</Tooltip
										>
									{/if}
									{#if tools.includes('edit')}
										<Button
											class="icon-[lucide--edit] h-6 w-6 rounded-xl bg-black font-bold transition-all duration-150 hover:bg-slate-500 dark:bg-white"
											on:click={() => edit(potleks.data.items.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500"
											>{get_type_string(type)[0].toUpperCase() + get_type_string(type).substring(1)}
											szerkesztése</Tooltip
										>
									{/if}
								</TableBodyCell>
							{/if}
						</TableBodyRow>
					{/each}
				</TableBody>
			</Table>

			{#if potleks.data.items.length === 0}
				{#if bajvan}
					<h2>Lekérés nem jött össze. 😭</h2>
				{:else}
					<h2>Nincs ilyen elem az adatbázisban!</h2>
				{/if}
			{/if}
		{:else}
			<h2>Sikertelen lekérdezés</h2>
		{/if}
	</div>
</div>
{#if multipage}
	<div class="mb-5 mt-5 flex items-center justify-center gap-4">
		{#if pagee > 0}
			<button
				aria-label="Előző oldal"
				onclick={() => switchPage('prev')}
				class="bg-linear-to-r rounded-full from-emerald-500 via-teal-600 to-red-500 bg-[size:200%] bg-[position:0] text-black duration-300 hover:bg-[position:100%] dark:text-white"
				style="width: calc(5vw*2.5); height: 5vh;"
				><span class="icon-[solar--map-arrow-left-bold] h-full w-full"></span></button
			>
		{/if}
		{#if Math.ceil(originallength / 10) - 1 > pagee}
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
