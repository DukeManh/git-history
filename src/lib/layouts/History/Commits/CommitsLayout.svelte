<script lang="ts">
	import { onMount } from 'svelte';
	import repository from '$lib/stores/repository';
	import HistoryLayout from '$lib/layouts/History/HistoryLayout.svelte';
	import type { HistoryList } from '$lib/types/ui';
	export let active = undefined;

	onMount(() => {
		repository
			.getCommits(100)
			.then((commits) => {
				console.log(commits);
			})
			.catch((err) => {
				console.log(err);
			});
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

<HistoryLayout tab="commits" {active} {rows}>
	<slot />
</HistoryLayout>
