import { apiUrl } from '$lib/api';
import type { RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ request, cookies, params }) => {
	const dcauth = cookies.get('auth_token') as string;
	const body = await request.json();
	const post = await fetch(`${apiUrl}/ucp/sys/config/faction-post?faction=${params.faction}`, {
		method: 'POST',
		headers: {
			cookie: dcauth,
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(body)
	});
	const text = await post.text();
	return new Response(text, { status: post.status });
};
