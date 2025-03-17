import { apiUrl } from '$lib/api';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { Logs } from '$lib/types';

export const load = (async ({ cookies }) => {
	let req = await fetch(`${apiUrl}/ucp/admin/faction/logs/get_all`, {
		headers: {
			cookie: cookies.get('auth_token') as string,
			faction: cookies.get('selected_faction') as string
		}
	});
	if (req.ok) {
		let list: Logs[] = await req.json();
		return {
			logs: list
		};
	} else {
		throw redirect(302, '/ucp/fm/logs');
	}
}) satisfies PageServerLoad;
