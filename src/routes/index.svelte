<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';

	import RepoSelectorButton from '../lib/RepoSelectorButton.svelte';

	function readLocalRepo(localRepo: string) {
		return invoke('read_repo', {
			localRepo
		});
	}

	let repo = 'Select a repository';

	async function selectRepo() {
		try {
			const folder = await open({
				title: 'Select ',
				multiple: false,
				directory: true
			});
			repo = Array.isArray(folder) ? folder[0] : folder;
			await readLocalRepo(repo);
		} catch (error) {
			console.log(error);
		}
	}
</script>

<svelte:head>
	<title>Commits history</title>
</svelte:head>

<RepoSelectorButton on:click={selectRepo} />
<p class="text-lg">{repo}</p>
