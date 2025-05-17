<script lang="ts">
	import {
		Checkbox,
		Select,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
		Tooltip
	} from 'flowbite-svelte';
	import { formatRelative } from 'date-fns';
	import { locale } from '$lib/time';
	import { get_status_string, get_type_string } from '$lib/ucp/types';
	import type { Logs, SMGetItemsFull } from '$lib/types';
	import { loading } from '$lib/loading.svelte';
	import type { LayoutData } from '../../routes/ucp/$types';
	import { onMount } from 'svelte';
	import { page } from '$app/state';

	let modal: HTMLDialogElement | undefined = $state();
	let modalItem: SMGetItemsFull | undefined = $state();
	let modaltype: string | undefined = $state();
	let modalDetails: string | undefined = $state();
	let {
		data,
		multifaction = false,
		filters
	}: {
		data: {
			logs: Logs[];
			cdn?: string;
			offset?: number;
			layout?: LayoutData['layout'];
		};
		multifaction?: boolean;
		filters: string[];
	} = $props();
	let selected_filters: { login: boolean; upload_item: boolean; update_item: boolean } = $state({
		login: true,
		upload_item: true,
		update_item: true
	});

	let real_logs: Logs[] = $state([]);

	let filter: string[] = $state([]);

	let pagen = $state(
		page.url.searchParams.get('page') ? Number(page.url.searchParams.get('page')) : 0
	);

	let pagenums = $state(50);

	function handle_msg(msg: string) {
		let raw = ['status', 'price', 'supp_type', 'reason'];
		let hun = ['állapot', 'összeg', 'típus', 'megjegyzés'];
		let msgs = msg.split(';');
		let done_text = '';
		for (const text of msgs) {
			if (done_text.length > 0) {
				done_text += ', ';
			}
			let letters = text.split(' ');
			for (const letter of letters) {
				if (raw.includes(letter)) {
					done_text += `${hun[raw.indexOf(letter)]}: ${letter === 'status' ? get_status_string(Number(text.split('FROM')[1].split('TO')[0].replaceAll('{saes_semicolon}', ';'))) : text.split('FROM')[1].split('TO')[0].replaceAll('{saes_semicolon}', ';')} -> ${letter === 'status' ? get_status_string(Number(text.split('TO')[1].replaceAll('{saes_semicolon}', ';'))) : text.split('TO')[1].replaceAll('{saes_semicolon}', ';')}`;
				}
			}
		}
		return done_text;
	}

	function filter_check() {
		if (filter.includes('login') && !selected_filters.login) {
			filter = filter.filter((e) => e !== 'login');
		}
		if (!filter.includes('login') && selected_filters.login) {
			filter.push('login');
		}
		if (filter.includes('update item') && !selected_filters.update_item) {
			filter = filter.filter((e) => e !== 'update item');
		}
		if (!filter.includes('update item') && selected_filters.update_item) {
			filter.push('update item');
		}
		if (filter.includes('upload item') && !selected_filters.upload_item) {
			filter = filter.filter((e) => e !== 'upload item');
		}
		if (!filter.includes('upload item') && selected_filters.upload_item) {
			filter.push('upload item');
		}
	}

	function get_details(details: string, type: string) {
		if (type === 'UPDATE ITEM') {
			modaltype = type;
			let msg = handle_msg(details);
			modal!.showModal();
			modalDetails = msg;
		}
		if (type === 'UPLOAD ITEM') {
			loading.value = true;
			let jsoff: { type: number; id: number } = JSON.parse(details);
			fetch('/web-api/faction/get_by_id', {
				headers: {
					item_id: jsoff.id.toString(),
					item_type: jsoff.type.toString()
				}
			}).then(async (res) => {
				let body: SMGetItemsFull = await res.json();
				loading.value = false;
				modalItem = body;
				modaltype = type;
				modal!.showModal();
			});
		}
	}

	onMount(() => {
		render_logs();
	});

	function render_logs() {
		filter_check();
		if (data.logs.length > pagenums) {
			real_logs = [];
			let extra = 0;
			for (let i = pagen * pagenums; i < pagenums * (pagen + 1) + extra; i++) {
				if (!data.logs[i]) break;
				if (filter.includes(data.logs[i].action.toLowerCase())) real_logs.push(data.logs[i]);
				if (i === pagenums * (pagen + 1) + extra - 1 && real_logs.length < pagenums) extra++;
			}
		} else {
			real_logs = data.logs;
		}
	}

	function closeModal() {
		modal!.close();
		modalItem = undefined;
		modaltype = undefined;
	}
