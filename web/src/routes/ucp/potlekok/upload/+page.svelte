<script lang="ts">
	import { get_type_number } from '$lib/ucp/types.js';
	import Upload from '$lib/ucp/upload.svelte';

	let { data } = $props();
</script>

<Upload
	{data}
	display="Pótlék"
	tipus={get_type_number('pótlék')}
	agent={data.agent}
	info={data.faction === 'TOW'
		? '8:00-16:00: Délelőtti pótlék, 20:00-4:00: Éjszakai pótlék'
		: '8:00-15:00: Délelőtti pótlék, 22:00-4:00: Éjszakai pótlék'}
/>
