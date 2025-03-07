import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
	cookies.set('maintenance', 'true', {
		maxAge: 600,
		httpOnly: true,
		secure: true,
		path: '/'
	});
	throw redirect(307, '/ucp?cookie_refresh=true');
}) satisfies PageServerLoad;
