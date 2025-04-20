<script lang="ts">
	import { goto } from '$app/navigation';
	import { listen } from '@tauri-apps/api/event';

	listen<string>('changepanel', (ev) => {
		goto(ev.payload, { replaceState: true });
	});
</script>

<div class="h-screen w-screen bg-gray-950 pointer-events-none select-none">
	<div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
		<img src="/favicon.png" alt="" />
	</div>
</div>
