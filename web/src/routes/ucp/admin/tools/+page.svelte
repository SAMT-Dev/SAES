<script lang="ts">
	import Grid from '$lib/admin/grid.svelte';
	import { getFactionPerm, Permissions } from '$lib/permissions.js';

	let { data } = $props();
</script>

<Grid
	{data}
	title="Eszközök"
	items={[
		{
			title: 'APP Feldolgozó',
			description:
				'Az SCKK Application kódját felhasználva meg lehet nézni, ki, mikor vitt hívást az adott napon, illetve a műszakok hogyan teljesítettek.',
			href: '/ucp/admin/tools/app',
			border: 'border-taxi',
			background: 'bg-yellow-200 hover:bg-taxi',
			permission: [
				getFactionPerm(Permissions.SaesFactAdmin, 'taxi'),
				getFactionPerm(Permissions.SaesFactAdmin, 'tow'),
				getFactionPerm(Permissions.SaesFactAdmin, 'uni')
			]
		},
		{
			title: 'Hívásszámláló',
			description:
				'Akár több .log fájlból meg lehet nézni, ki hány hívást vitt el, akár több napra nézve!',
			href: '/ucp/admin/tools/call',
			border: 'border-blue-600',
			background: 'bg-blue-300 hover:bg-blue-600',
			permission: [
				getFactionPerm(Permissions.SaesFactAdmin, 'taxi'),
				getFactionPerm(Permissions.SaesFactAdmin, 'tow'),
				getFactionPerm(Permissions.SaesFactAdmin, 'uni')
			]
		}
	]}
/>
