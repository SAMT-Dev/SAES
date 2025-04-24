import type { LayoutServerLoad } from "./$types";
import { allowPerms, apiUrl, apiUrlPublic, cdnUrl, countPerms } from "$lib/api";
import { isRedirect, redirect } from "@sveltejs/kit";
import {
	Factions,
	factPermissions,
	getAllFactionPermissions,
	Permissions,
} from "$lib/permissions";

export type AccessType = "Write" | "Read" | "None";

export const load = (async ({ cookies, request, url }) => {
	if (!cookies.get("auth_token")) {
		if (!cookies.get("dc-auth")) {
			return {
				noauth: true,
				api: apiUrlPublic,
			};
		}
		let jwt = await fetch(`${apiUrl}/auth/jwt`, {
			headers: {
				cookie: cookies.get("dc-auth")!,
			},
		});
		if (jwt.ok) {
			let realjwt: { jwt: string; exp: number } = await jwt.json();
			cookies.set("auth_token", realjwt.jwt, {
				path: "/",
				expires: new Date(realjwt.exp * 1000),
				secure: false,
				httpOnly: true,
			});
			return {
				refresh: true,
			};
		}
	}
	try {
		const aha = await fetch(`${apiUrl}/ucp`, {
			headers: {
				cookie: cookies.get("auth_token")!,
				faction: cookies.get("selected_faction")
					? cookies.get("selected_faction")!
					: "",
			},
		});
		if (aha.status === 404 || aha.status === 406) {
			return {
				noauth: true,
				api: apiUrlPublic,
			};
		}
		if (aha.status === 403) {
			return {
				noaccess: await aha.text(),
			};
		}
		if (aha.status === 402) {
			return {
				error: await aha.text(),
			};
		}
		if (aha.ok) {
			const jeson: {
				discordid: string;
				driverid: number;
				name: string;
				admin: boolean;
				perms: string[];
				access: {
					supplements: AccessType;
					hails: AccessType;
					bills: AccessType;
				};
				site_access: {
					ucp: boolean;
					admin: boolean;
					shift: boolean;
					fleet: boolean;
					faction: boolean;
				};
				faction?: string;
				factions?: {
					factionid: number;
					factionname: string;
					factionshortname: string;
					positionid: number;
					positionname: string;
					shiftid: number;
					shiftname: string;
				};
			} = await aha.json();
			if (jeson.name) {
				if (url.searchParams.get("select_faction")) {
					let sfact = url.searchParams.get(
						"select_faction",
					) as string;
					if (Object.values(Factions).includes(sfact as Factions)) {
						if (
							sfact === Factions.Taxi &&
							allowPerms({ layout: jeson }, [
								factPermissions[Factions.Taxi].SaesFactUcp,
							])
						) {
							cookies.set("selected_faction", Factions.Taxi, {
								path: "/",
								maxAge: 360 * 24 * 60,
								secure: false,
								sameSite: true,
								httpOnly: true,
							});
							throw redirect(303, url.pathname);
						}
						if (
							sfact === Factions.Apms &&
							allowPerms({ layout: jeson }, [
								factPermissions.APMS.SaesFactUcp,
							])
						) {
							cookies.set("selected_faction", Factions.Apms, {
								path: "/",
								maxAge: 360 * 24 * 60,
								secure: false,
								sameSite: true,
								httpOnly: true,
							});
							throw redirect(303, url.pathname);
						}
						if (
							sfact === Factions.Tow &&
							allowPerms({ layout: jeson }, [
								factPermissions[Factions.Tow].SaesFactUcp,
							])
						) {
							cookies.set("selected_faction", Factions.Tow, {
								path: "/",
								maxAge: 360 * 24 * 60,
								secure: false,
								sameSite: true,
								httpOnly: true,
							});
							throw redirect(303, url.pathname);
						}
						if (
							sfact === Factions.Uni &&
							allowPerms({ layout: jeson }, [
								factPermissions[Factions.Uni].SaesFactUcp,
							])
						) {
							cookies.set("selected_faction", Factions.Uni, {
								path: "/",
								maxAge: 360 * 24 * 60,
								secure: false,
								sameSite: true,
								httpOnly: true,
							});
							throw redirect(303, url.pathname);
						}
					}
				}
				if (url.searchParams.get("clear_faction")) {
					cookies.delete("selected_faction", { path: "/" });
					throw redirect(303, url.pathname);
				}
				if (url.searchParams.get("cookie_refresh")) {
					return {
						refresh: true,
					};
				}
				if (!cookies.get("selected_faction")) {
					if (
						countPerms(
							{ layout: jeson },
							getAllFactionPermissions(Permissions.SaesFactUcp),
						) >= 2
					) {
						return {
							layout: jeson,
							auth: cookies.get("auth_token")!,
							api: apiUrlPublic,
							nofact: true,
						};
					}
					if (
						allowPerms({ layout: jeson }, [
							factPermissions[Factions.Taxi].SaesFactUcp,
						])
					) {
						throw redirect(303, "?select_faction=SCKK");
					}
					if (
						allowPerms({ layout: jeson }, [
							factPermissions.APMS.SaesFactUcp,
						])
					) {
						throw redirect(303, "?select_faction=APMS");
					}
					if (
						allowPerms({ layout: jeson }, [
							factPermissions[Factions.Tow].SaesFactUcp,
						])
					) {
						throw redirect(303, "?select_faction=TOW");
					}
					if (
						allowPerms({ layout: jeson }, [
							factPermissions[Factions.Uni].SaesFactUcp,
						])
					) {
						throw redirect(303, "?select_faction=UNI");
					}
				}
				switch (cookies.get("selected_faction")) {
					case Factions.Taxi:
						if (
							!allowPerms({ layout: jeson }, [
								factPermissions[Factions.Taxi].SaesFactUcp,
							])
						) {
							throw redirect(303, "?clear_faction=true");
						}
						break;
					case Factions.Apms:
						if (
							!allowPerms({ layout: jeson }, [
								factPermissions.APMS.SaesFactUcp,
							])
						) {
							throw redirect(303, "?clear_faction=true");
						}
						break;
					case Factions.Tow:
						if (
							!allowPerms({ layout: jeson }, [
								factPermissions[Factions.Tow].SaesFactUcp,
							])
						) {
							throw redirect(303, "?clear_faction=true");
						}
						break;
					case Factions.Uni:
						if (
							!allowPerms({ layout: jeson }, [
								factPermissions[Factions.Uni].SaesFactUcp,
							])
						) {
							throw redirect(303, "?clear_faction=true");
						}
						break;
				}
				return {
					layout: jeson,
					api: apiUrlPublic,
					cdn: cdnUrl,
					faction: cookies.get("selected_faction"),
					country: process.env.NODE_ENV === "development"
						? "HU"
						: (request.headers.get("cf-ipcountry") as string),
					auth: cookies.get("auth_token")!,
					offset: process.env.SUMMER_TIMEZONE === "true"
						? -60 * 60 * 1000 * 2
						: -60 * 60 * 1000,
					agent: request.headers.get("user-agent") as string,
				};
			} else {
				return {
					noaccess: "SAMT API elérése sikertelen.",
				};
			}
		}
	} catch (err) {
		if (isRedirect(err)) {
			throw redirect(err.status, err.location);
		}
		return {
			error: "SAES API elérése sikertelen. 😭",
		};
	}
}) satisfies LayoutServerLoad;
