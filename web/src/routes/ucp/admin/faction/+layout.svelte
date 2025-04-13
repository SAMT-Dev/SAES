<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { LayoutData } from './$types';
	import { Tooltip } from 'flowbite-svelte';

	let { children, data }: { data: LayoutData; children: Snippet } = $props();
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

<nav
	class="grid grid-cols-2 items-center justify-between bg-blue-700 text-black lg:flex dark:text-white"
>
	<div class="ml-2 flex shrink items-center gap-2 xl:ml-[10vw]">
		<h1 class="hidden text-3xl font-bold drop-shadow-xl md:block">
			{data.layout?.factions?.factionname}
		</h1>
		<h1 class="text-3xl font-bold drop-shadow-xl md:hidden">
			{data.layout?.factions?.factionshortname}
		</h1>
		<a
			href="/ucp"
			aria-label="leave"
			class="icon-[material-symbols--logout-rounded] min-h-6 min-w-6 transition-colors duration-500 hover:text-red-600"
		></a>
		<Tooltip placement="bottom" class="bg-gray-600">Kilépés</Tooltip>
	</div>
	<button
		aria-label="mv-menu"
		class="mr-[10vw] flex cursor-pointer self-center justify-self-end text-3xl font-semibold transition-all duration-200 hover:text-blue-500 lg:hidden"
		onclick={tognav}
	>
		<span class="icon-[material-symbols--menu]"></span>
	</button>
	<div
		bind:this={nav}
		class="child:px-2 child:rounded-lg child:drop-shadow-xl lg:flex! col-span-2 hidden flex-col items-center justify-center text-center text-xl md:flex-row lg:z-auto lg:col-span-1 xl:mr-[10vw]"
	>
		<a href="/ucp/admin/faction" class="transition-all duration-200 hover:bg-blue-600">Főoldal</a>
		<a href="/ucp/admin/faction/logs" class="transition-all duration-200 hover:bg-blue-600"
			>Eseménynapló</a
		>
	</div>
</nav>

{@render children()}
