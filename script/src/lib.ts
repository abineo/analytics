export function config(document: Document, name: string) {
	return document.querySelector<HTMLMetaElement>('meta[name="abineo:' + name + '"]')?.content;
}

export function getScrollDistance(element: Element, innerHeight: number) {
	return element.scrollTop / (element.scrollHeight - innerHeight);
}

export function Visitor(navigator: Navigator, screen: Screen): Visitor {
	return {
		timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
		language: navigator.language,
		screen: [screen.width, screen.height],
	};
}

export function Page(location: Location, document: Document, ref?: string): Page {
	return {
		host: location.host,
		path: document.querySelector<HTMLLinkElement>('link[rel=canonical]')?.href || location.pathname,
		search: location.search,
		ref,
	};
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

export type Document = {
	querySelector<T extends HTMLElement>(selectors: string): T | null;
};

export type Element = {
	scrollTop: number;
	scrollHeight: number;
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
	timezone: string;
	language: string;
	screen: [number, number];
};

export type Page = {
	host: string;
	path: string;
	search?: string;
	ref?: string;
};
