<script lang="ts">
	import SMList from '$lib/admin/itemlist.svelte';
	import { get_type_number } from '$lib/ucp/types.js';
	let { data } = $props();
</script>

<SMList
	title="Pótlékok"
	des={`Taxi: 8-15 Délelőtti, 22-4 Éjszakai; AM: 8-16 Délelőtti, 20-4 Éjszakai`}
	type={get_type_number('pótlék')}
	editdes="pótlékának"
	extraText="Típus"
	{data}
	tools={['délelőtti', 'éjszakai', 'event', 'decline', 'edit']}
/>
