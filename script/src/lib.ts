export function config(document: Document, name: string) {
	return document.querySelector<HTMLMetaElement>('meta[name=abineo-' + name + ']')?.content;
}

export function getScrollDistance(element: Element, innerHeight: number) {
	return element.scrollTop / (element.scrollHeight - innerHeight);
}

export function getVisitorId(sessionStorage: SessionStorage) {
	let visitorId = sessionStorage.getItem('abineo:visitor');
	if (!visitorId) {
		let array = new BigInt64Array(1);
		crypto.getRandomValues(array);
		let n = array[0];
		visitorId = (n < 0 ? -n : n).toString(16);
		sessionStorage.setItem('abineo:visitor', visitorId);
	}
	return visitorId;
}

export function Visitor(
	sessionStorage: SessionStorage,
	navigator: Navigator,
	screen: Screen
): Visitor {
	return {
		id: getVisitorId(sessionStorage),
		timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
		language: navigator.language,
		screen: [screen.width, screen.height],
	};
}

export function Page(location: Location, document: Document, referrer?: string): Page {
	return {
		host: location.host,
		path: location.pathname,
		search: location.search,
		canonical: document.querySelector<HTMLLinkElement>('link[rel=canonical]')?.href,
		referrer,
	};
}

export function Api(fetch: FetchFn, apiUrl: string, project: string, visitor: Visitor) {
	if (!project) throw 'missing project id';

	function post(endpoint: string, data: object) {
		return fetch(apiUrl + '/api/v1/' + project + '/' + endpoint, {
			body: JSON.stringify(data),
			method: 'POST',
			keepalive: true,
		});
	}

	return {
		trackPageEnter_: (page: Page) => {
			return post('pageview', { visitor, page });
		},
		trackPageExit_: (page: Page, state: ExitState) => {
			return post('exit', { visitor, page, state });
		},
		trackEvent_: (name: string, data: object, page?: Page) => {
			return post('event', { name, data, visitor, page });
		},
	};
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

export type Document = {
	querySelector<T extends HTMLElement>(selectors: string): T | null;
};

export type FetchFn = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;

export type Element = {
	scrollTop: number;
	scrollHeight: number;
};

export type SessionStorage = {
	getItem(key: string): string | null;
	setItem(key: string, value: string): void;
};

export type Navigator = {
	language: string;
};

export type Screen = {
	width: number;
	height: number;
};

export type Location = {
	protocol: string;
	host: string;
	pathname: string;
	search: string;
};

export type Visitor = {
	id: string;
	timezone: string;
	language: string;
	screen: [number, number];
};

export type Page = {
	host: string;
	path: string;
	search?: string;
	canonical?: string;
	referrer?: string;
};

export type ExitState = {
	time: number;
	distance: number;
};
