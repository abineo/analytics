async function POST({ request, params }) {
	const data = await request.json();
	console.info(params.event + ':', params.project, data);
	return new Response();
}

export { POST };
