import { apiUrl, cdnUrl } from "$lib/api";
import type { PageServerLoad } from "./$types";

export const load = (async ({ cookies }) => {
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
                data,
                cdn: cdnUrl,
            };
        }
    } catch (e) {
        return {
            error: e,
        };
    }
}) satisfies PageServerLoad;
