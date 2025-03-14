<script lang="ts">
	import { allowPerms, christmas, countPerms } from '$lib/api';
	import { pages } from './public';
	import { page as statepage } from '$app/state';
	import { Tooltip } from 'flowbite-svelte';
	import { Factions, Permissions } from '$lib/permissions';

	interface Props {
		tip: any;
		isAdmin?: boolean;
		faction: string;
		data: {
			layout?: {
				taxi?: any;
				tow?: any;
				admin: boolean;
				perms: string[];
			};
		};
		nosocket: string | boolean;
	}

	let { tip, isAdmin = false, faction = 'SAMT', data, nosocket }: Props = $props();

	let multifact =
		countPerms(data, [Permissions.SaesTaxiUcp, Permissions.SaesTowUcp, Permissions.SaesApmsUcp]) >=
		2;

	let pagesz = pages(faction);
</script>

<header
	class={`${
		faction === Factions.Taxi
			? 'selection:bg-taxi'
			: faction === Factions.Tow
				? 'selection:bg-tow'
				: faction === Factions.Apms
					? 'selection:bg-apms'
					: ''
	} z-30`}
>
	<div class="relative z-20 border-b bg-white dark:bg-gray-700 dark:text-white">
		<div class="mx-0 px-0 xl:container lg:mx-auto lg:py-4">
			<div class="flex items-center justify-between gap-2">
				<div class="m-auto flex items-center justify-center gap-2">
					<a
						class="group relative z-20 flex items-center gap-3"
						data-sveltekit-reload={multifact ? true : false}
						href={multifact ? '?clear_faction=true' : '/ucp'}
					>
						<div
							class={`${faction === Factions.Tow ? 'group-hover:border-tow' : faction === Factions.Taxi ? 'group-hover:border-taxi' : 'group-hover:border-apms'} pointer-events-none ml-5 rounded-full border-2 border-solid drop-shadow-xl duration-200`}
						>
							<img
								src={faction === Factions.Taxi || faction === Factions.Tow
									? '/sckk_icon.png'
									: faction === Factions.Apms
										? '/apms_icon.png'
										: '/favicon.png'}
								class="border-1 pointer-events-none rounded-full border-solid border-black"
								width="40"
								height="40"
								alt="Logó"
							/>
						</div>
						<h1
							class={`text-3xl font-bold drop-shadow-xl transition-colors duration-200 ${
								faction === Factions.Tow
									? 'group-hover:text-tow'
									: faction === Factions.Taxi
										? 'group-hover:text-taxi'
										: 'group-hover:text-apms'
							}`}
						>
							{tip}
						</h1>
						{#if christmas}
							<img
								src="/santa.svg"
								class="absolute bottom-2 left-3.5 w-14 -rotate-[24deg]"
								alt=""
							/>
						{/if}
					</a>
					{#if multifact}
						<Tooltip>Frakcióváltás</Tooltip>
					{/if}
					<a href="/ucp/settings" aria-label="Beállítások" class="group hidden"
						><span
							class={`icon-[material-symbols--settings] h-6 w-6 transition-colors duration-200 ${
								faction === Factions.Tow
									? 'group-hover:text-tow'
									: faction === Factions.Taxi
										? 'group-hover:text-taxi'
										: 'group-hover:text-apms'
							}`}
						></span></a
					>
					<Tooltip>Beállítások</Tooltip>
				</div>
				<div class="flex items-center justify-end border-l lg:border-l-0">
					<input type="checkbox" name="hamburger" id="hamburger" class="peer opacity-0" hidden />
					<label
						for="hamburger"
						class="peer-checked:hamburger relative z-20 mr-6 block cursor-pointer p-6 lg:hidden"
					>
						<div
							aria-hidden="true"
							class="m-auto h-0.5 w-6 rounded-sm bg-white transition duration-300"
						></div>
						<div
							aria-hidden="true"
							class="m-auto mt-2 h-0.5 w-6 rounded-sm bg-white transition duration-300"
						></div>
					</label>

					<div
						class="fixed inset-0 z-20 w-[calc(100%-4.5rem)] translate-x-[-100%] border-r bg-white shadow-xl transition duration-300 peer-checked:translate-x-0 lg:static lg:w-auto lg:translate-x-0 lg:border-r-0 lg:shadow-none dark:bg-gray-700"
					>
						<div class="flex h-full flex-col justify-between lg:flex-row lg:items-center">
							<ul
								class="flex flex-col items-center gap-5 pt-32 text-center text-gray-700 md:space-y-8 lg:flex-row lg:space-x-3 lg:space-y-0 lg:px-2 lg:pt-0 xl:space-x-12 xl:px-12"
							>
								{#each pagesz as page}
									{#if page.faction.includes(faction as Factions)}
										<li>
											<a
												href={page.url}
												class={`${faction === Factions.Tow ? 'before:bg-tow' : faction === Factions.Taxi ? 'before:bg-taxi' : 'before:bg-apms'} group relative text-white before:absolute before:inset-x-0 before:-bottom-1.5 before:h-2 before:origin-right before:scale-x-0 before:transition before:duration-200 hover:before:origin-left ${statepage.url.pathname === page.url ? 'before:scale-x-100' : 'hover:before:scale-x-100'}`}
											>
												<span class="relative" class:text-red-500={nosocket}>{page.display}</span>
											</a>
										</li>
									{/if}
								{/each}
							</ul>

							<div
								class="border-t px-6 py-8 md:px-12 md:py-16 lg:border-l lg:border-t-0 lg:py-0 lg:pl-6 lg:pr-0"
							>
								{#if isAdmin}
									<div class="flex items-center gap-3">
										<a
											href="/ucp/admin"
											class:text-red-500={nosocket}
											class={`${faction === Factions.Tow ? 'from-tow via-blue-600 to-emerald-400' : faction === Factions.Taxi ? 'from-taxi via-amber-600 to-red-500' : 'from-apms via-[#ad8447] to-[#d48613]'} bg-linear-to-r block rounded-full bg-[size:200%] bg-[position:0] px-6 py-3 text-center font-bold drop-shadow-lg transition-all duration-500 hover:bg-[position:100%]`}
										>
											Adminisztráció
											{#if christmas}
												<span class="icon-[fluent-emoji--sled] absolute bottom-11 right-8 h-6 w-6"
												></span>
											{/if}
										</a>
									</div>
								{/if}
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
	{#if faction === Factions.Taxi || faction === Factions.Tow}
		<h2 class="bg-linear-to-r z-20 from-rose-600 to-amber-600 py-1 text-center text-xl text-white">
			Nem vagy biztos valamiben? Nézd meg a <a href="/ucp/segedlet" class="text-taxi z-20 font-bold"
				>segédletet</a
			>!
		</h2>
	{/if}
</header>
