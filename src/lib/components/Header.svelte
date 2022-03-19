<script lang="ts">
	import repository from '$lib/stores/repository';
</script>

<div class="bg-gray-900 h-14 text-yellow-400">
	<div class="p-2 w-full h-full flex flex-row justify-center items-center">
		<div class="min-w-fit flex flex-row justify-center items-center align-middle">
			<div>{$repository.fullName || $repository.name}</div>
			:
			<div class="cursor-pointer">{'<' + $repository.branch + '>'}</div>
		</div>
	</div>
</div>
