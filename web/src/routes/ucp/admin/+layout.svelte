<script lang="ts">
	import { page } from '$app/state';
	import { allowPerms } from '$lib/api';
	import { factPermissions, Permissions } from '$lib/permissions';

	let { children, data } = $props();
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
	class="grid grid-cols-2 items-center justify-between bg-emerald-700 text-black lg:flex dark:text-white"
>
	<div class="ml-2 flex shrink items-center gap-2 xl:ml-[10vw]">
		<h1 class="hidden text-3xl font-bold drop-shadow-xl md:block">Adminisztráció</h1>
		<h1 class="text-3xl font-bold drop-shadow-xl md:hidden">Admin</h1>
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
		<a
			href="/ucp/admin"
			class={`transition-all duration-200 hover:bg-emerald-600 ${page.url.pathname.endsWith('/admin') ? 'bg-emerald-600' : ''}`}
			>Főoldal</a
		>
		{#if allowPerms(data, [factPermissions.SCKK.SaesFactAdmin, factPermissions.TOW.SaesFactAdmin])}
			<a
				href="/ucp/admin/tools"
				class={`transition-all duration-200 hover:bg-emerald-600 ${page.url.pathname.endsWith('/admin/tools') ? 'bg-emerald-600' : ''}`}
				>Eszközök</a
			>
		{/if}
		<a
			href="/ucp/admin/items"
			class={`transition-all duration-200 hover:bg-emerald-600 ${page.url.pathname.startsWith('/ucp/admin/items') ? 'bg-emerald-600' : ''}`}
			>Feltöltött elemek</a
		>
		{#if allowPerms( data, [factPermissions.SCKK.SaesFactAdminShift, factPermissions.TOW.SaesFactAdminShift] )}
			<a
				href="/ucp/admin/shift"
				class={`transition-all duration-200 hover:bg-emerald-600 ${page.url.pathname.startsWith('/ucp/admin/shift') ? 'bg-emerald-600' : ''}`}
				>Műszakvezetés</a
			>
		{/if}
		{#if allowPerms( data, [factPermissions.SCKK.SaesFactAdminFleet, factPermissions.TOW.SaesFactAdminFleet] )}
			<a
				href="/ucp/admin/fleet"
				class={`transition-all duration-200 hover:bg-emerald-600 ${page.url.pathname.startsWith('/ucp/admin/fleet') ? 'bg-emerald-600' : ''}`}
				>Flottakezelés</a
			>
		{/if}
		{#if allowPerms( data, [factPermissions.SCKK.SaesFactAdminFaction, factPermissions.TOW.SaesFactAdminFaction] )}
			<a
				href="/ucp/admin/faction"
				class={`transition-all duration-200 hover:bg-emerald-600 ${page.url.pathname.startsWith('/ucp/admin/faction') ? 'bg-emerald-600' : ''}`}
				>Frakciókezelés</a
			>
		{/if}
		{#if allowPerms(data, [])}
			<a
				href="/ucp/admin/sys"
				class={`transition-all duration-200 hover:bg-emerald-600 ${page.url.pathname.startsWith('/ucp/admin/sys') ? 'bg-emerald-600' : ''}`}
				>Sysadmin</a
			>
		{/if}
	</div>
</nav>
{@render children?.()}
