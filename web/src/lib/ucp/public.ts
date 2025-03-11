import { Factions } from '$lib/permissions';

export function getRealText(text: string) {
	switch (text) {
		case 'pótlék_délelőtti':
			return 'Délelőtti pótlékok';
		case 'pótlék_éjszakai':
			return 'Éjszakai pótlékok';
		case 'számla':
			return 'Számlák';
		case 'leintés':
			return 'Leintések';
	}
}

export function getAlterText(text: string) {
	switch (text) {
		case 'pótlék_délelőtti':
			return 'potlek_de';
		case 'pótlék_éjszakai':
			return 'potlek_ej';
		case 'számla':
			return 'szamla';
		case 'leintés':
			return 'leintes';
	}
}

const Reeler = {
	leintesek: ['leintés', 'Leintéseid', 'Leintés', 'Leintések', 'leintésének'],
	potlekok: ['pótlék', 'Pótlékaid', 'Pótlék', 'Pótlékok', 'pótlékjának'],
	szamlak: ['számla', 'Számláid', 'Számla', 'Számlák', 'számlájának']
};

export const Reeler_keys = Object.keys(Reeler);
export const Reeler_vals = Object.values(Reeler);

export const pages = (fact: string) => {
	return [
		{
			url: '/ucp',
			display: 'Kezdőlap',
			faction: [Factions.Taxi, Factions.Apms, Factions.Tow]
		},
		{
			url: '/ucp/segedlet',
			display: 'Segédlet',
			faction: [Factions.Taxi, Factions.Tow]
		},
		{
			url: '/ucp/links',
			display: 'Hasznos linkek',
			faction: [Factions.Taxi, Factions.Tow]
		},
		{
			url: '/ucp/potlekok',
			display: 'Pótlékok',
			faction: [Factions.Taxi, Factions.Tow]
		},
		{
			url: '/ucp/leintesek',
			display: `Leintések${fact === Factions.Tow ? ' / Bejelentések' : ''}`,
			faction: [Factions.Taxi, Factions.Tow]
		},
		{
			url: '/ucp/szamlak',
			display: 'Szereltetési számlák',
			faction: [Factions.Taxi, Factions.Apms, Factions.Tow]
		}
	];
};
