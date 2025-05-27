import { apiUrl, cdnUrl } from "$lib/api";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ cookies, params }) => {
    let auth = cookies.get("auth_token")!;
    try {
        let get = await fetch(`${apiUrl}/sys/getfactions`, {
            headers: { cookie: auth },
        });
        if (get.ok) {
            let data: Record<string, {
                name: string;
                shortname: string;
                archived: boolean;
                managed: boolean;
                icon: string | undefined;
            }> = await get.json();
            return {
                factinfo: data[params.factionid],
                id: params.factionid,
                cdn: cdnUrl,
            };
        }
    } catch (e) {
        return {
            error: e,
        };
    }
}) satisfies LayoutServerLoad;
