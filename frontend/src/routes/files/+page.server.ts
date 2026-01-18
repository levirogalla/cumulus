import { PUBLIC_API_URL } from "$env/static/public";
import type { RawServerFileObjectMetadata, ServerFileObjectMetadata } from "$lib/models";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
	const res = await fetch(`${PUBLIC_API_URL}/files`);
	const rawFiles: RawServerFileObjectMetadata[] = await res.json();

	const files: ServerFileObjectMetadata[] = rawFiles.map((f) => ({
		...f,
		last_modified: f.last_modified ?new Date(
			f.last_modified.secs_since_epoch * 1000 + Math.floor(f.last_modified.nanos_since_epoch / 1e6)
		) : null,
	}));

	return { files };
};

export const actions: Actions = {
  upload: async ({ request, fetch }) => {
    const req = new Request(`${PUBLIC_API_URL}/upload-file`, request);
    const res = await fetch(req);

    if (!res.ok) {
      return { success: false };
    }

    return { success: true };
  },

	delete: async ({ request, fetch }) => {
		const data = await request.formData();
		const key = data.get("key");
		const req = new Request(`${PUBLIC_API_URL}/delete-file/${key}`, { method: 'DELETE' });
    const res = await fetch(req);

    if (!res.ok) {
      return { success: false };
    }

    return { success: true };
	}	
};