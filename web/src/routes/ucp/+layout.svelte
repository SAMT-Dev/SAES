<script lang="ts">
	import { marked } from 'marked';
	import { navigating, page } from '$app/state';
	import Error from '$lib/error.svelte';
	import { Reeler_keys, Reeler_vals } from '$lib/ucp/public.js';
	import { onMount } from 'svelte';
	import { loading } from '$lib/loading.svelte';
	import { socket } from '$lib/socket.js';
	import ViewTransition from '$lib/navigation.svelte';
	import Header from '$lib/ucp/header.svelte';
	import { allowPerms } from '$lib/api.js';
	import { Factions, factPermissions, Permissions } from '$lib/permissions.js';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { io } from 'socket.io-client';
	let { data, children } = $props();
	let maintenance = $state(false);
	let initial_socket = $state(false);
	let announcement = $state(false);
	let nosocket: boolean | string = $state('Socket csatlakozás');
	let tip = $state('SAMT');
	if (!data.noaccess && !data.noauth) {
		if (data.faction === Factions.Taxi) {
			tip = 'TAXI';
		}
		if (data.faction === Factions.Apms) {
			tip = 'APMS';
		}
		if (data.faction === Factions.Tow) {
			tip = 'TOW';
		}
		if (data.faction === Factions.Uni) {
			tip = 'Akadémia';
		}
	}
	if (data.refresh && browser) {
		location.reload();
	}
	onMount(() => {
		if (!data.noaccess && !data.noauth && !data.error && !data.nofact && !data.refresh) {
			$socket = io(data.api, {
				auth: {
					auth_token: data.auth
				}
			});
			$socket.on('maintenance', (data) => {
				if (data !== '') {
					maintenance = data;
				}
			});
			$socket.on('announcement', (data) => {
				if (data !== '') {
					announcement = data;
				}
			});
			$socket.on('doneload', () => {
				console.log('[SOCKET] Socket csatlakozva');
				nosocket = false;
				initial_socket = true;
				loading.value = false;
			});
			$socket.on('disconnect', () => {
				nosocket = 'Socket csatlakozás sikertelen';
			});
			loading.value = true;
		}
	});
</script>

