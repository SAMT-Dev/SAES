<script lang="ts">
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	let facts: (keyof typeof factResolver)[] = $state([]);
	onMount(async () => {
		facts = await invoke<(keyof typeof factResolver)[]>('get_faction_options');
		if (facts.length === 1) {
			await selectFact(facts[0]);
		}
	});
	let factResolver = {
		SCKK: {
			display: 'Taxi',
			logo: 'sckk_icon.png'
		},
		TOW: {
			display: 'Autómentés',
			logo: 'sckk_icon.png'
		},
		APMS: {
			display: 'APMS',
			logo: 'apms_icon.png'
		},
		UNI: {
			display: 'Akadémia',
			logo: 'uni_icon.png'
		}
	};
	async function selectFact(fact: string) {
		await invoke('set_faction', { faction: fact });
		await goto('/main', { replaceState: true });
	}
</script>

<div class="h-screen w-screen bg-gray-950 pointer-events-none select-none text-white text-center">
	<div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
		<div class="flex gap-3 items-center justify-center">
			{#each facts as f}
				{#if factResolver[f]}
					<button
						class="cursor-pointer pointer-events-auto bg-gray-700 p-4 rounded-xl"
						onclick={async () => selectFact(f)}
					>
						<img src={`https://samt.hu/${factResolver[f].logo}`} alt="" class="w-28 m-auto" />
						<h1 class="text-3xl font-bold">{factResolver[f].display}</h1>
					</button>
				{/if}
			{/each}
		</div>
	</div>
</div>
