<script>
	export let sha, message;
</script>

<li class="commit">{sha}: {message}</li>

<style lang="scss">
	.commit {
		@apply h-8 bg-cyan-700 overflow-hidden whitespace-nowrap text-ellipsis px-2 cursor-pointer select-text;
	}
</style>
