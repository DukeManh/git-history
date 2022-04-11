<script>
	import CommitsLayout from '$lib/layouts/History/Commits/CommitsLayout.svelte';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import repository from '$lib/stores/repository';

	$: sha = $page.params.sha;

	let commit;
	$: {
		(async function () {
			commit = await repository.showCommit(sha);
		})();
	}
</script>

<svelte:head>
	<title>Commit History</title>
</svelte:head>

<CommitsLayout active={sha}>
	{JSON.stringify(commit)}
</CommitsLayout>
