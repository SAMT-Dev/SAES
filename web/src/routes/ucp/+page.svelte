<script lang="ts">
	import { allowFacts, christmas } from '$lib/api.js';

	let { data } = $props();
	let date = new Date();
	let hour = date.getHours();
	let greet = $state('');
	let end = $state('!');
	if (christmas) {
		greet = 'Kellemes ünnepeket';
		if (
			(date.getMonth() === 11 && (date.getDate() === 24 || date.getDate() === 25)) ||
			(date.getMonth() === 0 && (date.getDate() === 6 || date.getDate() === 7))
		) {
			greet = 'Boldog karácsonyt';
		}
		if (date.getMonth() === 0 && date.getDate() <= 2) {
			greet = 'Boldog, és sikerekben gazdag új évet kívánok';
		}
	} else {
		if (hour >= 19 || (hour >= 0 && hour < 2)) {
			greet = 'Szép estét';
		}
		if (hour >= 13 && hour < 19) {
			greet = 'Szép délutánt';
		}
		if (hour >= 12 && hour < 13) {
			greet = 'Szép delet';
		}
		if (hour >= 8 && hour < 12) {
			greet = 'Jó reggelt';
		}
		if (hour >= 4 && hour < 8) {
			greet = 'Jó reggelt, bár nem kapsz pótlékot';
		}
		if (hour >= 2 && hour < 4) {
			greet = 'Miért nem alszol';
			end = '?';
		}
	}
	// const sendResponse = async () => {
	// 	await fetch('/api/upload', {
	// 		method: 'POST',
	// 		headers: {
	// 			tip: 'midnightReason'
	// 		},
	// 		body: reason
	// 	});
	// };
	import * as Card from '$lib/components/ui/card/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
</script>

<div class="child:p-2 md:child:p-16 text-center text-black dark:text-white">
	{#if !data.error}
		<Card.Root>
			<Card.Header>
				<Card.Title>{greet}, {data.layout?.admin ? 'SA ' : ''}{data.layout?.name}{end}</Card.Title>
			</Card.Header>
			<Card.Content>
				{#if data.layout?.factions}
					<h2 class="text-xl drop-shadow-lg md:text-2xl">
						Pozíciód: {data.layout.factions.positionname}
					</h2>
					<h2 class="text-xl drop-shadow-lg md:text-2xl">
						Műszakod: {data.layout.factions.shiftname}
					</h2>
				{/if}
				{#if allowFacts(data, ['SCKK', 'TOW', 'UNI'])}
					{#if Number(data.calls?.app) + Number(data.calls?.leintes) > 0}
						{#if data.calls?.app === null}
							<h2 class="text-xl drop-shadow-lg md:text-2xl">
								Hívásaid (app nem megy, csak leintés): {data.calls?.leintes}
							</h2>
						{:else}
							<h2 class="text-xl drop-shadow-lg md:text-2xl">
								Hívásaid (app+leintés): {data.calls?.app}+{data.calls?.leintes}={Number(
									data.calls?.app
								) + Number(data.calls?.leintes)}
							</h2>
						{/if}
					{/if}
				{/if}
				{#if (data.calls?.potlek.de && data.calls?.potlek.de > 0) || (data.calls?.potlek.du && data.calls?.potlek.du > 0)}
					<h2 class="text-xl drop-shadow-lg md:text-2xl">
						Elfogadott pótlékaid: délelőtti: {data.calls?.potlek.de}, éjszakai: {data.calls?.potlek
							.du}
					</h2>
				{/if}
				{#if data.calls?.price! > 0}
					<h2 class="text-xl drop-shadow-lg md:text-2xl">
						Elfogadott számlák összege: {data.calls?.price}$.
					</h2>
				{/if}
				{#if allowFacts(data, ['APMS']) && data.szamlak}
					<h2 class="text-xl drop-shadow-lg md:text-2xl">
						Kezelt számlák (feltöltött+elfogadott): {data.szamlak?.uploaded}+{data.szamlak
							?.accepted}={Number(data.szamlak?.uploaded) + Number(data.szamlak?.accepted)}
					</h2>
				{/if}
			</Card.Content>
			<Card.Footer>
				<Button variant="destructive">Kijelentkezés</Button>
			</Card.Footer>
		</Card.Root>
	{/if}
</div>
{#if data.error}
	<h2 class="text-center text-3xl font-bold text-black dark:text-white">
		Sikertelen API lekérdezés
	</h2>
	<h2 class="text-center text-xl text-gray-300">{data.error}</h2>
{/if}
