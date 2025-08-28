<script lang="ts">
	import { christmas } from '$lib/api';
	import { pages } from './public';
	import { cn } from '$lib/utils.js';
	import * as NavigationMenu from '$lib/components/ui/navigation-menu/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { navigationMenuTriggerStyle } from '@/components/ui/navigation-menu/navigation-menu-trigger.svelte';
	import type { HTMLAttributes } from 'svelte/elements';
	import { marked } from 'marked';
	import { goto } from '$app/navigation';

	interface Props {
		tip: any;
		isAdmin?: boolean;
		announcement?: string;
		maintenance?: string | true;
		faction: string;
		icon: string;
		data: {
			layout?: {
				admin: boolean;
				perms: string[];
			};
			info?: {
				icon_id?: string;
				primary?: string;
				secondary?: string;
				tertiary?: string;
			};
		};
		nosocket: string | boolean;
	}

	let {
		tip,
		isAdmin = false,
		faction = 'SAMT',
		data,
		nosocket,
		icon,
		announcement,
		maintenance
	}: Props = $props();

	let pagesz = pages(faction);

	type ListItemProps = HTMLAttributes<HTMLAnchorElement> & {
		title: string;
		href: string;
	};
</script>

{#snippet ListItem({ title, href, class: className, ...restProps }: ListItemProps)}
	<li>
		<NavigationMenu.Link>
			{#snippet child()}
				<a
					{href}
					class={cn(
						'hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors',
						className
					)}
					{...restProps}
				>
					<div class="text-sm font-medium leading-none">{title}</div>
				</a>
			{/snippet}
		</NavigationMenu.Link>
	</li>
{/snippet}

<nav
	style={`--color-primary: ${data.info?.primary}; --color-secondary: ${data.info?.secondary}; --color-tertiary: ${data.info?.tertiary};`}
	class="bg-background/20 fixed left-1/2 top-0 z-50 mt-2 flex h-20 w-11/12 max-w-[80dvw] -translate-x-1/2 flex-col items-center justify-between rounded-full px-5 backdrop-blur-lg"
>
	{#if maintenance}
		<div
			class="text-md absolute top-0 flex h-full w-full items-start justify-center gap-1 overflow-hidden rounded-full text-center text-black dark:text-white"
		>
			<div class="flex h-4 w-full items-center justify-center gap-1 bg-red-400/40 backdrop-blur-lg">
				{#if typeof maintenance == 'string'}
					<h2>Karbantartás:</h2>
					<h2>
						{@html marked(maintenance.toString())}
					</h2>
				{:else}
					Karbantartás
				{/if}
			</div>
		</div>
	{/if}
	{#if announcement}
		<div
			class="text-md absolute bottom-0 flex h-full w-full items-end justify-center overflow-hidden rounded-full text-center text-black dark:text-white"
		>
			<h2
				class={`flex h-4 w-full items-center justify-center backdrop-blur-lg ${nosocket === false ? 'bg-blue-400/40' : 'bg-red-500/80'}`}
			>
				{#if nosocket}
					{nosocket}
				{:else}
					{@html marked(announcement.toString())}
				{/if}
			</h2>
		</div>
	{/if}
	<div class="z-30 m-auto flex w-full items-center justify-between gap-2">
		<a
			class="group relative z-20 flex items-center gap-3"
			data-sveltekit-reload={true}
			href="?clear_faction=true"
		>
			<div
				class="pointer-events-none ml-5 rounded-full border-2 border-solid drop-shadow-xl duration-200 group-hover:border-[var(--color-primary)]"
			>
				<img
					src={icon}
					class="border-1 pointer-events-none rounded-full border-solid border-black"
					width="40"
					height="40"
					alt="Logó"
				/>
			</div>
			<h1
				class="text-3xl font-bold drop-shadow-xl transition-colors duration-200 group-hover:text-[var(--color-primary)]"
			>
				{tip}
			</h1>
			{#if christmas}
				<img src="/santa.svg" class="absolute bottom-2 left-3.5 w-14 -rotate-[24deg]" alt="" />
			{/if}
		</a>
		<div class="justify-center-center flex items-center gap-4">
			<NavigationMenu.Root>
				<NavigationMenu.List>
					{#each pagesz as page}
						{#if page.faction.includes(faction)}
							<NavigationMenu.Item>
								{#if !page.children}
									<NavigationMenu.Link>
										{#snippet child()}
											<a
												href={page.url}
												class={navigationMenuTriggerStyle('hover:text-[var(--color-primary)]')}
												>{page.display}</a
											>
										{/snippet}
									</NavigationMenu.Link>
								{:else}
									<NavigationMenu.Trigger>{page.display}</NavigationMenu.Trigger>
									<NavigationMenu.Content>
										<ul class="grid gap-2 p-2 md:w-[400px] lg:w-[500px] lg:grid-cols-[.75fr_1fr]">
											{#each page.children as pc}
												{#if pc.faction.includes(faction)}
													{@render ListItem({ title: pc.display, href: pc.url })}
												{/if}
											{/each}
										</ul>
									</NavigationMenu.Content>
								{/if}
							</NavigationMenu.Item>
						{/if}
					{/each}
				</NavigationMenu.List>
			</NavigationMenu.Root>
			{#if isAdmin}
				<Button
					onclick={async () => await goto('/ucp/admin')}
					class="text-foreground bg-[var(--color-primary)] font-bold hover:bg-[var(--color-secondary)]"
					>Adminisztráció</Button
				>
			{/if}
		</div>
	</div>
</nav>
