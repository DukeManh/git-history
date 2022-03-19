<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';
	import repository from '$lib/stores/repository';

	import RepoSelectorButton from '$lib/components/OpenButton.svelte';

	let repo = 'Select a repository';

	async function selectRepo() {
		try {
			const folder = await open({
				title: 'Select ',
				multiple: false,
				directory: true
			});
			repo = Array.isArray(folder) ? folder[0] : folder;
			if (repo) {
				repository.openRepo(repo);
			}
		} catch (error) {
			console.error(error);
		}
	}
</script>

<svelte:head>
	<title>Commits history</title>
</svelte:head>

<RepoSelectorButton on:click={selectRepo} />
<p class="text-lg">{repo}</p>
