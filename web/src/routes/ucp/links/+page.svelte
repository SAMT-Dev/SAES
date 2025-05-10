<script lang="ts">
	import Grid from '$lib/admin/grid.svelte';
	import { getFactionPerm, Permissions } from '$lib/permissions.js';

	let { data } = $props();
</script>

<Grid
	{data}
	title="Hasznos linkek"
	items={[
		{
			title: 'SCKK Application letöltése',
			description:
				'Az SCKK Application live olvasás segítségével képes egyből rögzíteni a hívásaidat.',
			href: '/s/app',
			border: 'border-taxi',
			background: 'bg-yellow-200 hover:bg-taxi',
			permission: [
				getFactionPerm(Permissions.SaesFactUcp, 'taxi'),
				getFactionPerm(Permissions.SaesFactUcp, 'tow'),
				getFactionPerm(Permissions.SaesFactUcp, 'uni')
			]
		},
		{
			title: 'Steam csoport',
			description: 'Miért van Steam csoport? Miért ne legyen? Csak csatlakozz! 😉',
			href: '/s/steam',
			border: 'border-blue-600',
			background: 'bg-blue-300 hover:bg-blue-600',
			permission: [
				getFactionPerm(Permissions.SaesFactUcp, 'taxi'),
				getFactionPerm(Permissions.SaesFactUcp, 'tow')
			]
		}
	]}
/>
