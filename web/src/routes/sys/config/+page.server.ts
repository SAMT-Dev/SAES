import { apiUrl, apiUrlPublic } from "$lib/api";
import { isRedirect, redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import type { Factions } from "$lib/permissions";

export type ShiftAccess = "SameShift" | "OtherManager" | "OtherShift";
export type FactionAccess = "None" | "Read" | "Write";

export interface FactionAccessConfig {
	supplements: FactionAccess;
	hails: FactionAccess;
	bills: FactionAccess;
}

export interface FactionSiteAccessConfig {
	ucp: boolean;
	admin: boolean;
	shift: boolean;
	fleet: boolean;
	faction: boolean;
}

export interface Config {
	global: {
		announcement?: string;
		maintenance?: string;
	};
	factions: {
		[key: string]: {
			shift_access: ShiftAccess;
			access: FactionAccessConfig;
			site_access: FactionSiteAccessConfig;
		};
	};
}

export const load = (async ({ cookies }) => {
	if (!cookies.get("auth_token")) {
		return {
			noauth: true,
			api: apiUrlPublic,
		};
	}
	try {
		const aha = await fetch(`${apiUrl}/sys/config/get`, {
			headers: {
				cookie: cookies.get("auth_token") as string,
			},
		});
		if (aha.status === 404) {
			return {
				noauth: true,
				api: apiUrlPublic,
			};
		}
		if (aha.status === 403) {
			throw redirect(302, "/ucp/admin");
		}
		if (aha.status === 402) {
			return {
				error: await aha.text(),
			};
		}
		if (aha.ok) {
			let config: Config = await aha.json();
			return {
				config,
			};
		}
	} catch (err) {
		if (isRedirect(err)) {
			throw redirect(err.status, err.location);
		}
		return {
			error: true,
		};
	}
}) satisfies PageServerLoad;
