import { PUBLIC_API_URL } from "$env/static/public";
import type { BucketName, RawServerFileObjectMetadata, ServerFileObjectMetadata } from "$lib/models";
import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ url }) => {
	const bucket: BucketName = url.searchParams.get("bucket") == "media" ? "media" : "files";

	const res = await fetch(`${PUBLIC_API_URL}/files?bucket=${bucket}`);
	if (!res.ok) {
		console.error(res)
	}
	const rawFiles: RawServerFileObjectMetadata[] = await res.json();

	const files: ServerFileObjectMetadata[] = rawFiles.map((f) => ({
		...f,
		last_modified: f.last_modified ?new Date(
			f.last_modified.secs_since_epoch * 1000 + Math.floor(f.last_modified.nanos_since_epoch / 1e6)
		) : null,
	}));

	return { files, bucket: bucket };
};

export const actions: Actions = {
  upload: async ({ request, fetch, url }) => {
    const req = new Request(`${PUBLIC_API_URL}/upload-file?bucket=${url.searchParams.get("bucket") ?? "files"}`, request);
    const res = await fetch(req);

    if (!res.ok) {
      return { success: false };
    }

    return { success: true };
  },

	delete: async ({ request, fetch, url }) => {
		const data = await request.formData();
		const key = data.get("key")?.toString();
		if (!key) {
			return { success: false }
		}
		const req = new Request(`${PUBLIC_API_URL}/delete-file/${encodeURI(key)}?bucket=${url.searchParams.get("bucket") ?? "files"}`, { method: 'DELETE' });
    const res = await fetch(req);

    if (!res.ok) {
      return { success: false };
    }

    return { success: true };
	}	
};