async function POST({ request, params }) {
	const data = await request.json();
	const view = crypto.randomUUID();
	console.info("visit:", params.project, view, data);
	return new Response(view);
}

export { POST };
