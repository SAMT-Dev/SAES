import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load = (async ({ cookies, url }) => {
	cookies.delete("auth_token", {
		path: "/",
		domain: process.env.NODE_ENV === "development"
			? "localhost"
			: url.hostname,
	});
	throw redirect(302, "/");
}) satisfies PageServerLoad;
