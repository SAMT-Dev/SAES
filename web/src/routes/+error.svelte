<script lang="ts">
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button/index.js';
</script>

<svelte:head>
	<title>{page.status} - SAMT</title>
</svelte:head>

{#if page.error}
	<div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 transform text-center">
		<h1 class="text-3xl font-bold text-red-600">
			{page.status}
		</h1>
		<h1 class="text-3xl text-red-600">{page.error.message}</h1>
		<Button href="/">Vissza a kezdőlapra</Button>
	</div>
{/if}
