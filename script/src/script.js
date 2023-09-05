let doc = document;
let html = doc.documentElement;
let loc = location;
let pushState = history.pushState;

let project = config('project');
let domain = config('domain') || 'abineo-analytics.com';
let startTime;
let scrollDistance;
let referrer = doc.referrer;

function startSession() {
	startTime = Date.now();
	scrollDistance = getScrollDistance();
}

// UTILITIES -------------------------------------------------------------------

function config(name) {
	return doc.querySelector('meta[name=abineo:' + name + ']')?.content;
}

function getScrollDistance() {
	return html.scrollTop / (html.scrollHeight - innerHeight);
}
