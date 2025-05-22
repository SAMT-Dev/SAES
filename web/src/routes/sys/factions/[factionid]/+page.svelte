<script lang="ts">
	import { cdnUrl } from '$lib/api';
	import { loading } from '$lib/loading.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	async function upload(ev: Event) {
		ev.preventDefault();
		loading.value = true;
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
			const mama = await fetch('/web-api/sys/upload', {
				method: 'POST',
				headers: {
					dates: JSON.stringify(dates),
					factionid: data.id!
				},
				body: formData
			});
			loading.value = false;
			const ret = await mama.json();
			console.log(ret, mama.status);
		}
	}
</script>

<div class="m-auto items-center justify-center text-center text-white">
	<h1>Új ikon feltöltése</h1>
	<img src={`${cdnUrl}/get?id=${data.factinfo?.icon}`} alt="" class="m-auto w-64" />
	<form
		onsubmit={(ev) => upload(ev)}
		enctype="multipart/form-data"
		class="flex justify-center gap-4"
	>
		<input class="hidden" type="file" name="file" id="file" accept="image/*" required />
		<label
			for="file"
			class="bg-linear-to-r mb-2 cursor-pointer rounded-xl from-gray-600 via-amber-400 to-rose-600 bg-[size:200%] bg-[position:0] px-3 py-1 text-xl font-bold uppercase drop-shadow-lg transition-all duration-300 hover:bg-[position:100%]"
			>Fájl kiválasztása</label
		>
		<button
			type="submit"
			aria-label="Feltöltés"
			class="bg-linear-to-r mb-2 w-16 rounded-xl from-gray-600 via-amber-400 to-emerald-400 bg-[size:200%] bg-[position:0] px-3 py-1 text-xl font-bold uppercase drop-shadow-lg transition-all duration-300 hover:bg-[position:100%] disabled:cursor-not-allowed"
		>
			<span class="icon-[material-symbols--upload] h-full w-full"></span>
		</button>
	</form>
</div>
