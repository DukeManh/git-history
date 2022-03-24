<script lang="ts">
	export let selected: 'commits' | 'tags';

	import { classNames } from '$lib/utils/helper';

	const tabs = [
		{
			href: '/commits',
			name: 'commits',
			title: ' Commits '
		},
		{
			href: '/tags',
			name: 'tags',
			title: ' Tags '
		}
	];
</script>

<div class="h-full flex flex-wrap flex-row shadow-sm">
	{#each tabs as { name, href, title }, i (i)}
		<a class={classNames('tile', selected === name && 'selected')} {href}>{title}</a>
	{/each}
</div>

<style lang="scss">
	.tile {
		@apply flex-1 flex items-center justify-center;
		@apply transition-colors hover:bg-green-400;
	}

	.selected {
		@apply bg-teal-600;
	}
</style>
