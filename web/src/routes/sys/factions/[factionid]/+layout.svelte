<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { LayoutData } from './$types';
	let { data, children }: { data: LayoutData; children: Snippet } = $props();
	let url = `/sys/factions/${data.id}`;
</script>

<div
	class="mb-2 flex w-screen items-center justify-between bg-indigo-600 px-8 text-center text-white"
>
	<h1 class="font-bold">{data.factinfo?.name}</h1>
	<div class="flex items-center justify-center gap-2">
		<a href={url}>Információk</a>
		<a href={`${url}/positions`}>Pozíciók</a>
		<a href={`${url}/shifts`}>Műszakok</a>
	</div>
</div>

{@render children()}
