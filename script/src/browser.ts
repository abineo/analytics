import { Api, Page, Visitor, config, getScrollDistance } from './lib';

let doc = document;
let html = doc.documentElement;
let addEventListener = window.addEventListener;
let loc = location;
let now = Date.now;
let pushState = history.pushState;

let protocol = config(doc, 'protocol') || 'https://';
let domain = config(doc, 'domain') || 'abineo-analytics.com';
let project = config(doc, 'project') || '';
let startTime: number;
let scrollDistance: number;
let referrer = doc.referrer;

let visitor = Visitor(sessionStorage, navigator, screen);
let page = Page(loc, doc, referrer);

let { trackPageEnter_, trackPageExit_, trackEvent_ } = Api(
	fetch,
	protocol + domain,
	project,
	visitor
);

function onPageEnter() {
	startTime = now();
	scrollDistance = getScrollDistance(html, innerHeight);
	return trackPageEnter_(page);
}

function onPageExit() {
	let exitState = {
		time: now() - startTime,
		distance: scrollDistance,
	};
	return trackPageExit_(page, exitState);
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
window.Abineo = { trackEvent: trackEvent_ };
