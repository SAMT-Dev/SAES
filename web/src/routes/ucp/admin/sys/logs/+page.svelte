<script lang="ts">
	import Loglist from '$lib/fm/loglist.svelte';

	let { data } = $props();
</script>

<Loglist {data} multifaction={true} filters={['login', 'update_item', 'upload_item']} />
