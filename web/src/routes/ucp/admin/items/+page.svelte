<script lang="ts">
	import Grid from '$lib/admin/grid.svelte';
	import { allowPerms } from '$lib/api.js';
	import { Factions, Permissions } from '$lib/permissions.js';

	let { data } = $props();
	const supportCountries = ['HU', 'SK', 'RO'];
	let color = supportCountries.includes(data.country as string) ? data.country : 'HU';
</script>

<Grid
	{data}
	items={[
		{
			title: 'Pótlékok',
			description: 'A délelőtti, és éjszakai pótlékok.',
			href: '/ucp/admin/items/potlekok',
			border: 'border-yellow-400',
			background: 'bg-yellow-200 hover:bg-yellow-400',
			permission: [Permissions.SaesTaxiAdminShift, Permissions.SaesTowAdminShift]
		},
		{
			title: 'Leintések',
			description: 'Nem tablet alapú hívások.',
			href: '/ucp/admin/items/leintesek',
			border: 'border-green-400',
			background: 'bg-green-200 hover:bg-green-400',
			permission: [Permissions.SaesTaxiAdminShift, Permissions.SaesTowAdminShift]
		},
		{
			title: 'Szereltetési számlák',
			description: 'Az ütközés papír alapú változatai.',
			href: '/ucp/admin/items/szamlak',
			border: 'border-tow',
			background: 'bg-blue-200 hover:bg-tow',
			permission: [
				Permissions.SaesTaxiAdminShift,
				Permissions.SaesTowAdminShift,
				Permissions.SaesApmsAdmin
			]
		}
	]}
/>

{#if data.faction === Factions.Taxi || data.faction === Factions.Tow}
	<div class="mt-5 text-center">
		<h1 class="mb-2 text-3xl font-bold text-black dark:text-white">Statisztika</h1>
		<div
			class="child:p-2 md:child:p-4 ml-5 mr-5 grid grid-cols-3 gap-5 text-center text-black dark:text-white"
		>
			{#if allowPerms(data, [Permissions.SaesTaxiAdminShift, Permissions.SaesTowAdminShift])}
				<div
					class="rounded-lg"
					class:bg-red-700={color === 'HU'}
					class:bg-white={color === 'SK'}
					class:text-black={color === 'SK'}
					class:bg-blue-900={color === 'RO'}
				>
					<h1 class="font-itim text-2xl font-bold">Új pótlékok száma:</h1>
					<h2 class="font-itim text-3xl font-bold">{data.stat.potlek.feltoltve}</h2>
				</div>
				<div
					class="rounded-lg"
					class:bg-red-700={color === 'HU'}
					class:bg-white={color === 'SK'}
					class:text-black={color === 'SK'}
					class:bg-yellow-400={color === 'RO'}
				>
					<h1 class="font-itim text-2xl font-bold">Új leintések száma:</h1>
					<h2 class="font-itim text-3xl font-bold">{data.stat.leintes.feltoltve}</h2>
				</div>
				<div
					class="rounded-lg"
					class:bg-red-700={color === 'HU' || color === 'RO'}
					class:bg-white={color === 'SK'}
					class:text-black={color === 'SK'}
				>
					<h1 class="font-itim text-2xl font-bold">Új számlák száma:</h1>
					<h2 class="font-itim text-3xl font-bold">{data.stat.szamla.feltoltve}</h2>
				</div>
				<div
					class="rounded-lg"
					class:bg-white={color === 'HU'}
					class:text-black={color === 'HU'}
					class:bg-blue-700={color === 'SK'}
					class:bg-blue-900={color === 'RO'}
				>
					<h1 class="font-itim text-2xl font-bold">Elfogadott pótlékok száma:</h1>
					<h2 class="font-itim text-3xl font-bold">{data.stat.potlek.elfogadva}</h2>
				</div>
				<div
					class="rounded-lg"
					class:bg-white={color === 'HU'}
					class:text-black={color === 'HU'}
					class:bg-blue-700={color === 'SK'}
					class:bg-yellow-400={color === 'RO'}
				>
					<h1 class="font-itim text-2xl font-bold">Elfogadott leintések száma:</h1>
					<h2 class="font-itim text-3xl font-bold">{data.stat.leintes.elfogadva}</h2>
				</div>
				<div
					class="rounded-lg"
					class:bg-white={color === 'HU'}
					class:text-black={color === 'HU'}
					class:bg-blue-700={color === 'SK'}
					class:bg-red-700={color === 'RO'}
				>
					<h1 class="font-itim text-2xl font-bold">Elfogadott számlák száma:</h1>
					<h2 class="font-itim text-3xl font-bold">{data.stat.szamla.elfogadva}</h2>
				</div>
				<div
					class="rounded-lg"
					class:bg-blue-900={color === 'RO'}
					class:bg-red-600={color === 'SK'}
					class:bg-green-600={color === 'HU'}
				>
					<h1 class="font-itim text-2xl font-bold">Elutasított pótlékok száma:</h1>
					<h2 class="font-itim text-3xl font-bold">{data.stat.potlek.elutasitva}</h2>
				</div>
				<div
					class="rounded-lg"
					class:bg-red-600={color === 'SK'}
					class:bg-yellow-400={color === 'RO'}
					class:bg-green-600={color === 'HU'}
				>
					<h1 class="font-itim text-2xl font-bold">Elutasított leintések száma:</h1>
					<h2 class="font-itim text-3xl font-bold">{data.stat.leintes.elutasitva}</h2>
				</div>
				<div
					class="rounded-lg"
					class:bg-red-600={color === 'SK'}
					class:bg-red-700={color === 'RO'}
					class:bg-green-600={color === 'HU'}
				>
					<h1 class="font-itim text-2xl font-bold">Elutasított számlák száma:</h1>
					<h2 class="font-itim text-3xl font-bold">{data.stat.szamla.elutasitva}</h2>
				</div>
			{/if}
		</div>
	</div>
{/if}
