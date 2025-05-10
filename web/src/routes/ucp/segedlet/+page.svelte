<script lang="ts">
	let { data } = $props();
</script>

<div class="text-center text-black dark:text-white">
	<h1 class="mb-4 mt-4 text-4xl font-bold">Segédlet az oldal használatához</h1>
	{#if ['SCKK', 'TOW', 'UNI'].includes(data.faction!)}
		<h2 class="text-black dark:text-white">
			A segédlet felépítése pontokra bontott, azon belül pedig alpontokra bontott.
		</h2>
		<h2 class="text-black dark:text-white">
			Az itt feltüntetett pontok betartása kötelező, ellenkező esetben a feltöltött elem
			elutasításra kerül.
		</h2>
		<h2 class="mb-3 text-black dark:text-white">
			Amennyiben az egyik szabálypontot megszeged a feltöltött elem kezelőjének (elutasítójának)
			kötelező indokként feltüntetni a sértett szabálypontokat.
		</h2>
		<div class="text-left align-bottom">
			<h1 class="ml-[20%] mt-5 text-left text-3xl font-bold">1. Pótlékok</h1>
			<ul class="child:mx-[20%] child:text-left ml-2">
				<li class="flex items-center gap-2">
					<h2 class="text-left text-xl font-bold">1.</h2>
					<h2 class="text-xl">
						{#if data.faction === 'TOW'}
							A fotónak tartalmaznia kell a kiadott számlát és a teljes játékot. <a
								class="text-taxi hidden text-xl font-bold uppercase"
								target="_blank"
								href="{data.cdn}/img?id=8740">példa</a
							>
						{:else}
							A fuvar igazolásáról kell egy teljes képernyős képet készíteni. (4-es pont).
						{/if}
					</h2>
				</li>
				<li class="flex items-center gap-2">
					<h2 class="text-left text-xl font-bold">2.</h2>
					<h2 class="text-xl">
						Pótlékok a következő időben érvényesek: {#if data.faction === 'TOW'}
							8-16 óra (délelőtti), 20-4 (éjszakai).
						{:else}
							8-15 óra (délelőtti), 22-4 (éjszakai).
						{/if}
					</h2>
				</li>
			</ul>
			<h1 class="ml-[20%] mt-5 text-left text-3xl font-bold">
				2. Leintések {#if data.faction === 'TOW'}/ Bejelentések
				{/if}
			</h1>
			<ul class="child:mx-[20%] child:text-left ml-2">
				<li class="flex gap-2">
					<h2 class="text-xl font-bold">1.</h2>
					<h2 class="text-xl">Egy érvényes leintés 2 képből áll!</h2>
				</li>
				<li class="flex gap-2">
					<h2 class="text-xl font-bold">2.</h2>
					<h2 class="text-xl">
						{#if data.faction === 'TOW'}
							Az első képen a kiadott számlának kell látszódnia. <a
								class="text-taxi hidden text-xl font-bold uppercase"
								target="_blank"
								href="{data.cdn}/limg?id=1015&ver=1">példa</a
							>
						{:else}
							Az első képen a 10-12 rádiózásának, vagy hívás átvételnél a rádió rp-nek kell
							látszódnia.
						{/if}
					</h2>
					{#if data.faction !== 'TOW'}
						<a
							class="text-taxi hidden text-xl font-bold uppercase"
							target="_blank"
							href="{data.cdn}/limg?id=5965&ver=0">példa a 10-12-ről</a
						>
						<a
							class="text-taxi hidden text-xl font-bold uppercase"
							target="_blank"
							href="{data.cdn}/limg?id=5509&ver=0">példa a rádió rpről</a
						>
					{/if}
				</li>
				<li class="flex gap-2">
					<h2 class="text-xl font-bold">3.</h2>
					<h2 class="text-xl">
						{#if data.faction === 'TOW'}
							A második képen a fizetésről kell képet készíteni. (chat)
							<a
								class="text-taxi hidden text-xl font-bold uppercase"
								target="_blank"
								href="{data.cdn}/limg?id=1015&ver=0">példa</a
							>
						{:else}
							A második képen a hívást kell igazolni (4-es pont).
						{/if}
					</h2>
				</li>
			</ul>
			<h1 class="ml-[20%] mt-5 text-left text-3xl font-bold">3. Számlák</h1>
			<ul class="child:mx-[20%] child:text-left ml-2">
				<li class="flex gap-2">
					<h2 class="text-xl font-bold">1.</h2>
					<h2 class="text-xl">Csak a saját szereléseket tölthetsz fel, autó függetlenül.</h2>
				</li>
				<li class="flex gap-2">
					<h2 class="text-xl font-bold">2.</h2>
					<h2 class="text-xl">Csak az APMS általi szerelésekért tudjuk visszaadni a pénzt.</h2>
				</li>
				<li class="flex gap-2">
					<h2 class="text-xl font-bold">3.</h2>
					<h2 class="text-xl">Csak az aznap lefotózott számlákat tudjuk elfogadni!</h2>
				</li>
				<li class="flex gap-2">
					<h2 class="text-xl font-bold">4.</h2>
					<h2 class="text-xl">
						Szereltetési számlák összege egy héten nem lehet több, mint 400.000$.
					</h2>
				</li>
			</ul>

			{#if data.faction === 'SCKK' || data.faction === 'UNI'}
				<h1 class="ml-[20%] mt-5 text-left text-3xl font-bold">4. Hívások igazolása</h1>
				<ul class="child:mx-[20%] child:text- ml-2">
					<li class="flex gap-2">
						<h2 class="text-xl font-bold">1.</h2>
						<h2 class="text-xl">
							Hívásokat az alábbi kétféleképpen lehet igazolni, más módot nem fogadunk el!
						</h2>
					</li>
					<li class="flex gap-2">
						<h2 class="text-xl font-bold">2.</h2>
						<h2 class="text-xl">
							Lehetséges igazolni a fuvar végén kinyomtatott nyugta fotózásával.
						</h2>
						<a
							class="text-taxi hidden text-xl font-bold uppercase"
							target="_blank"
							href="{data.cdn}/img?id=5544">példa</a
						>
					</li>
					<li class="flex gap-2">
						<h2 class="text-xl font-bold">3.</h2>
						<h2 class="text-xl">
							Lehetséges igazolni a fuvar kifizetésénél felbukkanó kék dobozzal, vagy a chaten
							látható azonos "kifizette az utazást" szöveget tartalmaztó chatsorral.
						</h2>
						<a
							class="text-taxi hidden text-xl font-bold uppercase"
							target="_blank"
							href="{data.cdn}/img?id=5883">példa</a
						>
					</li>
				</ul>
			{/if}
		</div>
	{/if}
	{#if data.faction === 'APMS'}
		<h2 class="text-3xl">
			Fel kell tölteni a szereltetési számlákhoz az adott szereltetésről a képet. (F12)
		</h2>
		<h2 class="text-3xl font-bold">CSAK aznap lefotózott képet lehet feltölteni!</h2>
	{/if}
</div>
