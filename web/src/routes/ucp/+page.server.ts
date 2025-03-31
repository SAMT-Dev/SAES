// import { sheet } from '$lib/server/google';
// import { prisma } from '$lib/server/prisma';
import type { PageServerLoad } from "./$types";
import { apiUrl } from "$lib/api";
import { Factions } from "$lib/permissions";

export const load = (async ({ cookies }) => {
	if (!cookies.get("auth_token")) {
		return {
			noauth: true,
		};
	}
	try {
		const aha = await fetch(
			cookies.get("selected_faction") === Factions.Apms
				? `${apiUrl}/ucp/apms_calls`
				: `${apiUrl}/ucp/calls`,
			{
				headers: {
					cookie: cookies.get("auth_token") as string,
					faction: cookies.get("selected_faction") as string,
				},
			},
		);
		if (aha.status === 400) {
			return {
				error: "SAES API elérése sikertelen: " + (await aha.text()),
			};
		}
		if (aha.ok) {
			if (cookies.get("selected_faction") === Factions.Apms) {
				const text: {
					accepted?: number;
					uploaded: number;
				} = await aha.json();
				return {
					szamlak: text,
				};
			} else {
				const text: {
					app?: number;
					leintes: number;
					price: number;
					potlek: {
						de: number;
						du: number;
					};
				} = await aha.json();
				return {
					calls: text,
				};
			}
		}
	} catch (err) {
		return {
			error: "SAES API elérése sikertelen",
		};
	}
}) satisfies PageServerLoad;
