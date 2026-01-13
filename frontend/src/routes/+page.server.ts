import type { Actions, PageServerLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';
import type { RawServerFileObjectMetadata, ServerFileObjectMetadata } from '$lib/models';

export const load: PageServerLoad = async () => {
	const res = await fetch(`${PUBLIC_API_URL}/files`);
	const rawFiles: RawServerFileObjectMetadata[] = await res.json();
	console.log(rawFiles);

	const files: ServerFileObjectMetadata[] = rawFiles.map((f) => ({
		...f,
		last_modified: new Date(
			f.last_modified.secs_since_epoch * 1000 + Math.floor(f.last_modified.nanos_since_epoch / 1e6)
		)
	}));

	return { files };
};

export const actions: Actions = {
	upload: async ({ request }) => {
		const req = new Request(`${PUBLIC_API_URL}/upload`, request);

		const res = await fetch(req);

		return res.status;
	}
};