</script>

<dialog
	bind:this={modal}
	class="m-auto h-screen w-screen rounded-3xl bg-black/75 text-center text-white lg:h-[800px] lg:w-[600px]"
>
	<button
		aria-label="Bezárás"
		class="absolute right-4 top-2 flex items-center justify-center rounded-xl bg-black bg-opacity-40 p-2 text-3xl font-bold text-red-600 duration-150 hover:bg-opacity-90"
		onclick={() => closeModal()}><span class="icon-[carbon--close-filled] m-auto"></span></button
	>
	<div class="m-auto mt-4 items-center justify-center">
		{#if modaltype === 'UPDATE ITEM'}
			<h1 class="mb-3 text-3xl font-bold">Változások:</h1>
			<h2>{modalDetails}</h2>
		{/if}
		{#if modaltype === 'UPLOAD ITEM' && modalItem}
			<h1 class="mb-3 text-3xl font-bold">Feltöltött elem információi:</h1>
			{#if modalItem?.item_type === 2}
				<div class="flex gap-1">
					<a target="_blank" href={`${data.cdn}/get?id=${modalItem?.img_1}`}
						><img
							class="m-auto"
							src={`${data.cdn}/get?id=${modalItem?.img_1}`}
							width="300vw"
							alt=""
						/></a
					>
					<a target="_blank" href={`${data.cdn}/get?id=${modalItem?.img_2}`}
						><img
							class="m-auto"
							src={`${data.cdn}/get?id=${modalItem?.img_2}`}
							width="300vw"
							alt=""
						/></a
					>
				</div>
			{:else}
				<a target="_blank" href={`${data.cdn}/get?id=${modalItem?.img_1}`}
					><img
						class="m-auto"
						src={`${data.cdn}/get?id=${modalItem?.img_1}`}
						width="300vw"
						alt=""
					/></a
				>
			{/if}
			<h2>Feltöltő: {modalItem.owner}</h2>
			<h2>
				Kép dátuma: {formatRelative(
					new Date(new Date(modalItem.date).valueOf() - data.offset!),
					new Date(),
					{
						locale
					}
				)}
			</h2>
			<h2>Státusz: {get_status_string(modalItem.status)}</h2>
			{#if modalItem.reason}
				<h2>Megjegyzés: {modalItem.reason}</h2>
			{/if}
			{#if modalItem.price}
				<h2>Összeg: {modalItem.price}</h2>
			{/if}
			{#if modalItem.type}
				<h2>Típus: {get_type_string(modalItem.type)}</h2>
			{/if}
			{#if modalItem.handled_by}
				<h2>Kezelte: {modalItem.handled_by}</h2>
			{/if}
		{/if}
	</div>
</dialog>

<div class="flex">
	<div class="m-auto text-center text-black dark:text-white">
		<h1 class="font-itim mt-2 text-3xl font-bold">Események</h1>
		<Select bind:value={pagenums} onchange={render_logs}>
			<option value={10}>10</option>
			<option value={20}>20</option>
			<option value={50}>50</option>
			<option value={100}>100</option>
		</Select>
		<div class="mt-3 flex items-center justify-center gap-5">
			{#if filters.includes('login')}
				<Checkbox
					bind:checked={selected_filters.login}
					onchange={render_logs}
					name="login"
					class="gap-1 rounded-lg bg-gray-400 bg-opacity-30 px-2 py-1 font-sans text-black dark:text-white"
					>Bejelentkezés</Checkbox
				>
			{/if}
			{#if filters.includes('upload_item')}
				<Checkbox
					bind:checked={selected_filters.upload_item}
					onchange={render_logs}
					class="gap-1 rounded-lg bg-gray-400 bg-opacity-30 px-2 py-1 font-sans text-black dark:text-white"
					name="upload item">Elem feltöltés</Checkbox
				>
			{/if}
			{#if filters.includes('update_item')}
				<Checkbox
					bind:checked={selected_filters.update_item}
					onchange={render_logs}
					class="gap-1 rounded-lg bg-gray-400 bg-opacity-30 px-2 py-1 font-sans text-black dark:text-white"
					name="update item">Elem szerkesztés</Checkbox
				>
			{/if}
		</div>
		<Table class="mt-5 table-auto p-10 text-center text-black dark:text-white">
			<TableHead class="rounded-xl bg-gray-700 text-white">
				<TableHeadCell>Kép</TableHeadCell>
				<TableHeadCell>Dátum</TableHeadCell>
				<TableHeadCell>Esemény létrehozója</TableHeadCell>
				<TableHeadCell>Esemény</TableHeadCell>
				{#if multifaction}
					<TableHeadCell>Frakció</TableHeadCell>
				{/if}
				<TableHeadCell>Elem (típus/id)</TableHeadCell>
				<TableHeadCell>Részletek</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each real_logs as log}
					{#if filter.includes(log.action.toLowerCase())}
						<TableBodyRow>
							<TableBodyCell
								><span
									class={`${
										log.action === 'UPLOAD ITEM'
											? 'icon-[material-symbols--upload-file] text-green-500'
											: ''
									}${
										log.action === 'UPDATE ITEM'
											? 'icon-[material-symbols--edit-document] text-blue-600'
											: ''
									}${log.action === 'LOGIN' ? 'icon-[material-symbols--how-to-reg] text-yellow-300' : ''} h-10 w-10`}
								></span></TableBodyCell
							>
							<TableBodyCell
								>{formatRelative(new Date(log.date), new Date(), {
									locale
								})}</TableBodyCell
							>
							<TableBodyCell>{log.owner}</TableBodyCell>
							<TableBodyCell
								>{#if log.action === 'UPLOAD ITEM'}
									Elem feltöltés
								{/if}
								{#if log.action === 'UPDATE ITEM'}
									Elem szerkesztés
								{/if}
								{#if log.action === 'LOGIN'}
									Bejelentkezés
								{/if}
							</TableBodyCell>
							{#if multifaction}
								<TableBodyCell>{log.faction}</TableBodyCell>
							{/if}
							<TableBodyCell>
								{#if log.action !== 'LOGIN'}
									{log.item_type ? get_type_string(log.item_type) : ''} / {log.item_id}
								{/if}
							</TableBodyCell>
							<TableBodyCell
								>{#if log.action === 'UPDATE ITEM'}<button
										onclick={() => get_details(log.message!, log.action)}
										aria-label="More"
										class="icon-[material-symbols--ad] h-10 w-10 transition-colors duration-150 hover:text-emerald-400"
									></button><Tooltip class="bg-gray-600">Részletek megnézése</Tooltip>
								{/if}
								{#if log.action === 'UPLOAD ITEM'}<button
										onclick={() =>
											get_details(
												JSON.stringify({
													type: log.item_type,
													id: log.item_id
												}),
												log.action
											)}
										aria-label="More"
										class="icon-[material-symbols--ad] h-10 w-10 transition-colors duration-150 hover:text-emerald-400"
									></button><Tooltip class="bg-slate-500">Részletek megnézése</Tooltip>
								{/if}
							</TableBodyCell>
						</TableBodyRow>
					{/if}
				{/each}
			</TableBody>
		</Table>
	</div>
</div>
