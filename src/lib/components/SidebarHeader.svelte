<script lang="ts">
	export let selected: 'commits' | 'tags';

	import { classNames } from '$lib/utils/helper';
</script>

<div class="h-full flex flex-wrap flex-row shadow-sm">
	<a class={classNames('tile', selected !== 'tags' && 'selected')} href="/commits"> Commits </a>
	<a class={classNames('tile', selected === 'tags' && 'selected')} href="/tags"> Tags </a>
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
