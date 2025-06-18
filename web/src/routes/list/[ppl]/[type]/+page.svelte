<script lang="ts">
	let { data } = $props();
</script>

<div class="flex flex-col items-center justify-center">
	{#each data.cucc as puszedli, i (i)}
		{#if data.type === 'szamla'}
			<h2 class="mt-5 text-center text-xl text-black dark:text-white">
				{i + 1}. ({puszedli.price}$):
			</h2>
		{:else}
			<h2 class="mt-5 text-center text-xl text-black dark:text-white">
				{i + 1}:
			</h2>
		{/if}
		{#if puszedli.reason}
			<h2 class="text-center dark:text-gray-400">Megjegyzés: {puszedli.reason}</h2>
		{/if}
		{#if data.type === 'leintes'}
			<img src={`${data.image}/get?id=${puszedli.img_1}`} alt="kép" />
			<h2 class="dark:text-gray-400">Kép ID: {puszedli.img_1}</h2>
			<img src={`${data.image}/get?id=${puszedli.img_2}`} alt="kép" />
			<h2 class="dark:text-gray-400">Kép ID: {puszedli.img_2}</h2>
		{:else}
			<img src={`${data.image}/get?id=${puszedli.img_1}`} alt="kép" />
			<h2 class="dark:text-gray-400">Kép ID: {puszedli.img_1}</h2>
		{/if}
		<h2 class="dark:text-gray-400">ID: {puszedli.id}</h2>
	{/each}
</div>
