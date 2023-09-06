<script>
	import { onMount } from 'svelte';

	onMount(() => {
		window.Abineo.trackEvent('homepage', {
			time: new Intl.DateTimeFormat(navigator.languages, { timeStyle: 'short' }).format(new Date())
		});
	});
</script>

<h1>Home</h1>
