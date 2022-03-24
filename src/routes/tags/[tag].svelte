<script>
	import TagsLayout from '$lib/layouts/History/Tags/TagsLayout.svelte';
	import { page } from '$app/stores';

	$: tag = $page.params.tag;
</script>

<svelte:head>
	<title>Commit History</title>
</svelte:head>

<TagsLayout active={tag}>
	{tag}
</TagsLayout>
