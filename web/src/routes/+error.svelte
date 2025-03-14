<script lang="ts">
	import { page } from '$app/stores';
</script>

<svelte:head>
	<title>{$page.status} - SAMT</title>
</svelte:head>

{#if $page.error}
	<div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 transform text-center">
		<h1 class="text-3xl font-bold text-red-600">
			{$page.status}
		</h1>
		<h1 class="text-3xl text-red-600">{$page.error.message}</h1>
		<a
			href="/"
			class="bg-linear-to-r mb-5 ml-5 mr-5 mt-5 block rounded-full from-indigo-600 via-teal-400 to-blue-400 bg-[size:200%] bg-[position:0] px-2 py-1 text-center text-lg font-bold text-black drop-shadow-lg transition-all duration-500 hover:bg-[position:100%] dark:text-white"
			>Vissza a kezdőlapra</a
		>
	</div>
{/if}
