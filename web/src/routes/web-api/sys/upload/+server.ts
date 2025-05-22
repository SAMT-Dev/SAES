import { apiUrl } from "$lib/api";
import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.formData();
	let count = 0;
	body.forEach(() => {
		count++;
	});
	console.log(count);
	const dcauth = cookies.get("auth_token") as string;
	const fact = request.headers.get("factionid");
	const dates = request.headers.get("dates");
	const ate = JSON.parse(dates!);
	if (count === 1) {
		const mama = await fetch(
			`${apiUrl}/sys/factions/change_image?faction=${fact}&dates=${ate.toString()}`,
			{
				method: "post",
				headers: {
					cookie: dcauth,
				},
				body,
			},
		);
		if (mama.status === 406) {
			return new Response(JSON.stringify({ error: "toobig" }));
		}
		console.log(mama.status);
		const bodi = await mama.json();
		return new Response(JSON.stringify(bodi));
	} else {
		return new Response(JSON.stringify({ error: "leintestipik" }));
	}
};
