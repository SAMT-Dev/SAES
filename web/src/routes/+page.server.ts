import type { PageServerLoad } from "./$types";

import { redirect } from "@sveltejs/kit";

export const load = (async ({ cookies }) => {
	// if (cookies.get('auth_token')) {
	// 	return {};
	// }
	// throw redirect(302, 'https://api.ampix.hu/sckk/auth');
	if (
		cookies.get("auth_token") || cookies.get("dc-auth") ||
		cookies.get("dc-refresh")
	) {
		throw redirect(302, "ucp");
	}
	return {
		noauth: true,
	};
}) satisfies PageServerLoad;
