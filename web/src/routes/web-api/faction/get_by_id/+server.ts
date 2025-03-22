import { apiUrl } from '$lib/api';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ request, cookies }) => {
	const dcauth = cookies.get('auth_token') as string;
	const mama = await fetch(
		`${apiUrl}/ucp/admin/faction/get_by_id?type=${request.headers.get('item_type')}&id=${request.headers.get(
			'item_id'
		)}`,
		{
			headers: {
				cookie: dcauth,
				faction: cookies.get('selected_faction')!
			}
		}
	);
	if (mama.ok) {
		return new Response(JSON.stringify(await mama.json()));
	}

	return new Response(null, { status: 400 });
};
