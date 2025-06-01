import type { LayoutServerLoad } from "./$types";
import { allowPerms, apiUrl, apiUrlPublic, cdnUrl, countPerms } from "$lib/api";
import { isRedirect, redirect } from "@sveltejs/kit";
import { getFactionPerm, Permissions } from "$lib/permissions";

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
				secure: true,
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
			if (cookies.get("dc-auth")) {
				cookies.delete("auth_token", { path: "/" });
				return {
					refresh: true,
				};
			}
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
				driver: {
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
				};
				info?: {
					display: string;
					icon_id: string;
					perm_name: string;
					primary: string;
					secondary: string;
					tertiary: string;
				};
			} = await aha.json();
			if (jeson.driver.name) {
				if (url.searchParams.get("select_faction")) {
					let sfact = url.searchParams.get(
						"select_faction",
					) as string;
					cookies.set("selected_faction", sfact, {
						path: "/",
						maxAge: 360 * 24 * 60,
						secure: true,
						sameSite: true,
						httpOnly: true,
					});
					throw redirect(303, url.pathname);
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
				let selfactions = await fetch(`${apiUrl}/ucp/getfactions`, {
					headers: {
						cookie: cookies.get("auth_token")!,
					},
				});
				if (!selfactions.ok) {
					return {
						error: "Frakcióid lekérése sikertelen.",
					};
				}
				let factions: Record<
					string,
					{
						icon_id: number;
						name: string;
						perm_name: string;
						primary: string;
						secondary: string;
						tertiary: string;
					}
				> = await selfactions.json();
				if (!cookies.get("selected_faction")) {
					if (
						Object.keys(factions).length > 1
					) {
						return {
							layout: jeson.driver,
							auth: cookies.get("auth_token")!,
							api: apiUrlPublic,
							nofact: factions,
						};
					}
					throw redirect(
						303,
						`?select_faction=${Object.keys(factions)[0]}`,
					);
				}
				let selfact = cookies.get("selected_faction")!;
				if (!factions[selfact]) {
					throw redirect(303, "?clear_faction=true");
				}
				if (
					!allowPerms({ layout: jeson.driver }, [
						getFactionPerm(
							Permissions.SaesFactUcp,
							factions[selfact].perm_name,
						),
					])
				) {
					throw redirect(303, "?clear_faction=true");
				}
				if (jeson.info) {
					jeson.info.icon_id =
						`${cdnUrl}/get?id=${jeson.info.icon_id}`;
				}
				return {
					layout: jeson.driver,
					info: jeson.info,
					api: apiUrlPublic,
					cdn: cdnUrl,
					faction: cookies.get("selected_faction"),
					country: process.env.NODE_ENV === "development"
						? "HU"
						: (request.headers.get("cf-ipcountry") as string),
					auth: cookies.get("auth_token")!,
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
