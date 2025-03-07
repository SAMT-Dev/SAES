import type { LayoutServerLoad } from './$types';
import { allowPerms, apiUrl, apiUrlPublic, cdnUrl, countPerms } from '$lib/api';
import { isRedirect, redirect } from '@sveltejs/kit';
import { Factions, Permissions } from '$lib/permissions';

export const load = (async ({ cookies, request, url }) => {
	if (!cookies.get('auth_token')) {
		return {
			noauth: true,
			api: apiUrlPublic
		};
	}
	try {
		const aha = await fetch(`${apiUrl}/ucp`, {
			headers: {
				cookie: cookies.get('auth_token')!
			}
		});
		if (aha.status === 404 || aha.status === 406) {
			return {
				noauth: true,
				api: apiUrlPublic
			};
		}
		if (aha.status === 403) {
			return {
				noaccess: await aha.text()
			};
		}
		if (aha.status === 402) {
			return {
				error: await aha.text()
			};
		}
		if (aha.ok) {
			const jeson: {
				discordid: string;
				driverid: number;
				name: string;
				admin: boolean;
				perms: string[];
				taxi?: {
					factionid: number;
					factionname: string;
					positionid: number;
					positionname: string;
					shiftid: number;
					shiftname: string;
				};
				tow?: {
					factionid: number;
					factionname: string;
					positionid: number;
					positionname: string;
					shiftid: number;
					shiftname: string;
				};
				apms?: {
					factionid: number;
					factionname: string;
					positionid: number;
					positionname: string;
					shiftid: number;
					shiftname: string;
				};
			} = await aha.json();
			if (jeson.name) {
				if (url.searchParams.get('select_faction')) {
					let sfact = url.searchParams.get('select_faction') as string;
					if (Object.values(Factions).includes(sfact as Factions)) {
						if (
							sfact === Factions.Taxi &&
							allowPerms({ layout: jeson }, [Permissions.SaesTaxiUcp])
						) {
							cookies.set('selected_faction', Factions.Taxi, {
								path: '/',
								maxAge: 360 * 24 * 30,
								secure: true,
								sameSite: true,
								httpOnly: true
							});
							throw redirect(303, url.pathname);
						}
						if (
							sfact === Factions.Apms &&
							allowPerms({ layout: jeson }, [Permissions.SaesApmsUcp])
						) {
							cookies.set('selected_faction', Factions.Apms, {
								path: '/',
								maxAge: 360 * 24 * 30,
								secure: true,
								sameSite: true,
								httpOnly: true
							});
							throw redirect(303, url.pathname);
						}
						if (sfact === Factions.Tow && allowPerms({ layout: jeson }, [Permissions.SaesTowUcp])) {
							cookies.set('selected_faction', Factions.Tow, {
								path: '/',
								maxAge: 360 * 24 * 30,
								secure: true,
								sameSite: true,
								httpOnly: true
							});
							throw redirect(303, url.pathname);
						}
					}
				}
				if (url.searchParams.get('clear_faction')) {
					cookies.delete('selected_faction', { path: '/' });
					throw redirect(303, url.pathname);
				}
				if (url.searchParams.get('cookie_refresh')) {
					return {
						refresh: true
					};
				}
				if (!cookies.get('selected_faction')) {
					if (
						countPerms({ layout: jeson }, [
							Permissions.SaesTaxiUcp,
							Permissions.SaesTowUcp,
							Permissions.SaesApmsUcp
						]) >= 2
					) {
						return {
							layout: jeson,
							auth: cookies.get('auth_token')!,
							api: apiUrlPublic,
							maintenance: cookies.get('maintenance')
								? jeson.admin
									? cookies.get('maintenance')
									: false
								: false,
							nofact: true
						};
					}
					if (allowPerms({ layout: jeson }, [Permissions.SaesTaxiUcp])) {
						throw redirect(303, '?select_faction=SCKK');
					}
					if (allowPerms({ layout: jeson }, [Permissions.SaesApmsUcp])) {
						throw redirect(303, '?select_faction=APMS');
					}
					if (allowPerms({ layout: jeson }, [Permissions.SaesTowUcp])) {
						throw redirect(303, '?select_faction=TOW');
					}
				}
				switch (cookies.get('selected_faction')) {
					case Factions.Taxi:
						if (!allowPerms({ layout: jeson }, [Permissions.SaesTaxiUcp])) {
							throw redirect(303, '?clear_faction=true');
						}
						break;
					case Factions.Apms:
						if (!allowPerms({ layout: jeson }, [Permissions.SaesApmsUcp])) {
							throw redirect(303, '?clear_faction=true');
						}
						break;
					case Factions.Tow:
						if (!allowPerms({ layout: jeson }, [Permissions.SaesTowUcp])) {
							throw redirect(303, '?clear_faction=true');
						}
						break;
				}
				return {
					layout: jeson,
					api: apiUrlPublic,
					cdn: cdnUrl,
					faction: cookies.get('selected_faction'),
					country:
						process.env.NODE_ENV === 'development'
							? 'HU'
							: (request.headers.get('cf-ipcountry') as string),
					auth: cookies.get('auth_token')!,
					offset: process.env.SUMMER_TIMEZONE === 'true' ? -60 * 60 * 1000 * 2 : -60 * 60 * 1000,
					agent: request.headers.get('user-agent') as string,
					maintenance: cookies.get('maintenance')
						? jeson.admin
							? cookies.get('maintenance')
							: false
						: false
				};
			} else {
				return {
					noaccess: 'SAMT API elérése sikertelen.'
				};
			}
		}
	} catch (err) {
		if (isRedirect(err)) {
			throw redirect(err.status, err.location);
		}
		return {
			error: 'SAES API elérése sikertelen. 😭'
		};
	}
}) satisfies LayoutServerLoad;
