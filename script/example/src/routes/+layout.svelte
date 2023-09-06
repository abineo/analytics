<nav>
	<strong>standard navigation:</strong>
	<a data-sveltekit-reload href="/">Home</a>
	<a data-sveltekit-reload href="/features?utm_campain=self">Features</a>
	<a data-sveltekit-reload href="/team">Team</a>
	<a data-sveltekit-reload href="/contact">Contact</a>
	<span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</span>
	<strong>client side navigation:</strong>
	<a href="/">Home</a>
	<a href="/features?utm_campain=self">Features</a>
	<a href="/team">Team</a>
	<a href="/contact">Contact</a>
</nav>
<hr />

<slot />

{#each [...Array(100).keys()] as i}
	<p>{i + 1}</p>
{/each}

<style>
	:root {
		color-scheme: light dark;
	}
	p {
		display: flex;
		align-items: center;
		height: 8rem;
	}
</style>
