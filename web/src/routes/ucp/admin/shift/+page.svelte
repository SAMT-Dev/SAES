<script lang="ts">
	import Grid from '$lib/admin/grid.svelte';
	import { getFactionPerm, Permissions } from '$lib/permissions.js';

	let { data } = $props();
</script>

<Grid
	{data}
	title="Műszakvezetés"
	items={[
		{
			title: 'Aktuális hét',
			description: 'Az aktuális heti statisztika',
			href: '/ucp/admin/shift/stat/current',
			border: 'border-violet-600',
			background: 'bg-violet-300 hover:bg-violet-600',
			permission: [
				getFactionPerm(Permissions.SaesFactAdminShift, 'taxi'),
				getFactionPerm(Permissions.SaesFactAdminShift, 'tow'),
				getFactionPerm(Permissions.SaesFactAdminShift, 'uni')
			]
		},
		{
			title: 'Előző hét',
			description: 'Az előző heti statisztika',
			href: '/ucp/admin/shift/stat/previous',
			border: 'border-rose-600',
			background: 'bg-rose-300 hover:bg-rose-600',
			permission: [
				getFactionPerm(Permissions.SaesFactAdminShift, 'taxi'),
				getFactionPerm(Permissions.SaesFactAdminShift, 'tow'),
				getFactionPerm(Permissions.SaesFactAdminShift, 'uni')
			]
		}
	]}
/>
