<script lang="ts">
	import { Factions } from '$lib/permissions.js';

	let { data } = $props();
</script>

<div class="flex items-center text-center text-white">
	<div class="m-auto">
		<h1 class="font-itim mb-5 mt-5 text-4xl font-bold">Hasznos linkek</h1>
		<div class="flex w-screen items-center justify-center gap-5">
			{#if data.faction === Factions.Taxi}
				<a
					href="/s/app"
					target="_blank"
					class="border-taxi hover:bg-taxi w-1/2 rounded-3xl border-4 p-5 transition-colors duration-500 lg:w-1/4"
				>
					<h1 class="text-2xl font-bold">SCKK Application letöltése</h1>
					<h2 class="font-itim">
						Az SCKK Application live olvasás segítségével képes egyből rögzíteni a hívásaidat.
					</h2>
				</a>
			{/if}
			{#if data.faction === Factions.Taxi || data.faction === Factions.Tow}
				<a
					href="/s/steam"
					target="_blank"
					class="col-span-2 w-1/2 rounded-3xl border-4 border-[#1a9fff] p-5 transition-colors duration-500 hover:bg-[#1a9fff] lg:w-1/4"
				>
					<h1 class="text-2xl font-bold">SCKK Steam csoport</h1>
					<h2 class="font-itim">Miért van Steam csoport? Miért ne legyen? Csak csatlakozz! 😉</h2>
				</a>
			{/if}
		</div>
	</div>
</div>
