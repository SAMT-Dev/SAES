export enum Permissions {
	SaesLogin = 'saes.login',
	SaesTest = 'saes.test',
	SaesMaintenance = 'saes.maintenance',
	SaesTaxiUcp = 'saes.taxi.ucp',
	SaesApmsUcp = 'saes.apms.ucp',
	SaesTowUcp = 'saes.tow.ucp',
	SaesTaxiAdmin = 'saes.taxi.admin',
	SaesApmsAdmin = 'saes.apms.admin',
	SaesTowAdmin = 'saes.tow.admin',
	SaesTaxiAdminShift = 'saes.taxi.admin.shift',
	SaesApmsAdminShift = 'saes.apms.admin.shift',
	SaesTowAdminShift = 'saes.tow.admin.shift',
	SaesTaxiAdminFleet = 'saes.taxi.admin.fleet',
	SaesApmsAdminFleet = 'saes.apms.admin.fleet',
	SaesTowAdminFleet = 'saes.tow.admin.fleet',
	SaesTaxiAdminFaction = 'saes.taxi.admin.faction',
	SaesApmsAdminFaction = 'saes.apms.admin.faction',
	SaesTowAdminFaction = 'saes.tow.admin.faction'
}

export enum Factions {
	Taxi = 'SCKK',
	Apms = 'APMS',
	Tow = 'TOW'
}

export function get_faction_by_id(faction: number) {
	if (faction === 1) {
		return Factions.Taxi;
	}
	if (faction === 2) {
		return Factions.Apms;
	}
	if (faction === 3) {
		return Factions.Tow;
	}
}
