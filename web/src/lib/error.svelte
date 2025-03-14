<script lang="ts">
	import { page } from '$app/state';

	interface Props {
		data: { error?: any; noauth?: any; apiUrl?: any };
		children?: import('svelte').Snippet;
	}
	let { data, children }: Props = $props();
</script>

{#if !data.error}
	{@render children?.()}
{:else}
	<div
		class="child:p-2 md:child:p-16 grid grid-cols-1 grid-rows-1 text-center text-black dark:text-white"
	>
		<h2 class="text-center text-3xl font-bold text-black dark:text-white">
			Sikertelen API lekérdezés
		</h2>
		<h2 class="text-center text-xl text-gray-300">{data.error}</h2>
	</div>
{/if}
