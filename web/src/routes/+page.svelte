<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';
</script>

<div class="flex h-screen text-center">
	<a href="/ucp" class="m-auto">
		<Card.Root>
			<Card.Header>
				<Card.Title class="text-3xl font-bold">SAMT UCP</Card.Title>
				<Card.Description>Bejelentkezéshez kattints ide</Card.Description>
			</Card.Header>
			<Card.Content>
				<img
					src="/favicon.png"
					class="group-hover:border-taxi m-auto w-52 rounded-full border-4 border-solid border-white transition-colors duration-300"
					alt="SAMT Logo"
				/>
			</Card.Content>
		</Card.Root>
	</a>
</div>

<!--
			RÉGI GOMB
			<a
						href="user"
						class="text-black dark:text-white bg-green-500 text-2xl font-bold px-3 rounded-xl mt-2 hover:bg-green-800 transition-all duration-200"
						>Belépés Discordal</a>
			-->
