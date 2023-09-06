async function POST({ request, params }) {
	const data = await request.json();
	console.info(params.project, params.event, data);
	return new Response();
}

export { POST };
