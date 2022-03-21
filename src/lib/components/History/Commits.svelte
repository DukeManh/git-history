<script>
	import Commit from './Commit.svelte';
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
</script>

<div class="flex-1 min-h-0 flex flex-col max-w-[25rem] select-none">
	<ul class="h-full space-y-[1px]">
		{#each commits as { sha, message }, i (i)}
			<Commit {sha} {message} />
		{/each}
	</ul>
</div>
