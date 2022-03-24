<script>
	import CommitsLayout from '$lib/layouts/History/Commits/CommitsLayout.svelte';
	import { page } from '$app/stores';

	$: sha = $page.params.sha;
</script>

<svelte:head>
	<title>Commit History</title>
</svelte:head>

<CommitsLayout active={sha}>
	{sha}
</CommitsLayout>
