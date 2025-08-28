// import { sheet } from '$lib/server/google';
// import { prisma } from '$lib/server/prisma';
import type { Actions, PageServerLoad } from "./$types";
import { apiUrl } from "$lib/api";
import { redirect } from "@sveltejs/kit";

export const load = (async ({ cookies }) => {
	if (!cookies.get("auth_token")) {
		return {
			noauth: true,
		};
	}
	try {
		const aha = await fetch(
			cookies.get("selected_faction") === "apms"
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
			if (cookies.get("selected_faction") === "apms") {
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

export const actions = {
	logout: async ({ cookies, request }) => {
		cookies.delete("auth_token", { path: "/" });
		cookies.delete("dc-auth", { path: "/" });
		cookies.delete("dc-refresh", { path: "/" });
		cookies.delete("selected_faction", { path: "/" });
		throw redirect(302, new URL(request.url).pathname);
	},
} satisfies Actions;
