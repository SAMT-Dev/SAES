import { apiUrl } from "$lib/api";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ request, cookies }) => {
	let dcauth = cookies.get("auth_token") as string;
	const mama = await fetch(`${apiUrl}/ucp/getlegacynames`, {
		headers: {
			cookie: dcauth,
			faction: cookies.get("selected_faction")!,
			ids: request.headers.get("ids")!,
		},
	});
	if (mama.ok) {
		const json = await mama.json();
		return new Response(JSON.stringify(json));
	}
	return new Response(null, { status: 400 });
};
