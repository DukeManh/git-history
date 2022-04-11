<script lang="ts">
	import { onMount } from 'svelte';
	import repository from '$lib/stores/repository';
	import HistoryLayout from '$lib/layouts/History/HistoryLayout.svelte';
	import type { HistoryList } from '$lib/types/ui';
	export let active = undefined;

	onMount(() => {
		if ($repository.commits.length == 0) {
			repository.getCommits().catch((err) => {
				console.log(err);
			});
		}
	});

	let rows: HistoryList = [];

	$: {
		rows = $repository.commits.map(({ sha, ...commit }) => ({
			key: sha,
			keyType: 'sha',
			url: `/commits/${sha}`,
			...commit
		}));
	}
</script>

<svelte:head>
	<title>Commit History</title>
</svelte:head>

<HistoryLayout
	tab="commits"
	{active}
	{rows}
	loadMore={() => {
		if ($repository.commits.length) {
			repository.getCommits(`${$repository.commits[$repository.commits.length - 1].sha}~1`);
		}
	}}
>
	<slot />
</HistoryLayout>
