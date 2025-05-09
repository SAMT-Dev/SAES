<script lang="ts">
	import { cdnUrl } from '$lib/api';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	let selector: 'all' | 'managed' | 'unmanaged' = $state('all');
</script>

<div class="m-auto w-[80dvw] text-center text-white">
	{#if data.error}
		<h1>Hiba: {data.error}</h1>
	{:else}
		<h1 class="text-3xl font-bold">Frakciók</h1>
		<div class="m-auto mb-4 flex w-80 items-center justify-center rounded-xl">
			<button
				onclick={() => (selector = 'all')}
				class={`w-full cursor-pointer rounded-l-2xl ${selector === 'all' ? 'bg-violet-500' : 'bg-gray-500'}`}
			>
				Összes
			</button>
			<button
				onclick={() => (selector = 'managed')}
				class={`w-full cursor-pointer  ${selector === 'managed' ? 'bg-violet-500' : 'bg-gray-500'}`}
			>
				Kezeltek
			</button>
			<button
				onclick={() => (selector = 'unmanaged')}
				class={`w-full cursor-pointer rounded-r-2xl  ${selector === 'unmanaged' ? 'bg-violet-500' : 'bg-gray-500'}`}
			>
				Kezeletlenek
			</button>
		</div>
		<div class="m-auto flex items-center justify-center gap-3">
			{#each Object.keys(data.data!) as f}
				{#if selector === 'all' || (selector === 'managed' && data.data![f].managed) || (selector === 'unmanaged' && !data.data![f].managed)}
					<div
						class:bg-green-500={data.data![f].managed}
						class:bg-gray-700={!data.data![f].managed}
						class="rounded-xl p-4"
					>
						<h1 class="text-3xl font-bold uppercase">
							{data.data![f].managed ? 'Kezelt' : 'Kezeletlen'}
						</h1>
						<h1>{data.data![f].name} ({f})</h1>
						<h1>Rövid név: {data.data![f].shortname}</h1>
						{#if data.data![f].icon}
							<img src={`${cdnUrl}/get?id=${data.data![f].icon}`} alt="Faction logo" class="w-64" />
						{/if}
					</div>
				{/if}
			{/each}
		</div>
	{/if}
</div>
