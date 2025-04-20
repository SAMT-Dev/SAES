<script lang="ts">
	import { goto } from '$app/navigation';
	import { emit, listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';

	listen<string>('changepanel', (ev) => {
		goto(ev.payload, { replaceState: true });
	});
	onMount(async () => {
		await emit('panel');
	});
</script>

<div class="h-screen w-screen bg-gray-950 pointer-events-none select-none">
	<div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
		<img src="/favicon.png" alt="" />
	</div>
</div>
