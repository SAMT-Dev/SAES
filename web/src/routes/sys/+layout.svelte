<script lang="ts">
	import { page } from '$app/state';

	let { children } = $props();
	let nav: HTMLDivElement = $state()!;
	const tognav = () => {
		if (nav.classList.contains('hidden')) {
			nav.classList.remove('hidden');
			nav.classList.add('flex');
		} else {
			nav.classList.add('hidden');
			nav.classList.remove('flex');
		}
	};
</script>

<svelte:head>
	<link rel="icon" href="/favicon.png" />
	<title>SA Management - SAMT</title>
</svelte:head>

<nav
	class="grid grid-cols-2 items-center justify-between bg-rose-500 text-black lg:flex dark:text-white"
>
	<div class="ml-2 flex shrink items-center gap-2 xl:ml-[10vw]">
		<h1 class="hidden text-3xl font-bold drop-shadow-xl md:block">Rendszergizda</h1>
		<h1 class="text-3xl font-bold drop-shadow-xl md:hidden">Sys</h1>
	</div>
	<button
		aria-label="mv-menu"
		class="mr-[10vw] flex cursor-pointer self-center justify-self-end text-3xl font-semibold transition-all duration-200 hover:text-emerald-500 lg:hidden"
		onclick={tognav}
	>
		<span class="icon-[material-symbols--menu]"></span>
	</button>
	<div
		bind:this={nav}
		class="child:px-2 child:rounded-lg child:drop-shadow-xl lg:flex! col-span-2 hidden flex-col items-center justify-center gap-2 text-center text-xl md:flex-row lg:z-auto lg:col-span-1 xl:mr-[10vw]"
	>
		<a href="/ucp" class="transition-all duration-200 hover:bg-rose-700">UCP</a>
		<a
			href="/sys"
			class={`transition-all duration-200 hover:bg-rose-700 ${page.url.pathname.endsWith('/sys') ? 'bg-rose-700' : ''}`}
			>Főoldal</a
		>
		<a
			href="/sys/config"
			class={`transition-all duration-200 hover:bg-rose-700 ${page.url.pathname.endsWith('/sys/config') ? 'bg-rose-700' : ''}`}
			>Config</a
		>
		<a
			href="/sys/logs"
			class={`transition-all duration-200 hover:bg-rose-700 ${page.url.pathname.endsWith('/sys/logs') ? 'bg-rose-700' : ''}`}
			>Logs</a
		>
		<a
			href="/sys/factions"
			class={`transition-all duration-200 hover:bg-rose-700 ${page.url.pathname.endsWith('/sys/factions') ? 'bg-rose-700' : ''}`}
			>Frakciók</a
		>
	</div>
</nav>
{@render children?.()}
