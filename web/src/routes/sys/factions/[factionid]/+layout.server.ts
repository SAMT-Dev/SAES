import { apiUrl, cdnUrl } from "$lib/api";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ cookies, params }) => {
    let auth = cookies.get("auth_token")!;
    try {
        let factget = await fetch(`${apiUrl}/sys/getfactions`, {
            headers: { cookie: auth },
        });
        let posget = await fetch(`${apiUrl}/sys/getpositions`, {
            headers: { cookie: auth },
        });
        if (factget.ok) {
            let data: Record<string, {
                name: string;
                shortname: string;
                archived: boolean;
                managed: boolean;
                icon: string | undefined;
                comment: string | undefined;
                sheetkey: string | undefined;
                dcrole: string | undefined;
                defpos: number | undefined;
                permgroup: number | undefined;
            }> = await factget.json();
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
