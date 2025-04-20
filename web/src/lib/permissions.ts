export enum Permissions {
	SaesLogin = "saes.login",
	SaesTest = "saes.test",
	SaesMaintenance = "saes.maintenance",
	SaesFactUcp = "saes.fc.ucp",
	SaesFactAdmin = "saes.fc.admin",
	SaesFactAdminShift = "saes.fc.admin.shift",
	SaesFactAdminFleet = "saes.fc.admin.fleet",
	SaesFactAdminFaction = "saes.fc.admin.faction",
}

function generateFactiontag(perm: Permissions, fact: Factions) {
	let faction: string = "";
	if (fact === Factions.Taxi) faction = "taxi";
	if (fact === Factions.Apms) faction = "apms";
	if (fact === Factions.Tow) faction = "tow";
	if (fact === Factions.Uni) faction = "uni";
	return perm.toString().replaceAll("fc", faction);
}

type FactionPermissions = {
	[key in Factions]: {
		[key in keyof typeof Permissions]: string;
	};
};

export enum Factions {
	Taxi = "SCKK",
	Apms = "APMS",
	Tow = "TOW",
	Uni = "UNI",
}

export const factPermissions: FactionPermissions = {
	[Factions.Taxi]: {
		SaesLogin: generateFactiontag(Permissions.SaesLogin, Factions.Taxi),
		SaesTest: generateFactiontag(Permissions.SaesTest, Factions.Taxi),
		SaesMaintenance: generateFactiontag(
			Permissions.SaesMaintenance,
			Factions.Taxi,
		),
		SaesFactUcp: generateFactiontag(Permissions.SaesFactUcp, Factions.Taxi),
		SaesFactAdmin: generateFactiontag(
			Permissions.SaesFactAdmin,
			Factions.Taxi,
		),
		SaesFactAdminShift: generateFactiontag(
			Permissions.SaesFactAdminShift,
			Factions.Taxi,
		),
		SaesFactAdminFleet: generateFactiontag(
			Permissions.SaesFactAdminFleet,
			Factions.Taxi,
		),
		SaesFactAdminFaction: generateFactiontag(
			Permissions.SaesFactAdminFaction,
			Factions.Taxi,
		),
	},
	[Factions.Apms]: {
		SaesLogin: generateFactiontag(Permissions.SaesLogin, Factions.Apms),
		SaesTest: generateFactiontag(Permissions.SaesTest, Factions.Apms),
		SaesMaintenance: generateFactiontag(
			Permissions.SaesMaintenance,
			Factions.Apms,
		),
		SaesFactUcp: generateFactiontag(Permissions.SaesFactUcp, Factions.Apms),
		SaesFactAdmin: generateFactiontag(
			Permissions.SaesFactAdmin,
			Factions.Apms,
		),
		SaesFactAdminShift: generateFactiontag(
			Permissions.SaesFactAdminShift,
			Factions.Apms,
		),
		SaesFactAdminFleet: generateFactiontag(
			Permissions.SaesFactAdminFleet,
			Factions.Apms,
		),
		SaesFactAdminFaction: generateFactiontag(
			Permissions.SaesFactAdminFaction,
			Factions.Apms,
		),
	},
	[Factions.Tow]: {
		SaesLogin: generateFactiontag(Permissions.SaesLogin, Factions.Tow),
		SaesTest: generateFactiontag(Permissions.SaesTest, Factions.Tow),
		SaesMaintenance: generateFactiontag(
			Permissions.SaesMaintenance,
			Factions.Tow,
		),
		SaesFactUcp: generateFactiontag(Permissions.SaesFactUcp, Factions.Tow),
		SaesFactAdmin: generateFactiontag(
			Permissions.SaesFactAdmin,
			Factions.Tow,
		),
		SaesFactAdminShift: generateFactiontag(
			Permissions.SaesFactAdminShift,
			Factions.Tow,
		),
		SaesFactAdminFleet: generateFactiontag(
			Permissions.SaesFactAdminFleet,
			Factions.Tow,
		),
		SaesFactAdminFaction: generateFactiontag(
			Permissions.SaesFactAdminFaction,
			Factions.Tow,
		),
	},
	[Factions.Uni]: {
		SaesLogin: generateFactiontag(Permissions.SaesLogin, Factions.Uni),
		SaesTest: generateFactiontag(Permissions.SaesTest, Factions.Uni),
		SaesMaintenance: generateFactiontag(
			Permissions.SaesMaintenance,
			Factions.Uni,
		),
		SaesFactUcp: generateFactiontag(Permissions.SaesFactUcp, Factions.Uni),
		SaesFactAdmin: generateFactiontag(
			Permissions.SaesFactAdmin,
			Factions.Uni,
		),
		SaesFactAdminShift: generateFactiontag(
			Permissions.SaesFactAdminShift,
			Factions.Uni,
		),
		SaesFactAdminFleet: generateFactiontag(
			Permissions.SaesFactAdminFleet,
			Factions.Uni,
		),
		SaesFactAdminFaction: generateFactiontag(
			Permissions.SaesFactAdminFaction,
			Factions.Uni,
		),
	},
};

export function getAllFactionPermissions(perm: Permissions): string[] {
	return Object.values(Factions).map((faction) =>
		generateFactiontag(perm, faction)
	);
}

export function get_faction_by_id(faction: number) {
	if (faction === 1) {
		return Factions.Taxi;
	}
	if (faction === 2) {
		return Factions.Tow;
	}
	if (faction === 3) {
		return Factions.Apms;
	}
	if (faction === 5) {
		return Factions.Uni;
	}
}
