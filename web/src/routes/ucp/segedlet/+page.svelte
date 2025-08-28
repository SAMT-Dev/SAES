<script lang="ts">
	let { data } = $props();
	import * as Accordion from '$lib/components/ui/accordion/index.js';
</script>

<div class="text-center text-black dark:text-white">
	<h1 class="mb-4 mt-4 text-4xl font-bold">Segédlet az oldal használatához</h1>
	<h2 class="text-black dark:text-white">
		A segédlet felépítése pontokra bontott, azon belül pedig alpontokra bontott.
	</h2>
	<h2 class="text-black dark:text-white">
		Az itt feltüntetett pontok betartása kötelező, ellenkező esetben a feltöltött elem elutasításra
		kerül.
	</h2>
	<h2 class="mb-3 text-black dark:text-white">
		Amennyiben az egyik szabálypontot megszeged a feltöltött elem kezelőjének (elutasítójának)
		kötelező indokként feltüntetni a sértett szabálypontokat.
	</h2>
	<div class="m-auto text-left align-bottom">
		<Accordion.Root type="single">
			<Accordion.Item value="1">
				<Accordion.Trigger>1. Pótlékok</Accordion.Trigger>
				<Accordion.Content
					><ul class="child:mx-[20%] child:text-left ml-2">
						<li class="flex items-center gap-2">
							<h2 class="text-left text-xl font-bold">1.</h2>
							<h2 class="text-xl">
								{#if data.faction === 'TOW'}
									A fotónak tartalmaznia kell a kiadott számlát és a teljes játékot. <a
										class="text-taxi hidden text-xl font-bold uppercase"
										target="_blank"
										href="{data.cdn}/img?id=8740">példa</a
									>
								{:else if data.faction === 'SCKK' || data.faction === 'UNI'}
									A fuvar igazolásáról kell egy teljes képernyős képet készíteni. (4-es pont).
								{:else if data.faction === 'APMS'}
									A szervíz számláról teljes képet kell készíteni.
								{/if}
							</h2>
						</li>
						<li class="flex items-center gap-2">
							<h2 class="text-left text-xl font-bold">2.</h2>
							<h2 class="text-xl">
								Pótlékok a következő időben érvényesek: {#if data.faction === 'TOW'}
									8-16 óra (délelőtti), 20-4 óra (éjszakai).
								{:else if data.faction === 'SCKK' || data.faction === 'UNI'}
									8-15 óra (délelőtti), 22-4 óra (éjszakai).
								{:else if data.faction === 'APMS'}
									9:00-tól 15:00-ig (délelőtti), 22:00-tól 4:00-ig (éjszakai).
								{/if}
							</h2>
						</li>
					</ul></Accordion.Content
				>
			</Accordion.Item>
			{#if ['SCKK', 'TOW', 'UNI'].includes(data.faction!)}
				<Accordion.Item value="2">
					<Accordion.Trigger
						>2. Leintések {#if data.faction === 'TOW'}/ Bejelentések
						{/if}</Accordion.Trigger
					>
					<Accordion.Content
						><ul class="child:mx-[20%] child:text-left ml-2">
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
						</ul></Accordion.Content
					>
				</Accordion.Item>
			{/if}
			<Accordion.Item value="3">
				<Accordion.Trigger>3. Számlák</Accordion.Trigger>
				<Accordion.Content
					><ul class="child:mx-[20%] child:text-left ml-2">
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
					</ul></Accordion.Content
				>
			</Accordion.Item>
			{#if data.faction === 'SCKK' || data.faction === 'UNI'}
				<Accordion.Item value="4">
					<Accordion.Trigger>4. Hívások igazolása</Accordion.Trigger>
					<Accordion.Content
						><ul class="child:mx-[20%] child:text- ml-2">
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
						</ul></Accordion.Content
					>
				</Accordion.Item>
			{/if}
		</Accordion.Root>
	</div>
</div>
