<script lang="ts">
	import CommitsLayout from '$lib/layouts/History/Commits/CommitsLayout.svelte';
	import { page } from '$app/stores';
	import type { GitShow, GitShowDiff } from '$lib/types/commands.type';
	import repository from '$lib/stores/repository';
	import { splitLines } from '$lib/utils/helper';

	$: sha = $page.params.sha;

	let commit: GitShow | null;
	let diff: GitShowDiff | null;
	$: {
		(async function () {
			commit = await repository.showCommit(sha);
			diff = await repository.showDiff(sha);
		})();
	}
</script>

<svelte:head>
	<title>Commit History</title>
</svelte:head>

<CommitsLayout active={sha}>
	<div slot="header" class="p-2">
		{#if commit}
			<div>
				{commit.commit.message}
			</div>
			<div>
				{commit.commit.author} - {commit.commit.sha.slice(0, 7)}
			</div>
		{/if}
	</div>
	<div slot="content" class="h-full flex flex-col">
		{#if diff}
			<div class="flex flex-row border-b border-teal-600 space-x-[1px]">
				{#each commit.files as file (file)}
					<div class="py-2 px-4 cursor-pointer bg-slate-500">{file}</div>
				{/each}
			</div>
			<div class="flex-1 min-h-0 overflow-scroll px-4 py-2">
				{#each splitLines(diff) as line, i (i)}
					<div class="h-8">{line}</div>
				{/each}
			</div>
			<div />
		{/if}
	</div>
</CommitsLayout>
