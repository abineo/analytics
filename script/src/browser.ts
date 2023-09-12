import { Page, Visitor, config, getScrollDistance } from './lib';

let doc = document;
let html = doc.documentElement;
let addEventListener = window.addEventListener;
let loc = location;
let now = Date.now;
let pushState = history.pushState;

let sessionKey = 'abineo:session';
let protocol = config(doc, 'protocol') || 'https://';
let domain = config(doc, 'domain') || 'abineo-analytics.com';
let project = config(doc, 'project') || '';
let startTime: number;
let scrollDistance: number;
let referrer = doc.referrer;

let visitor = Visitor(navigator, screen);
let page = Page(loc, doc, referrer);
let session = sessionStorage.getItem(sessionKey);

if (!project) throw 'missing project id';

if (!session) {
	session = crypto.randomUUID();
	sessionStorage.setItem(sessionKey, session);
}

function post(endpoint: string, data: object) {
	return fetch(protocol + domain + '/api/v1/' + project + '/' + endpoint, {
		body: JSON.stringify(data),
		method: 'POST',
		headers: [['content-type', 'application/json']],
		keepalive: true,
	});
}

function onPageEnter() {
	startTime = now();
	scrollDistance = getScrollDistance(html, innerHeight);
	return post('visit', { session, visitor, page }).then((res) => res.text());
}

function onPageExit() {
	return post('exit', {
		session,
		visitor,
		page,
		dur: now() - startTime,
		dist: scrollDistance,
	});
}

doc.addEventListener('scrollend', () => {
	scrollDistance = Math.max(scrollDistance, getScrollDistance(html, innerHeight));
});

addEventListener('visibilitychange', () => {
	if (doc.visibilityState == 'hidden') {
		onPageExit();
	} else {
		onPageEnter();
	}
});

addEventListener('popstate', function () {
	onPageExit().then(onPageEnter);
});

history.pushState = function () {
	onPageExit();
	page = Page(loc, doc, loc.href);
	// @ts-ignore
	pushState.apply(this, arguments);
	onPageEnter();
};

onPageEnter().then(() => console.log('✅ %sms', now() - startTime));

// @ts-ignore
window.Abineo = {
	trackEvent: (name: string, data: object = {}) =>
		post('event', { name, data, session, visitor, page }),
};