<svelte:head>
	{#if !maintenance && !data.noauth && !data.error && !data.nofact}
		{#if !navigating.type}
			{#if page.url.pathname.includes('shift')}
				<title>Műszakvezetői felület - {tip}</title>
			{:else if page.url.pathname.includes('faction')}
				<title>Frakcióvezetői felület - {tip}</title>
			{:else if page.url.pathname.includes('fleet')}
				<title>Flottakezelői felület - {tip}</title>
			{:else if page.url.pathname.includes('admin')}
				<title>Adminisztrátori felület - {tip}</title>
			{:else if Reeler_keys.some((el) => page.url.pathname.includes(el))}
				{#if page.url.pathname.endsWith('/upload')}
					<title
						>{Reeler_vals[Reeler_keys.indexOf(page.url.pathname.split('/')[2])][2]} feltöltés - {tip}</title
					>
				{:else}
					<title
						>{Reeler_vals[Reeler_keys.indexOf(page.url.pathname.split('/')[2])][1]} megtekintése - {tip}</title
					>
				{/if}
			{:else}
				<title>Felhasználói felület - {tip}</title>
			{/if}
		{/if}
	{:else}
		<title>{tip}</title>
	{/if}
	{#if data.faction === Factions.Taxi || data.faction === Factions.Tow}
		<link rel="icon" href="/sckk_icon.png" />
	{/if}
	{#if data.faction === Factions.Apms}
		<link rel="icon" href="/apms_icon.png" />
	{/if}
	{#if data.faction === Factions.Uni}
		<link rel="icon" href="/uni_icon.png" />
	{/if}
</svelte:head>
<Error {data}>
	{#if data.noauth}
		<main>
			<div class="flex h-screen">
				<div class="m-auto text-center">
					<h1 class="animate-pulse text-3xl font-bold text-black dark:text-white">
						Az oldal használatához kérlek lépj be!
					</h1>
					<button
						aria-label="Belépés Discord használatával"
						class="from-taxi bg-linear-to-r group relative m-auto mt-3 flex h-12 animate-bounce items-center space-x-2 overflow-hidden rounded-full via-rose-500 to-red-600 bg-[size:200%] bg-[position:0] px-6 transition-all duration-500 hover:bg-[position:100%]"
					>
						<a
							href={`${data.api}/auth?path=${page.url.pathname}`}
							aria-label="Belépés Discord használatával"
							class="flex w-full justify-end gap-2 text-black transition-colors duration-300 hover:text-black dark:text-white"
							><span class="icon-[ic--baseline-discord] m-auto h-12 w-12"></span>
							<h2 class="m-auto text-xl font-bold">Discord</h2></a
						>
						<div class="flex translate-x-3 items-center -space-x-3">
							<div
								class="h-[1.6px] w-2.5 origin-left scale-x-0 rounded-sm bg-white transition duration-300 group-hover:scale-x-100 group-hover:bg-black"
							></div>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="h-5 w-5 -translate-x-2 stroke-white transition duration-300 group-hover:translate-x-0 group-hover:stroke-black"
								fill="none"
								viewBox="0 0 24 24"
								stroke="currentColor"
								stroke-width="2"
							>
								<path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
							</svg>
						</div>
					</button>
				</div>
			</div>
		</main>
	{:else if data.noaccess}
		<main>
			<div class="flex h-screen">
				<div class="m-auto text-center">
					<h1 class="text-3xl font-bold text-red-600">{data.noaccess}</h1>
					<a
						href="/logout"
						data-sveltekit-reload
						class="bg-linear-to-r mb-5 ml-5 mr-5 mt-5 block rounded-full from-red-500 via-amber-400 to-rose-600 bg-[size:200%] bg-[position:0] px-2 py-1 text-center text-lg font-bold text-black drop-shadow-lg transition-all duration-500 hover:bg-[position:100%] dark:text-white"
						>Kijelentkezés</a
					>
				</div>
			</div>
		</main>
	{:else if data.nofact}
		<main>
			<div class="flex h-screen items-center justify-center text-center text-black dark:text-white">
				<div class="flex flex-col items-center justify-center gap-2 lg:flex-row lg:gap-5">
					{#if allowPerms(data, [factPermissions[Factions.Taxi].SaesFactUcp])}
						<a
							href="?select_faction=SCKK"
							data-sveltekit-reload
							class="group m-auto items-center justify-center rounded-xl bg-black bg-opacity-60 p-5"
						>
							<img
								src="/sckk_icon.png"
								class="group-hover:border-taxi m-auto min-w-20 max-w-20 rounded-full border-4 border-solid border-white bg-black p-0.5 transition-colors duration-300 lg:min-w-40 lg:max-w-40"
								alt="SCKK Logo"
							/>
							<h1
								class="group-hover:text-taxi text-3xl font-bold tracking-wider transition-colors duration-300"
							>
								TAXI
							</h1>
						</a>
					{/if}
					{#if allowPerms(data, [factPermissions[Factions.Tow].SaesFactUcp])}
						<a
							href="?select_faction=TOW"
							data-sveltekit-reload
							class="group m-auto items-center justify-center rounded-xl bg-black bg-opacity-60 p-5"
						>
							<img
								src="/sckk_icon.png"
								class="group-hover:border-tow m-auto min-w-20 max-w-20 rounded-full border-4 border-solid border-white bg-black p-0.5 transition-colors duration-300 lg:min-w-40 lg:max-w-40"
								alt="SCKK Logo"
							/>
							<h1
								class="group-hover:text-tow text-3xl font-bold tracking-wider transition-colors duration-300"
							>
								TOW
							</h1>
						</a>
					{/if}
					{#if allowPerms(data, [factPermissions.UNI.SaesFactUcp])}
						<a
							href="?select_faction=UNI"
							data-sveltekit-reload
							class="group m-auto items-center justify-center rounded-xl bg-black bg-opacity-60 p-5"
						>
							<img
								src="/uni_icon.png"
								class="group-hover:border-uni m-auto min-w-20 max-w-20 rounded-full border-4 border-solid border-white bg-black p-0.5 transition-colors duration-300 lg:min-w-40 lg:max-w-40"
								alt="SCKK Logo"
							/>
							<h1
								class="group-hover:text-uni text-3xl font-bold tracking-wider transition-colors duration-300"
							>
								Akadémia
							</h1>
						</a>
					{/if}
					{#if allowPerms(data, [factPermissions.APMS.SaesFactUcp])}
						<a
							href="?select_faction=APMS"
							data-sveltekit-reload
							class="group m-auto items-center justify-center rounded-xl bg-black bg-opacity-60 p-5"
						>
							<img
								src="/apms_icon.png"
								class="group-hover:border-apms m-auto min-w-20 max-w-20 rounded-full border-4 border-solid border-white bg-black p-0.5 transition-colors duration-300 lg:min-w-40 lg:max-w-40"
								alt="APMS Logo"
							/>
							<h1
								class="group-hover:text-apms text-3xl font-bold tracking-wider transition-colors duration-300"
							>
								APMS
							</h1>
						</a>
					{/if}
				</div>
			</div>
		</main>
	{:else if !maintenance || data.layout?.admin || allowPerms(data, [Permissions.SaesMaintenance])}
		{#if nosocket}
			<header>
				<div
					class="flex items-center justify-center bg-red-500 text-center text-2xl font-semibold uppercase text-black dark:text-white"
				>
					<h1>
						{#if nosocket !== true}
							{nosocket}
						{:else}
							Sikertelen socket csatlakozás
						{/if}
					</h1>
				</div>
			</header>
		{:else}
			{#if maintenance}
				<header>
					<div
						class="flex items-center justify-center bg-rose-900 text-center text-2xl font-bold uppercase text-black dark:text-white"
					>
						<h1 class="drop-shadow-lg">
							Karbantartás mód aktív {#if typeof maintenance === 'string'}
								- {maintenance}
							{/if}
						</h1>
					</div>
				</header>
			{/if}
			{#if announcement}
				<header>
					<div
						class="flex items-center justify-center bg-blue-500 text-center text-2xl text-black dark:text-white"
					>
						{@html marked(announcement.toString())}
					</div>
				</header>
			{/if}
		{/if}
		{#if initial_socket}
			{#if !page.url.pathname.startsWith('/ucp/admin/')}
				<Header
					{tip}
					faction={data.faction!}
					isAdmin={data.faction === Factions.Taxi
						? allowPerms(data, [factPermissions[Factions.Taxi].SaesFactAdmin])
						: data.faction === Factions.Tow
							? allowPerms(data, [factPermissions[Factions.Tow].SaesFactAdmin])
							: data.faction === Factions.Apms
								? allowPerms(data, [factPermissions[Factions.Apms].SaesFactAdmin])
								: data.faction === Factions.Uni
									? allowPerms(data, [factPermissions[Factions.Uni].SaesFactAdmin])
									: false}
					{data}
					{nosocket}
				/>
			{/if}
			<ViewTransition />
			<main
				class={data.faction === Factions.Taxi
					? 'selection:bg-taxi'
					: data.faction === Factions.Tow
						? 'selection:bg-tow'
						: data.faction === Factions.Apms
							? 'selection:bg-apms'
							: data.faction === Factions.Uni
								? 'selection:bg-uni'
								: ''}
			>
				{@render children?.()}
			</main>
		{/if}
	{:else}
		<main>
			<div class="flex h-screen">
				<div class="m-auto text-center">
					<h1 class="text-5xl font-bold uppercase text-red-600">Karbantartás</h1>
					<h1 class="text-3xl text-gray-300">
						Jelenleg karbantartás zajlik, kérlek nézz vissza később!
					</h1>
					{#if typeof maintenance === 'string'}
						<h1 class="text-2xl text-gray-300">Indoklás: {@html marked(maintenance)}</h1>
					{/if}
					{#if allowPerms(data, [Permissions.SaesMaintenance])}
						<a
							data-sveltekit-reload
							href="/ucp/keine"
							class="bg-linear-to-r mb-5 ml-5 mr-5 mt-5 block rounded-full from-red-500 via-amber-400 to-rose-600 bg-[size:200%] bg-[position:0] px-2 py-1 text-center text-lg font-bold text-black drop-shadow-lg transition-all duration-500 hover:bg-[position:100%] dark:text-white"
							>Továbblépés</a
						>
					{/if}
				</div>
			</div>
		</main>
	{/if}
</Error>
