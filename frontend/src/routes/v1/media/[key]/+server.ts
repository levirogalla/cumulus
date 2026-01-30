import { PUBLIC_API_URL } from '$env/static/public'
// import { error } from '@sveltejs/kit';

export const GET = async ({ params, fetch, request}) => {
  const { key } = params;

  const req = new Request(`${PUBLIC_API_URL}/media/${key}`, request)
  const res = await fetch(req);

  console.log(res);

  return res 
}