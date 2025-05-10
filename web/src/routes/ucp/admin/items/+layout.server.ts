import { apiUrl } from "$lib/api";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
    let fetcs = await fetch(`${apiUrl}/ucp/admin/getfactions`, {
        headers: {
            cookie: cookies.get("auth_token")!,
            faction: cookies.get("selected_faction")!,
        },
    });
    let data: Record<string, {
        name: string;
        id: number;
        perm_name: string;
    }> = await fetcs.json();
    return {
        factionlist: data,
    };
}) satisfies LayoutServerLoad;
