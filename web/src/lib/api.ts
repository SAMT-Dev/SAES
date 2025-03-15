import type { Factions } from './permissions';

export const apiUrl =
	process.env.NODE_ENV === 'development'
		? 'http://localhost:3000'
		: (process.env.api_url as string);
export const apiUrlPublic =
	process.env.NODE_ENV === 'development'
		? 'http://localhost:3000'
		: (process.env.api_url_pub as string);
export const cdnUrl =
	process.env.NODE_ENV === 'development'
		? 'http://localhost:3100'
		: (process.env.cdn_url_pub as string);

let date = new Date();

// * Hó engedélyezése Mikulás-naptól vízkeresztig
export const snow =
	(date.getMonth() === 11 && date.getDate() >= 6) || (date.getMonth() < 1 && date.getDate() < 20)
		? true
		: false;

export const christmas =
	(date.getMonth() === 11 && date.getDate() >= 6) || (date.getMonth() == 0 && date.getDate() <= 7)
		? true
		: false;

export function allowPerms(
	data: {
		layout?: {
			admin: boolean;
			perms: string[];
		};
	},
	perms: string[]
) {
	if (data.layout?.admin) return true;
	for (const perm of perms) {
		if (data.layout?.perms.includes(perm)) return true;
	}
	return false;
}

export function countPerms(
	data: {
		layout?: {
			admin: boolean;
			perms: string[];
		};
	},
	perms: string[]
) {
	if (data.layout?.admin) return 99;
	let i = 0;
	for (const perm of perms) {
		if (data.layout?.perms.includes(perm)) i++;
	}
	return i;
}

export function allowFacts(
	data: {
		faction?: string;
	},
	factions: Factions[]
) {
	if (factions.includes(data.faction as Factions)) return true;
	return false;
}
