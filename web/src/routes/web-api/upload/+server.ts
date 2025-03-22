import { apiUrl } from '$lib/api';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request, cookies }) => {
	const body = await request.formData();
	let count = 0;
	body.forEach(() => {
		count++;
	});
	const dcauth = cookies.get('auth_token') as string;
	const fact = cookies.get('selected_faction') as string;
	const tipus = request.headers.get('tip');
	const dates = request.headers.get('dates');
	const ate = JSON.parse(dates!);
	if (tipus != 'leintés' || count % 2 === 0) {
		const mama = await fetch(`${apiUrl}/ucp/items/post?tipus=${tipus}&dates=${ate.toString()}`, {
			method: 'post',
			headers: {
				cookie: dcauth,
				faction: fact
			},
			body
		});
		if (mama.status === 406) {
			return new Response(JSON.stringify({ error: 'toobig' }));
		}
		const bodi = await mama.json();
		return new Response(JSON.stringify(bodi));
	} else {
		return new Response(JSON.stringify({ error: 'leintestipik' }));
	}
};
