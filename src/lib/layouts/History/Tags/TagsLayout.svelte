<script lang="ts">
	import HistoryLayout from '$lib/layouts/History/HistoryLayout.svelte';
	import type { HistoryItem } from '$lib/types/ui';
	export let active = undefined;

	const tags = [
		{ tag: '0.0.4', message: 'fix: adds missing sponsors link' },
		{ tag: '0.0.7', message: "Merge branch 'master' of github.com:supabase/supabase" },
		{ tag: '0.0.8', message: 'Merge pull request #182 from supabase/blog_august' },
		{ tag: '0.0.9', message: 'Merge pull request #224 from duncanhealy/feature/todo-sql-policy' },
		{ tag: '0.20.11.30', message: 'fix: missing observer' },
		{ tag: '0.21.03', message: 'Adds the March Beta Post' },
		{ tag: '0.21.04', message: 'Merge pull request #1426 from markthom-as/patch-1' },
		{ tag: '0.21.05', message: 'Merge pull request #1896 from marcelscruz/style/general' },
		{ tag: '0.21.06', message: 'Merge pull request #2189 from supabase/chore/guides' },
		{ tag: '0.21.07', message: 'Merge pull request #2710 from supabase/blog/july' },
		{ tag: '0.21.08', message: 'Merge pull request #3181 from supabase/kiwicopple-patch-2' },
		{ tag: '0.21.09', message: 'blog: adds the sept release blog (#3416)' },
		{ tag: '0.21.10', message: 'Blog: oct release (#3793)' },
		{ tag: 'v0.0.10', message: 'blog: Adds our October blog post' },
		{ tag: 'v0.0.11', message: "Updates the blurb to be different from last month's" },
		{ tag: 'v0.0.12', message: 'spelling: Dec blog post' },
		{ tag: 'v0.21.01', message: "Merge branch 'master' of github.com:supabase/supabase" },
		{ tag: 'v0.21.02', message: 'Adds `useSupabase` library' }
	];

	const rows: HistoryItem[] = tags.map((tag) => ({
		key: tag.tag,
		keyType: 'tag',
		url: `/tags/${tag.tag}`,
		...tag
	}));
</script>

<HistoryLayout tab="tags" {active} {rows}>
	<slot />
</HistoryLayout>
