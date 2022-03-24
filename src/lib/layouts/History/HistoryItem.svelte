<script lang="ts">
	import type { HistoryItem } from '$lib/types/ui';

	export let row: HistoryItem;

	const { message, url, key } = row;
</script>

<a href={url} class="link">
	<slot>
		{key}: {message}
	</slot>
</a>

<style>
	.link {
		@apply block w-full h-full;
	}
</style>
