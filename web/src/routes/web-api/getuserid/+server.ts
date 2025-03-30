import { apiUrl } from "$lib/api";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ request, cookies }) => {
    let dcauth = cookies.get("auth_token") as string;
    const mama = await fetch(`${apiUrl}/ucp/getuserid`, {
        headers: {
            cookie: dcauth,
            faction: cookies.get("selected_faction")!,
            username: request.headers.get("username")!,
        },
    });
    if (mama.ok) {
        return new Response(JSON.stringify(await mama.json()));
    }
    return new Response(null, { status: 400 });
};
