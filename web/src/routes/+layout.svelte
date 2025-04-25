<script lang="ts">
	import '../app.css';
	import '../snow.css';
	import { navigating, page } from '$app/state';
	import { loading } from '$lib/loading.svelte';
	import { fade } from 'svelte/transition';
	import Snow from '$lib/snow.svelte';
	import { snow } from '$lib/api';
	let { children } = $props();
	$effect(() => {
		loading.value = !!navigating.type;
	});
	let error = page.url.searchParams.get('error');
</script>

<svelte:head>
	<title>SAMT</title>
	<meta name="description" content="A SAMT rendszerének weboldala." />
	<meta content="SAMT Weboldal" property="og:title" />
	<meta content="/" property="og:url" />
	<meta content="A SAMT rendszerének weboldala." property="og:description" />
	<meta content="/favicon.png" property="og:image" />
	<meta content="#fece01" data-react-helmet="true" name="theme-color" />
</svelte:head>
{#if snow}
	<Snow />
{/if}
{#if !error}
	{@render children?.()}
{/if}
{#if error === 'noperm'}
	<div class="flex h-screen">
		<div class="m-auto text-center text-white">
			<h1 class="text-center text-3xl font-bold">Nincs jogod az oldal használatához!</h1>
			<a
				href="/"
				data-sveltekit-reload
				class="rounded-xl bg-rose-900 px-2 uppercase text-rose-400 transition-all duration-300 hover:bg-rose-950 hover:text-white"
				>Visszalépés</a
			>
		</div>
	</div>
{/if}

{#if error === 'oauth2'}
	<div class="flex h-screen">
		<div class="m-auto text-center text-white">
			<h1 class="text-center text-3xl font-bold">Oauth2 session hiba.</h1>
			<a
				href="/"
				data-sveltekit-reload
				class="rounded-xl bg-rose-900 px-2 uppercase text-rose-400 transition-all duration-300 hover:bg-rose-950 hover:text-white"
				>Visszalépés</a
			>
		</div>
	</div>
{/if}

{#if loading.value}
	<div
		class="fixed top-0 z-50 h-full w-full bg-[rgba(0,0,0,0.4)]"
		transition:fade={{ duration: 300 }}
	>
		<div
			class="shadow-taxi border-taxi pointer-events-none fixed
	left-1/2 top-1/2 h-96 w-96 -translate-x-1/2 -translate-y-1/2 transform cursor-progress content-center overflow-hidden rounded-full border-2 shadow-lg"
		>
			<!-- <span class="loader"></span> -->
			{#if snow}
				<img src="/miklos.gif" alt="loading" class=" z-50 block w-[1000px] object-top" />
			{:else}
				<img src="/macska.avif" alt="loading" class=" z-50 block w-[1000px] object-top" />
			{/if}
		</div>
	</div>
{/if}
