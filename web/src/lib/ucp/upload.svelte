<script lang="ts">
	import { beforeNavigate } from '$app/navigation';
	import Error from '$lib/error.svelte';
	import { loading } from '$lib/loading.svelte';
	import { get_type_number } from './types';
	let formerror = $state('');
	interface Props {
		data: any;
		display?: string;
		tipus: number;
		info?: string;
		agent?: string;
	}

	let { data, display = '', tipus, info = '', agent = '' }: Props = $props();
	let fileas: string[] = $state([]);
	let uploading = $state(false);
	beforeNavigate((ev) => {
		if (uploading) {
			ev.cancel();
		}
	});
	async function upload() {
		loading.value = true;
		uploading = true;
		let files = document.getElementById('file') as HTMLInputElement;
		if (files.files) {
			const formData = new FormData();
			let dates: string[] = [];
			for (let i = 0; i < files.files.length; i++) {
				const offsetMs =
					new Date(Number(files.files[i].lastModified.toString())).getTimezoneOffset() * 60 * 1000;
				formData.append('files', files.files[i]);
				dates.push((Number(files.files[i].lastModified.toString()) + offsetMs).toString());
			}
			formerror = '';
			const mama = await fetch('/web-api/upload', {
				method: 'POST',
				headers: {
					tip: tipus.toString(),
					dates: JSON.stringify(dates)
				},
				body: formData
			});
			loading.value = false;
			uploading = false;
			const ret = await mama.json();
			if (ret.error === 'toobig') {
				formerror =
					'A feltöltött fájlok túl lépték a 64MB-os határértéket. Lehetséges, hogy pár fel lett tölve, ezért nézd meg azokat a rendes, nem feltöltési oldalrészen!';
			} else if (ret.error === 'leintestipik') {
				formerror = 'Leintéshez kérlek 2-vel osztható mennyiségű képet tölts fel!';
			} else {
				fileas = ret;
				console.log(fileas);
			}
		}
	}
	let selectedFiles: string[] = $state([]);

	function handleFileChange(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files) {
			selectedFiles = Array.from(input.files).map((file) => file.name);
		}
	}
</script>

<Error {data}>
	<div class="text-center text-white">
		<div
			class={`via-taxi bg-linear-to-r ml-64 mr-64 mt-16 rounded-lg from-rose-600 to-emerald-500 bg-[size:200%] ${uploading ? 'bg-[position:100%]' : 'bg-[position:0]'} p-2 transition-all duration-300`}
		>
			<h2 class="font-bold text-red-800 drop-shadow-xl">{formerror ? formerror : ''}</h2>
			<h1 class="text-3xl font-bold uppercase drop-shadow-lg">{display} feltöltése</h1>
			<h2 class="mb-2 font-bold text-yellow-100 drop-shadow-xl">
				{info}
			</h2>
			{#if agent.includes('Firefox')}
				<h1 class="mb-2 text-xl font-bold">
					Firefoxon (és az azon alapuló böngészőkön) jelenlegi állás szerint nem lehet elemeket
					feltölteni. Ez idő alatt kérlek használj egy Chromium alapú böngészőt! (Pl. Chrome, Opera,
					Edge, Brave, Arc, stb.)
				</h1>
			{:else}
				<form
					onsubmit={() => upload()}
					enctype="multipart/form-data"
					class="flex justify-center gap-4"
				>
					<input
						class="hidden"
						type="file"
						name="file"
						id="file"
						accept="image/*"
						required
						multiple
						onchange={handleFileChange}
					/>
					<label
						for={uploading ? '' : 'file'}
						class:bg-pos-100={uploading}
						class:!cursor-not-allowed={uploading}
						class="bg-linear-to-r mb-2 cursor-pointer rounded-xl from-gray-600 via-amber-400 to-rose-600 bg-[size:200%] bg-[position:0] px-3 py-1 text-xl font-bold uppercase drop-shadow-lg transition-all duration-300 hover:bg-[position:100%]"
						>Fájlok kiválasztása</label
					>
					<button
						type="submit"
						aria-label="Feltöltés"
						class="bg-linear-to-r mb-2 w-16 rounded-xl from-gray-600 via-amber-400 to-emerald-400 bg-[size:200%] bg-[position:0] px-3 py-1 text-xl font-bold uppercase drop-shadow-lg transition-all duration-300 hover:bg-[position:100%] disabled:cursor-not-allowed"
						class:bg-pos-100={uploading}
						disabled={uploading}
					>
						<span class="icon-[material-symbols--upload] h-full w-full"></span>
					</button>
				</form>
				{#if selectedFiles.length > 0}
					<h2 class="font-bold">Kiválaszottt fájlok:</h2>
					<div class="flex flex-wrap items-center justify-center gap-2">
						{#each selectedFiles as file}
							<h2 class="rounded-lg bg-black/30 px-2 font-light">
								{file}
							</h2>
						{/each}
					</div>
				{/if}
			{/if}
		</div>
		<div class="flex-row items-center justify-center align-middle">
			{#if !agent.includes('Firefox')}
				<h2 class="font-bold text-black dark:text-white">
					Ha sikeresen feltöltötted őket akkor itt fognak megjelenni:
				</h2>
			{/if}
			{#each fileas as nyam}
				{#if tipus === get_type_number('leintés')}
					<div class="flex flex-col">
						<img
							loading="lazy"
							src={`${data.cdn}/get?id=${nyam[0]}`}
							alt=""
							class="max-h-5xl m-auto max-w-5xl py-3"
						/>
						<img
							loading="lazy"
							src={`${data.cdn}/get?id=${nyam[1]}`}
							alt=""
							class="max-h-5xl m-auto max-w-5xl py-3"
						/>
					</div>
				{:else}
					<img
						loading="lazy"
						src={`${data.cdn}/get?id=${nyam[0]}`}
						alt=""
						class="max-h-5xl m-auto max-w-5xl py-3"
					/>
				{/if}
			{/each}
		</div>
	</div>
</Error>
