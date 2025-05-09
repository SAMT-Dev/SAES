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

export function getFactionPerm(perm: Permissions, fact: string) {
	return perm.toString().replaceAll("fc", fact);
}
