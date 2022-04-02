<script lang="ts">
	import repository from '$lib/stores/repository';
	import HistoryLayout from '$lib/layouts/History/HistoryLayout.svelte';
	import type { HistoryItem } from '$lib/types/ui';
	export let active = undefined;

	const rows: HistoryItem[] = $repository.commits.map(({ sha, ...commit }) => ({
		key: sha,
		keyType: 'sha',
		url: `/commits/${sha}`,
		...commit
	}));
</script>

<svelte:head>
	<title>Commit History</title>
</svelte:head>

<HistoryLayout tab="commits" {active} {rows}>
	<slot />
</HistoryLayout>
