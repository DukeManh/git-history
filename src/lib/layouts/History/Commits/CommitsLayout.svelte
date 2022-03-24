<script lang="ts">
	import HistoryLayout from '$lib/layouts/History/HistoryLayout.svelte';
	import type { HistoryItem } from '$lib/types/ui';
	export let active = undefined;

	const commits = [
		{ sha: '4b65530', message: 'Invoke a rust command to open a folder' },
		{ sha: 'd64969f', message: '(tailwind) Configure TailwindCSS' },
		{ sha: 'a232043', message: 'first commit' },
		{ sha: 'fa0e6b1', message: '(layout) Switch Preact -> Svelte and Tailwind' },
		{ sha: 'e71bc4d', message: 'Migrate to Svelte-Kit' },
		{ sha: 'e03c5d1', message: '(origin/main, routing) Skeleton pages' },
		{ sha: '92754fa', message: '(main) Choose a folder to read' },
		{ sha: '5f5886b', message: 'User static svelte adapter, disable ssr' },
		{ sha: '4b65530', message: 'Invoke a rust command to open a folder' },
		{ sha: 'd64969f', message: '(tailwind) Configure TailwindCSS' },
		{ sha: 'a232043', message: 'first commit' },
		{ sha: 'fa0e6b1', message: '(layout) Switch Preact -> Svelte and Tailwind' },
		{ sha: 'e71bc4d', message: 'Migrate to Svelte-Kit' },
		{ sha: 'e03c5d1', message: '(origin/main, routing) Skeleton pages' },
		{ sha: '92754fa', message: '(main) Choose a folder to read' },
		{ sha: '4b65530', message: 'Invoke a rust command to open a folder' },
		{ sha: 'd64969f', message: '(tailwind) Configure TailwindCSS' },
		{ sha: 'a232043', message: 'first commit' },
		{ sha: 'fa0e6b1', message: '(layout) Switch Preact -> Svelte and Tailwind' },
		{ sha: 'e71bc4d', message: 'Migrate to Svelte-Kit' },
		{ sha: 'e03c5d1', message: '(origin/main, routing) Skeleton pages' },
		{ sha: '92754fa', message: '(main) Choose a folder to read' },
		{ sha: '4b65530', message: 'Invoke a rust command to open a folder' },
		{ sha: 'd64969f', message: '(tailwind) Configure TailwindCSS' },
		{ sha: 'a232043', message: 'first commit' },
		{ sha: 'fa0e6b1', message: '(layout) Switch Preact -> Svelte and Tailwind' },
		{ sha: 'e71bc4d', message: 'Migrate to Svelte-Kit' },
		{ sha: 'e03c5d1', message: '(origin/main, routing) Skeleton pages' },
		{ sha: '92754fa', message: '(main) Choose a folder to read' },
		{ sha: '4b65530', message: 'Invoke a rust command to open a folder' },
		{ sha: 'd64969f', message: '(tailwind) Configure TailwindCSS' },
		{ sha: 'a232043', message: 'first commit' },
		{ sha: 'fa0e6b1', message: '(layout) Switch Preact -> Svelte and Tailwind' },
		{ sha: 'e71bc4d', message: 'Migrate to Svelte-Kit' },
		{ sha: 'e03c5d1', message: '(origin/main, routing) Skeleton pages' },
		{ sha: '92754fa', message: '(main) Choose a folder to read' }
	];

	const rows: HistoryItem[] = commits.map(({ sha, ...commit }) => ({
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
